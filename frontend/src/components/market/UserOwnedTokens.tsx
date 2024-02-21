import {
	AccountAddress,
	ConcordiumGRPCClient,
	ContractAddress,
} from "@concordium/web-sdk";
import { useEffect, useState } from "react";
import { useNodeClient } from "../NodeClientProvider";
import rwaMarket from "../../lib/rwaMarket";
import { Box, Stack } from "@mui/material";
import { default as NftTokensList } from "../nft/TokensList";
import { default as SftTokensList } from "../sft/TokensList";
import { WalletApi } from "@concordium/browser-wallet-api-helpers";
import ContractName from "./ContractName";
import { getTokenContractType } from "./types";
import { ContractType } from "../contracts/ContractTypes";
import ErrorDisplay from "../common/ErrorDisplay";
import InfoDisplay from "../common/InfoDisplay";

const ContractTokens = (props: {
	grpcClient: ConcordiumGRPCClient;
	walletApi: WalletApi;
	currentAccount: AccountAddress.Type;
	tokenContract: ContractAddress.Type;
	marketContract: ContractAddress.Type;
}) => {
	const { tokenContract: contract, grpcClient } = props;
	const [loading, setLoading] = useState(false);
	const [error, setError] = useState<string | undefined>(undefined);
	const [contractType, setContractType] = useState<ContractType | undefined>(
		undefined,
	);

	useEffect(() => {
		setLoading(true);
		getTokenContractType(grpcClient, contract)
			.then((type) => {
				setContractType(type);
			})
			.catch((error) => {
				setError(error.message);
			})
			.finally(() => {
				setLoading(false);
			});
	}, [contract, grpcClient]);

	return (
		<>
			<Box p={2}>
				<ContractName contract={contract} variant="h6" />
			</Box>
			{contractType && contractType === ContractType.RwaSecurityNft && (
				<NftTokensList
					contract={contract}
					currentAccount={props.currentAccount}
					walletApi={props.walletApi}
					grpcClient={props.grpcClient}
					marketContract={props.marketContract}
				/>
			)}
			{contractType && contractType === ContractType.RwaSecuritySft && (
				<SftTokensList
					contract={contract}
					currentAccount={props.currentAccount}
					walletApi={props.walletApi}
					grpcClient={props.grpcClient}
					marketContract={props.marketContract}
				/>
			)}
			{error && <ErrorDisplay text={error} />}
			{loading && <InfoDisplay text="Loading..." />}
		</>
	);
};

export default function UserOwnedTokens(props: {
	currentAccount: AccountAddress.Type;
	contract: ContractAddress.Type;
	wallet: WalletApi;
}) {
	const { contract: contract } = props;
	const { provider: grpcClient } = useNodeClient();
	const [sellContracts, setSellContracts] = useState<ContractAddress.Type[]>(
		[],
	);

	useEffect(() => {
		rwaMarket.allowedToList.invoke(grpcClient, contract).then((result) => {
			const ret = rwaMarket.allowedToList.parseReturnValue(result.returnValue!);
			setSellContracts(
				ret!.map((c) => ContractAddress.create(c.index, c.subindex)),
			);
		});
	}, [contract, grpcClient]);

	return (
		<Stack spacing={2}>
			{sellContracts.map((tokenContract, index) => (
				<ContractTokens
					key={index}
					tokenContract={tokenContract}
					grpcClient={grpcClient}
					walletApi={props.wallet}
					currentAccount={props.currentAccount}
					marketContract={contract}
				/>
			))}
		</Stack>
	);
}
