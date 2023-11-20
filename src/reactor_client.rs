use std::{pin::Pin, sync::Arc};

use anyhow::Result;
use ethers::{
    abi::FixedBytes,
    prelude::abigen,
    providers::{JsonRpcClient, Middleware, Provider, PubsubClient, StreamExt},
    types::{Address, BlockNumber, Filter, Log},
};
use futures::Stream;

use crate::{reactor_config::ReactorConfig, types::FillEvent};

abigen!(
    ExclusiveDutchOrderReactorContract,
    "abi/exclusive_dutch_order_reactor.json"
);

pub struct ReactorClient<M> {
    reactor_contract: ExclusiveDutchOrderReactorContract<Provider<M>>,
    reactor_contract_address: Address,
    provider: Arc<Provider<M>>,
}

impl<M: JsonRpcClient> ReactorClient<M> {
    pub fn new(provider: Arc<Provider<M>>, chain_id: u64) -> Self {
        let config = ReactorConfig::new(chain_id);

        Self {
            reactor_contract: ExclusiveDutchOrderReactorContract::new(
                config.address,
                provider.clone(),
            ),
            reactor_contract_address: config.address,
            provider,
        }
    }

    pub async fn get_fill_events<B: Into<BlockNumber>>(
        &self,
        from_block: B,
        to_block: Option<B>,
    ) -> Result<Vec<FillEvent>> {
        let filter = Filter::new()
            .from_block(from_block)
            .to_block(to_block.map(|b| b.into()).unwrap_or_default())
            .address(self.reactor_contract_address)
            .event("Fill(bytes32,address,address,uint256)");

        let fill_event_logs = self.provider.get_logs(&filter).await?;
        let mut events = vec![];

        for log in fill_event_logs
            .into_iter()
            .filter(|l| !l.removed.unwrap_or_default())
        {
            let fill = self.decode_fill_event(log)?;

            events.push(fill);
        }

        Ok(events)
    }

    pub fn decode_fill_event(&self, log: Log) -> Result<FillEvent> {
        let (order_hash, filler, swapper, _) =
            self.reactor_contract
                .decode_event::<(FixedBytes, Address, Address, u64)>(
                    "Fill",
                    log.topics.clone(),
                    log.data.clone(),
                )?;

        let fill = FillEvent::new(
            order_hash.into(),
            filler,
            swapper,
            log.transaction_hash.unwrap(),
            log.block_number.unwrap(),
        );

        Ok(fill)
    }
}

pub type FillEventStream<'a> = Pin<Box<dyn Stream<Item = FillEvent> + Send + 'a>>;

impl<M: JsonRpcClient + PubsubClient + Send + Sync + 'static> ReactorClient<M> {
    pub async fn get_fill_events_stream(&self) -> Result<FillEventStream<'_>> {
        let filter = Filter::new()
            .address(self.reactor_contract_address)
            .event("Fill(bytes32,address,address,uint256)");

        let stream = self
            .provider
            .subscribe_logs(&filter)
            .await?
            .filter_map(|log: Log| async { self.decode_fill_event(log).ok() });

        Ok(Box::pin(stream))
    }
}
