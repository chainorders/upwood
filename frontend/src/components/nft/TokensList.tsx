import { WalletApi } from "@concordium/browser-wallet-api-helpers";
import {
	AccountAddress,
	ContractAddress,
	ConcordiumGRPCClient,
	CIS2Contract,
	EntrypointName,
	Energy,
	CIS2,
	serializeTypeValue,
	toBuffer,
} from "@concordium/web-sdk";
import { useEffect, useState } from "react";
import { useContractsApi } from "../ContractsApiProvider";
import { NftHolder } from "../../lib/contracts-api-client";
import { useNavigate, useSearchParams } from "react-router-dom";
import { ActionButtonProps, Token } from "../common/TokenCardDisplay";
import TokensGrid from "../common/TokensGrid";
import { LensBlur, Sell, SubscriptionsRounded } from "@mui/icons-material";
import { Buffer } from "buffer/";

import rwaSecuritySft, { MintRequest } from "../../lib/rwaSecuritySft";
import { SFT_CONTRACT_INDEX, SFT_CONTRACT_SUBINDEX } from "./const";
import { useSponsorApi } from "../SponsorApiProvider";
import { permit } from "../../lib/sponsorUtils";

type Props = {
	currentAccount: AccountAddress.Type;
	walletApi: WalletApi;
	contract: ContractAddress.Type;
	grpcClient: ConcordiumGRPCClient;
	marketContract?: ContractAddress.Type;
	sponsorContract?: ContractAddress.Type;
};
export default function TokensList(props: Props) {
	const {
		currentAccount,
		walletApi,
		contract,
		grpcClient,
		marketContract,
		sponsorContract,
	} = props;

	const [pageCount, setPageCount] = useState(0);
	const [searchParams, setSearchParams] = useSearchParams();
	const [page, setPage] = useState(Number(searchParams.get("page") || "0"));
	const [loading, setLoading] = useState(false);
	const [error, setError] = useState("");
	const { provider: backendApi } = useContractsApi();
	const navigate = useNavigate();
	const { provider: sponsorApi } = useSponsorApi();

	let sftContract: ContractAddress.Type | undefined;
	if (SFT_CONTRACT_INDEX && SFT_CONTRACT_SUBINDEX) {
		sftContract = ContractAddress.create(
			BigInt(SFT_CONTRACT_INDEX),
			BigInt(SFT_CONTRACT_SUBINDEX),
		);
	}

	const [tokens, setTokens] = useState<NftHolder[]>([]);
	useEffect(() => {
		setLoading(true);
		setError("");
		backendApi.default
			.getRwaSecurityNftHolders({
				address: currentAccount.address,
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

	const onFractionalize = async (
		token: Token,
		sftContract: ContractAddress.Type,
		sponsorContract?: ContractAddress.Type,
	): Promise<string> => {
		const fracRequest: MintRequest = {
			deposited_token_id: {
				id: token.id,
				contract: {
					index: Number(contract.index),
					subindex: Number(contract.subindex),
				},
			},
			deposited_amount: token.amount,
			deposited_token_owner: currentAccount.address,
			owner: { Account: [currentAccount.address] },
		};
		const listRequestSerialized = serializeTypeValue(
			fracRequest,
			toBuffer(rwaSecuritySft.mint.paramsSchemaBase64!, "base64"),
		);
		const cis2CLient = await CIS2Contract.create(grpcClient, token.contract);
		const transfer = cis2CLient.createTransfer(
			{
				energy: Energy.create(
					rwaSecuritySft.mint.maxExecutionEnergy.value * BigInt(2),
				),
			},
			{
				from: currentAccount!,
				to: {
					address: sftContract,
					hookName: EntrypointName.fromString("deposit"),
				},
				amount: BigInt(0),
				tokenId: token.id,
				tokenAmount: BigInt(token.amount),
				data: Buffer.from(listRequestSerialized.buffer).toString("hex"),
			} as CIS2.Transfer,
		);

		if (!sponsorContract) {
			return walletApi!.sendTransaction(
				currentAccount!,
				transfer.type,
				transfer.payload,
				transfer.parameter.json,
				transfer.schema,
			);
		} else {
			return permit(
				grpcClient,
				walletApi,
				sponsorApi,
				sponsorContract,
				currentAccount,
				token.contract,
				transfer.parameter.hex,
			);
		}
	};

	const uiTokens = tokens.map(
		(token) =>
			({
				id: token.token_id,
				contract,
				amount: token.balance,
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
	if (sftContract) {
		actions.push({
			ariaLabel: "Fractionalize",
			children: <LensBlur />,
			variant: "outlined",
			title: "Fractionalize",
			disabled: false,
			sendTransaction: (token) => onFractionalize(token, sftContract!),
			onClick: (token: Token) => console.log("Fractionalize", token),
		});

		if (sponsorContract) {
			actions.push({
				ariaLabel: "Sponsored Fractionalize",
				children: <SubscriptionsRounded />,
				variant: "outlined",
				title: "Sponsored Fractionalize",
				disabled: false,
				sendTransaction: (token) =>
					onFractionalize(token, sftContract!, sponsorContract),
				onClick: (token: Token) => console.log("Sponsor Fractionalize", token),
			});
		}
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
