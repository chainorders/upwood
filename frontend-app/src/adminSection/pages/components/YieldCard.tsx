import { SystemContractsConfigApiModel, Yield } from "../../../apiClient";
import { Paper, Typography } from "@mui/material";

export default function YieldCard({ yield_, contracts }: { yield_: Yield; contracts: SystemContractsConfigApiModel }) {
	return (
		<Paper key={yield_.yield_token_id + yield_.yield_contract_address} elevation={2} sx={{ padding: 2, marginBottom: 2 }}>
			<Typography variant="subtitle1" fontWeight="bold">
				{
					{
						[`${contracts.euro_e_token_id}-${contracts.euro_e_contract_index}`]: "Euro E",
						[`0-${contracts.tree_ft_contract_index}`]: "Tree",
						[`${contracts.carbon_credit_token_id}-${contracts.carbon_credit_contract_index}`]: "Carbon Credit",
					}[`${yield_.yield_token_id}-${yield_.yield_contract_address}`]
				}
			</Typography>
			<Typography variant="subtitle2" fontWeight="bold">
				Yielded Token: {yield_.yield_token_id}-{yield_.yield_contract_address}
			</Typography>
			<Typography>
				Rate: {yield_.yield_rate_numerator} / {yield_.yield_rate_denominator}
			</Typography>
			<Typography>Yield Type: {yield_.yield_type}</Typography>
		</Paper>
	);
}
