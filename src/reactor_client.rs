use std::sync::Arc;

use alloy_json_rpc::{Id, Request, RequestMeta, ResponsePayload};
use alloy_primitives::{Address, U256};
use alloy_providers::provider::{Provider, TempProvider};
use alloy_pubsub::PubSubFrontend;
use alloy_rpc_types::{
    pubsub::{Params, SubscriptionKind},
    BlockNumberOrTag, Filter, Log,
};
use alloy_sol_types::{sol, SolEvent};
use alloy_transport::BoxTransport;
use anyhow::{anyhow, bail, Context, Result};
use futures::{stream::BoxStream, StreamExt, TryStreamExt};
use tokio_stream::wrappers::BroadcastStream;

use crate::{reactor_config::ReactorConfig, types::FillEvent};

sol!(
    ExclusiveDutchOrderReactorContract,
    "abi/exclusive_dutch_order_reactor.json"
);

pub struct ReactorClient {
    reactor_contract_address: Address,
}

impl ReactorClient {
    pub fn new(chain_id: u64) -> Self {
        let config = ReactorConfig::new(chain_id);

        Self {
            reactor_contract_address: config.address,
        }
    }

    pub async fn get_fill_events<B: Into<BlockNumberOrTag>>(
        &self,
        provider: Arc<Provider<BoxTransport>>,
        from_block: B,
        to_block: Option<B>,
    ) -> Result<Vec<FillEvent>> {
        let filter = Filter::new()
            .from_block(from_block)
            .to_block(to_block.map(|b| b.into()).unwrap_or_default())
            .address(self.reactor_contract_address)
            .event(ExclusiveDutchOrderReactorContract::Fill::SIGNATURE);

        let fill_event_logs = provider.get_logs(filter).await?;
        let mut events = vec![];

        for log in fill_event_logs.into_iter().filter(|l| !l.removed) {
            let fill = decode_fill_event(log)?;

            events.push(fill);
        }

        Ok(events)
    }

    pub async fn get_fill_events_stream(
        &self,
        front_end: &PubSubFrontend,
        id: Id,
    ) -> Result<BoxStream<Result<FillEvent>>> {
        let req = Request {
            meta: RequestMeta {
                method: "eth_subscribe",
                id,
            },
            params: [
                serde_json::to_value(SubscriptionKind::Logs)?,
                serde_json::to_value(Params::Logs(Box::new(
                    Filter::new()
                        .address(self.reactor_contract_address)
                        .event_signature(ExclusiveDutchOrderReactorContract::Fill::SIGNATURE_HASH),
                )))?,
            ],
        };

        let response = front_end
            .send(req.serialize()?)
            .await?
            .deser_success::<U256>()
            .map_err(|_| anyhow!("The payload can't be deserialized"))?;

        let subscription_id = match response.payload {
            ResponsePayload::Success(subscription_id) => subscription_id,
            ResponsePayload::Failure(err) => bail!(err),
        };

        let rx = front_end.get_subscription(subscription_id).await?;

        let stream = BroadcastStream::new(rx)
            .map_err(|err| anyhow!(err))
            .map(|r| {
                r.and_then(|value| {
                    serde_json::from_str::<Log>(value.get())
                        .map_err(|err| anyhow!(err))
                        .context("Failed to deserialize log")
                })
                .and_then(|log| decode_fill_event(log).context("Failed to decode fille event"))
            });

        Ok(stream.boxed())
    }
}

pub fn decode_fill_event(log: Log) -> Result<FillEvent> {
    let ev =
        ExclusiveDutchOrderReactorContract::Fill::decode_log_data(&log.clone().try_into()?, true)?;

    let fill = FillEvent::new(
        ev.orderHash,
        ev.filler,
        ev.swapper,
        log.transaction_hash.unwrap(),
        log.block_number.unwrap(),
    );

    Ok(fill)
}
