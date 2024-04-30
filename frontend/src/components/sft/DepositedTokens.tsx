import { WalletApi } from "@concordium/browser-wallet-api-helpers";
import {
	AccountAddress,
	ContractAddress,
	ConcordiumGRPCClient,
} from "@concordium/web-sdk";
import { useEffect, useState } from "react";
import { useContractsApi } from "../ContractsApiProvider";
import { ApiDepositedToken as DepositedToken } from "../../lib/contracts-api-client";
import { useNavigate, useSearchParams } from "react-router-dom";
import { ActionButtonProps, Token } from "../common/TokenCardDisplay";
import TokensGrid from "../common/TokensGrid";
import { Sell } from "@mui/icons-material";

type Props = {
	currentAccount: AccountAddress.Type;
	walletApi: WalletApi;
	contract: ContractAddress.Type;
	grpcClient: ConcordiumGRPCClient;
	marketContract?: ContractAddress.Type;
	sftContract?: ContractAddress.Type;
};
export default function TokensList(props: Props) {
	const { currentAccount, walletApi, contract, grpcClient, marketContract } =
		props;
	const [pageCount, setPageCount] = useState(0);
	const [searchParams, setSearchParams] = useSearchParams();
	const [page, setPage] = useState(Number(searchParams.get("page") || "0"));
	const [loading, setLoading] = useState(false);
	const [error, setError] = useState("");
	const { provider: backendApi } = useContractsApi();
	const navigate = useNavigate();

	const [tokens, setTokens] = useState<DepositedToken[]>([]);
	useEffect(() => {
		setLoading(true);
		setError("");
		backendApi.default
			.getRwaSecuritySftDeposited({
				owner: currentAccount.address,
				index: Number(contract.index),
				subindex: Number(contract.subindex),
				page,
			})
			.then((response) => {
				setTokens(response.data);
				setPageCount(response.page_count);
				setPage(response.page);
			})
			.catch((error) => setError(error.message))
			.finally(() => setLoading(false));
	}, [currentAccount, walletApi, contract, page, backendApi]);

	const uiTokens = tokens.map(
		(token) =>
			({
				id: token.token_id,
				contract,
				amount: token.deposited_amount,
			}) as Token,
	);

	const actions: Omit<ActionButtonProps, "token">[] = [];
	if (marketContract) {
		actions.push({
			ariaLabel: "Sell",
			children: <Sell />,
			variant: "outlined",
			title: "Sell",
			disabled: false,
			onClick: (token: Token) => {
				navigate(
					`/market/${marketContract.index.toString()}/${marketContract.subindex.toString()}/transferList/${contract.index.toString()}/${contract.subindex.toString()}/${
						token.id
					}/${token.amount.toString()}`,
				);
			},
		});
	}

	return TokensGrid({
		actions,
		grpcClient,
		error,
		loading,
		page,
		pageCount,
		tokens: uiTokens,
		onPageChange: (page) =>
			setSearchParams({ ...searchParams, page: page.toString() }),
	});
}
