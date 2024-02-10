import { useLocation, Location, useNavigate } from "react-router-dom";
import { MarketToken } from "../../lib/contracts-api-client";
import { Button, Stack, Typography } from "@mui/material";
import { ContractAddress } from "@concordium/web-sdk";
import SendTransactionButton from "../common/SendTransactionButton";
import rwaMarket, { WithdrawRequest } from "../../lib/rwaMarket";
import { useWallet } from "../WalletProvider";

type Props = {
	contract: ContractAddress.Type;
};
export default function WithdrawToken(props: Props) {
	const navigate = useNavigate();
	const { state: token }: Location<MarketToken | undefined> = useLocation();
	const { contract } = props;
	const { provider: wallet, currentAccount } = useWallet();

	if (!token) {
		navigate(-1);
		return <></>;
	}

	const withdrawRequest = (token: MarketToken) => {
		const request: WithdrawRequest = {
			amount: token.unlisted_amount,
			owner: token.owner,
			token_id: {
				id: token.token_id,
				contract: {
					index: token.token_contract.index,
					subindex: token.token_contract.subindex,
				},
			},
		};

		return rwaMarket.withdraw.update(
			wallet!,
			currentAccount!,
			props.contract,
			request,
		);
	};

	return (
		<Stack spacing={2} maxWidth="800px">
			<Typography variant="h4">Withdraw</Typography>
			<Typography paragraph>
				Are you sure you want to withdraw Token: {token.token_id} from the
				market contract ({contract.index.toString()}/
				{contract.subindex.toString()}) ?
			</Typography>
			<Typography paragraph>
				The token will be sent to the owner: {token.owner}
			</Typography>
			<SendTransactionButton
				onClick={() => withdrawRequest(token)}
				onDone={() => navigate(-1)}
			>
				Withdraw
			</SendTransactionButton>
			<Button
				variant="contained"
				color="secondary"
				onClick={() => navigate(-1)}
			>
				Cancel
			</Button>
		</Stack>
	);
}
