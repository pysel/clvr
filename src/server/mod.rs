use std::{net::TcpListener, sync::{Arc, Mutex}};

use alloy::{primitives::Address, providers::{ProviderBuilder, RootProvider}, transports::http::{Client, Http}};
use swap_router_v3::SwapRouterV3;
use crate::clvr::model::{clvr_model::CLVRModel, Omega};
use actix_web::{App};
use crate::trades::ITrade;

mod swap_router_v3;
pub mod handlers;
mod handlers_types;
mod eip2612;

#[cfg(test)]
mod eip2612_tests;

type DefaultTransport = Http<Client>;

pub struct Processor {
    omega: Omega,
    model: CLVRModel,

    // provider: RootProvider<DefaultTransport>,
    // swap_router_v3: SwapRouterV3::SwapRouterV3Instance<DefaultTransport, RootProvider<DefaultTransport>>,
}

impl Processor {
    pub fn new() -> Self {
        // create variables related to the algorithm
        let omega = Omega::new();
        let model = CLVRModel::new();

        // create a provider to post requests to the ethereum network
        let provider = Self::create_provider();

        // create an instance of the swap router contract
        let swap_router_address_str = std::env::var("SWAP_ROUTER_ADDRESS").expect("SWAP_ROUTER_ADDRESS must be set");
        let swap_router_address: Address = swap_router_address_str.parse().expect("SWAP_ROUTER_ADDRESS must be a valid address");
        let swap_router_contract = SwapRouterV3::new(swap_router_address, provider.clone());

        // create a listener
        let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to port 8080");

        Self {
            omega,
            model,
            // provider: provider,
            // swap_router_v3: swap_router_contract,
        }
    }

    fn add_trade(&mut self, trade: Box<dyn ITrade>) {
        self.omega.push(trade);
    }

    fn create_provider() -> RootProvider<DefaultTransport> {
        let rpc_url = std::env::var("ETHEREUM_RPC_URL").expect("ETHEREUM_RPC_URL must be set");
        let rpc_url = rpc_url.parse().expect("ETHEREUM_RPC_URL must be a valid URL");
        let provider = ProviderBuilder::new().on_http(rpc_url);

        provider
    }

    // pub async fn run_http_server(&self) -> std::io::Result<()> {
    //     let scheduled_db: ScheduledDatabase = Arc::new(Mutex::new(Vec::new()));
        
    //     HttpProcessor::new(|| {
    //         App::new()
    //             .service(num_trades)
    //     })
    //     .bind(("127.0.0.1", 8080))?
    //     .workers(2)
    //     .run()
    //     .await
    // }

}
