use alloy::{contract::Error, network::Ethereum, primitives::{Address, U256}, providers::{Provider, ProviderBuilder, RootProvider}, transports::{http::{Client, Http}, BoxTransport}};
use alloy::transports::Transport;
use swap_router_v3::{SwapRouterV3, ISwapRouter};
use crate::clvr::model::{clvr_model::CLVRModel, Omega};

use crate::trades::ITrade;

mod swap_router_v3;
mod handlers;
mod eip2612;

#[cfg(test)]
mod eip2612_tests;

type DefaultTransport = Http<Client>;

struct Server {
    omega: Omega,
    model: CLVRModel,

    // provider: RootProvider<DefaultTransport>,
    // swap_router_v3: SwapRouterV3::SwapRouterV3Instance<DefaultTransport, RootProvider<DefaultTransport>>,
}

impl Server {
    fn new() -> Self {
        let provider = Self::create_provider();

        let swap_router_address_str = std::env::var("SWAP_ROUTER_ADDRESS").expect("SWAP_ROUTER_ADDRESS must be set");
        let swap_router_address: Address = swap_router_address_str.parse().expect("SWAP_ROUTER_ADDRESS must be a valid address");
        let swap_router_contract = SwapRouterV3::new(swap_router_address, provider.clone());

        Self {
            omega: Omega::new(),
            model: CLVRModel::new(),
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
}
