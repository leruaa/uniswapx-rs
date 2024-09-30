use std::sync::Arc;

use alloy::primitives::Address;
use alloy::{
    network::Network,
    providers::{Provider, RootProvider},
    pubsub::PubSubFrontend,
    rpc::{
        json_rpc::{Id, Request, RequestMeta, ResponsePayload},
        types::eth::{
            pubsub::{Params, SubscriptionKind},
            BlockNumberOrTag, Filter, Log,
        },
    },
    sol,
    sol_types::SolEvent,
    transports::Transport,
};
use anyhow::{anyhow, bail, Result};
use futures::{
    stream::{self, BoxStream},
    StreamExt,
};
use tracing::error;

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

    pub async fn get_fill_events<B, T, N>(
        &self,
        provider: Arc<RootProvider<T, N>>,
        from_block: B,
        to_block: Option<B>,
    ) -> Result<Vec<FillEvent>>
    where
        B: Into<BlockNumberOrTag>,
        T: Transport + Clone,
        N: Network,
    {
        let filter = Filter::new()
            .from_block(from_block)
            .to_block(to_block.map(|b| b.into()).unwrap_or_default())
            .address(self.reactor_contract_address)
            .event(ExclusiveDutchOrderReactorContract::Fill::SIGNATURE);

        let fill_event_logs = provider.get_logs(&filter).await?;
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
        let stringified_id = id.to_string();

        let req = Request {
            meta: RequestMeta::new("eth_subscribe".into(), id),
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
            .deser_success()
            .map_err(|_| anyhow!("The payload can't be deserialized"))?;

        let subscription_id = match response.payload {
            ResponsePayload::Success(subscription_id) => subscription_id,
            ResponsePayload::Failure(err) => bail!(err),
        };

        let rx = front_end.get_subscription(subscription_id).await?;

        let stream = stream::unfold(
            (rx, stringified_id),
            |(mut rx, stringified_id)| async move {
                match rx.recv().await {
                    Ok(value) => Some((value, (rx, stringified_id))),
                    Err(err) => {
                        error!("Subscription {stringified_id} ended: {err}");
                        None
                    }
                }
            },
        );

        let stream = stream.map(|value| {
            {
                serde_json::from_str::<Log>(value.get())
                    .map_err(|err| anyhow!("Failed to deserialize log: {err}"))
            }
            .and_then(|log| {
                decode_fill_event(log).map_err(|err| anyhow!("Failed to decode fill event: {err}"))
            })
        });

        Ok(stream.boxed())
    }
}

pub fn decode_fill_event(log: Log) -> Result<FillEvent> {
    let ev = ExclusiveDutchOrderReactorContract::Fill::decode_log_data(log.data(), true)?;

    let fill = FillEvent::new(
        ev.orderHash,
        ev.filler,
        ev.swapper,
        log.transaction_hash.unwrap(),
        log.block_number.unwrap(),
    );

    Ok(fill)
}
