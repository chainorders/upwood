import { useLocation, Location, useNavigate } from "react-router-dom";
import { MarketToken } from "../../lib/contracts-api-client";
import { Stack, Typography } from "@mui/material";
import { ContractAddress } from "@concordium/web-sdk";
import rwaMarket, { GetListedResponse, ListRequest } from "../../lib/rwaMarket";
import { useWallet } from "../WalletProvider";
import { useEffect, useState } from "react";
import { useNodeClient } from "../NodeClientProvider";
import ErrorDisplay from "../common/ErrorDisplay";
import ListRequestForm from "./ListRequest";

type Props = {
	contract: ContractAddress.Type;
};
export default function ListToken(props: Props) {
	const navigate = useNavigate();
	const { contract } = props;
	const { state: token }: Location<MarketToken | undefined> = useLocation();
	const { provider: wallet, currentAccount } = useWallet();
	const { provider: grpcClient } = useNodeClient();
	const [error, setError] = useState<string | undefined>(undefined);
	const [loading, setLoading] = useState(false);
	const [listedToken, setListedToken] = useState<GetListedResponse | undefined>(
		undefined,
	);

	if (!token) {
		navigate(-1);
	}

	useEffect(() => {
		setLoading(true);
		rwaMarket.getListed
			.invoke(grpcClient, contract, {
				owner: token!.owner,
				token_id: {
					id: token!.token_id,
					contract: {
						index: token!.token_contract.index,
						subindex: token!.token_contract.subindex,
					},
				},
			})
			.then((result) =>
				rwaMarket.getListed.parseReturnValue(result.returnValue!),
			)
			.then((listedToken) => {
				if (!listedToken) {
					throw new Error("Token is not listed, got undefined from getListed");
				}

				return listedToken;
			})
			.then((listedToken) => setListedToken(listedToken))
			.catch((error) => {
				setError(error.message);
			})
			.finally(() => {
				setLoading(false);
			});
	}, [contract, token, grpcClient]);

	const sendTransaction = (request: ListRequest) => {
		return rwaMarket.list.update(
			wallet!,
			currentAccount!,
			props.contract,
			request,
		);
	};

	return (
		<Stack spacing={2}>
			{error && <ErrorDisplay text={error} />}
			{loading && <Typography variant="body1">Loading...</Typography>}
			{listedToken && (
				<ListRequestForm
					contract={contract}
					currentAccount={currentAccount!}
					listed={listedToken}
					onSendTransaction={sendTransaction}
				/>
			)}
		</Stack>
	);
}
