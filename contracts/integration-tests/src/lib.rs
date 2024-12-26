pub mod cis2;
pub mod cis2_conversions;
pub mod cis2_security;
pub mod compliance;
pub mod contract_base;
pub mod euroe;
pub mod identity_registry;
pub mod nft_multi_rewarded_client;
pub mod offchain_rewards_client;
pub mod security_mint_fund_client;
pub mod security_p2p_trading_client;
pub mod security_sft_multi_client;
pub mod security_sft_single_client;

use concordium_smart_contract_testing::Energy;
const MAX_ENERGY: Energy = Energy { energy: 30000 };
