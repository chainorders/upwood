import { ForestProjectAggApiModel, ForestProjectTokenContract, Market, MarketType } from "../apiClient";
import { User } from "../lib/user";
import MarketBuy from "./MarketBuy";
import MarketBuyMint from "./MarketBuyMint";

interface Props {
	project: ForestProjectAggApiModel;
	user: User;
	token_contract: ForestProjectTokenContract;
	market: Market;
	close: () => void;
}

export default function MarketBuyCombined({ user, token_contract, market, project, close }: Props) {
	if (market.market_type === MarketType.MINT) {
		return (
			<MarketBuyMint
				close={close}
				user={user}
				tokenContract={token_contract}
				project={project.forest_project}
				legalContractSigned={project.contract_signed}
				market={{
					sell_rate_numerator: market.sell_rate_numerator!,
					sell_rate_denominator: market.sell_rate_denominator!,
					contract_address: market.contract_address,
					currency_token_contract_address: market.currency_token_contract_address!,
					liquidity_provider: market.liquidity_provider!,
					token_contract_address: market.token_contract_address!,
					max_token_amount: market.max_token_amount!,
				}}
				supply={project.supply}
			/>
		);
	} else {
		return (
			<MarketBuy
				close={close}
				user={user}
				market={{
					sell_rate_denominator: market.sell_rate_denominator!,
					sell_rate_numerator: market.sell_rate_numerator!,
					currency_token_contract_address: market.currency_token_contract_address!,
					liquidity_provider: market.liquidity_provider!,
					token_contract_address: market.token_contract_address!,
					token_id: market.token_id!,
					contract_address: market.contract_address,
					max_token_amount: market.max_token_amount!,
					max_currency_amount: market.max_currency_amount!,
				}}
				tokenContract={token_contract}
				project={project.forest_project}
				supply={project.supply}
				legalContractSigned={project.contract_signed}
				userNotified={project.user_notified}
			/>
		);
	}
}
