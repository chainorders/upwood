import { WalletApi } from "@concordium/browser-wallet-api-helpers";
import {
	AccountAddress,
	ConcordiumGRPCClient,
	ContractAddress,
} from "@concordium/web-sdk";
import { Contract, ContractType } from "../contracts/ContractTypes";
import { useEffect, useState } from "react";
import ContractsList from "../contracts/ContractsList";
import rwaMarket, { AllowedToListResponse } from "../../lib/rwaMarket";
import { useNodeClient } from "../NodeClientProvider";
import { getContractName, getTokenContractType } from "./types";
import rwaSecurityNft from "../../lib/rwaSecurityNft";
import rwaSecuritySft from "../../lib/rwaSecuritySft";
import rwaCompliance from "../../lib/rwaCompliance";
import { Paper } from "@mui/material";
import { Route, Routes } from "react-router-dom";
import RwaIdentityRegistryContract from "../contracts/rwaIdentityRegistry/RwaIdentityRegistryContract";
import ConcordiumContract from "../contracts/ConcordiumContract";
import ErrorDisplay from "../common/ErrorDisplay";
import {
	ENTRYPOINT_DISPLAY_NAMES as rwaComplianceEntrypointNames,
	ENTRYPOINTS as rwaComplianceEntrypoints,
} from "../../lib/rwaCompliance";
import { ENTRYPOINTS_UI as rwaComplianceEntrypointsUI } from "../../lib/rwaComplianceUi";
import {
	ENTRYPOINT_DISPLAY_NAMES as rwaComplianceModuleEntrypointNames,
	ENTRYPOINTS as rwaComplianceModuleEntrypoints,
} from "../../lib/rwaComplianceModuleAllowedNationalities";
import { ENTRYPOINTS_UI as rwaComplianceModuleEntrypointsUI } from "../../lib/rwaComplianceModuleAllowedNationalitiesUi";
import {
	ENTRYPOINT_DISPLAY_NAMES as rwaMarketEntrypointNames,
	ENTRYPOINTS as rwaMarketEntrypoints,
} from "../../lib/rwaMarket";
import { ENTRYPOINTS_UI as rwaMarketEntrypointsUI } from "../../lib/rwaMarketUi";
import RwaSecurityNftContract from "../contracts/rwaSecurityNft/RwaSecurityNftContract";
import RwaSecuritySftContract from "../contracts/rwaSecuritySft/RwaSecuritySftContract";
const getTokenContracts = async (
	grpcClient: ConcordiumGRPCClient,
	marketContract: ContractAddress.Type,
): Promise<Contract[]> => {
	const addresses = await rwaMarket.allowedToList
		.invoke(grpcClient, marketContract)
		.then(
			(result) =>
				rwaMarket.allowedToList.parseReturnValue(
					result.returnValue!,
				) as AllowedToListResponse,
		)
		.then((c) => c.map((c) => ContractAddress.create(c.index, c.subindex)));

	const promises = await addresses.map((address) =>
		getTokenContractType(grpcClient, address).then(
			(type) => ({ type, address, name: `Token ${type}` }) as Contract,
		),
	);

	return Promise.all(promises);
};

const getIdentityRegistries = (
	grpcClient: ConcordiumGRPCClient,
	tokenContracts: Contract[],
): Promise<Contract[]> => {
	const promises = tokenContracts.map((c) => {
		switch (c.type) {
			case ContractType.RwaSecurityNft:
				return rwaSecurityNft.identityRegistry
					.invoke(grpcClient, c.address)
					.then((res) =>
						rwaSecurityNft.identityRegistry.parseReturnValue(res.returnValue!),
					)
					.then((contract) =>
						ContractAddress.create(contract!.index, contract!.subindex),
					);
			case ContractType.RwaSecuritySft:
				return rwaSecuritySft.identityRegistry
					.invoke(grpcClient, c.address)
					.then((res) =>
						rwaSecurityNft.identityRegistry.parseReturnValue(res.returnValue!),
					)
					.then((contract) =>
						ContractAddress.create(contract!.index, contract!.subindex),
					);
			default:
				Promise.resolve(undefined);
		}
	});

	return Promise.all(promises)
		.then((contracts) =>
			contracts
				.filter((c) => c !== undefined)
				.map((c) => c!)
				.map(
					(ir) =>
						({
							address: ir,
							name: `${getContractName(ir)} (Identity Registry)`,
							type: ContractType.RwaIdentityRegistry,
						}) as Contract,
				),
		)
		.then((contracts) =>
			contracts.filter(
				(c, i, self) =>
					self.findIndex((cc) => cc.address.index === c.address.index) === i,
			),
		);
};

const getComplianceContracts = (
	grpcClient: ConcordiumGRPCClient,
	tokenContracts: Contract[],
) => {
	const promises = tokenContracts.map((c) => {
		switch (c.type) {
			case ContractType.RwaSecurityNft:
				return rwaSecurityNft.compliance
					.invoke(grpcClient, c.address)
					.then((res) =>
						rwaSecurityNft.compliance.parseReturnValue(res.returnValue!),
					)
					.then((contract) =>
						ContractAddress.create(contract!.index, contract!.subindex),
					);
			case ContractType.RwaSecuritySft:
				return rwaSecuritySft.compliance
					.invoke(grpcClient, c.address)
					.then((res) =>
						rwaSecurityNft.compliance.parseReturnValue(res.returnValue!),
					)
					.then((contract) =>
						ContractAddress.create(contract!.index, contract!.subindex),
					);
			default:
				Promise.resolve(undefined);
		}
	});

	return Promise.all(promises)
		.then((contracts) =>
			contracts
				.filter((c) => c !== undefined)
				.map((c) => c!)
				.map(
					(ir) =>
						({
							address: ir,
							name: `${getContractName(ir)} (Compliance Contract)`,
							type: ContractType.RwaCompliance,
						}) as Contract,
				),
		)
		.then((contracts) =>
			contracts.filter(
				(c, i, self) =>
					self.findIndex((cc) => cc.address.index === c.address.index) === i,
			),
		);
};

const getComplianceModules = async (
	grpcClient: ConcordiumGRPCClient,
	complianceContracts: Contract[],
): Promise<Contract[]> => {
	const promises = complianceContracts
		.filter((c) => c.type === ContractType.RwaCompliance)
		.map((contract) =>
			rwaCompliance.modules
				.invoke(grpcClient, contract.address)
				.then((res) => rwaCompliance.modules.parseReturnValue(res.returnValue!))
				.then((modules) =>
					modules!.map((m) => ContractAddress.create(m.index, m.subindex)),
				),
		);
	const modules = await Promise.all(promises);
	return modules.flat().map(
		(m) =>
			({
				address: m,
				name: `${getContractName(m)} (Compliance Module)`,
				type: ContractType.RwaComplianceModule,
			}) as Contract,
	);
};

const Contracts = (props: {
	contract: ContractAddress.Type;
	wallet: WalletApi;
	currentAccount: AccountAddress.Type;
}) => {
	const { contract } = props;
	const { provider: grpcClient } = useNodeClient();
	const [tokenContracts, setTokenContracts] = useState<Contract[]>([]);
	const [identityRegistries, setIdentityRegistries] = useState<Contract[]>([]);
	const [complianceContracts, setComplianceContracts] = useState<Contract[]>(
		[],
	);
	const [complianceModules, setComplianceModules] = useState<Contract[]>([]);
	const [marketContracts] = useState<Contract[]>([
		{
			address: props.contract,
			name: "Market",
			type: ContractType.RwaMarket,
		},
	]);

	useEffect(() => {
		setTokenContracts([]);
		getTokenContracts(grpcClient, contract).then((contracts) =>
			setTokenContracts(contracts),
		);
	}, [contract, grpcClient]);

	useEffect(() => {
		getIdentityRegistries(grpcClient, tokenContracts).then((contracts) =>
			setIdentityRegistries(contracts),
		);
		getComplianceContracts(grpcClient, tokenContracts).then((contracts) =>
			setComplianceContracts(contracts),
		);
	}, [tokenContracts, grpcClient]);

	useEffect(() => {
		getComplianceModules(grpcClient, complianceContracts).then((contracts) =>
			setComplianceModules(contracts),
		);
	}, [grpcClient, complianceContracts]);

	return (
		<ContractsList
			contracts={[
				...marketContracts,
				...tokenContracts,
				...identityRegistries,
				...complianceContracts,
				...complianceModules,
			]}
		/>
	);
};

export default function Admin(props: {
	contract: ContractAddress.Type;
	wallet: WalletApi;
	currentAccount: AccountAddress.Type;
}) {
	return (
		<Paper variant="outlined" sx={{ p: 2, m: 1 }}>
			<Routes>
				<Route path="" element={<Contracts {...props} />} />
				<Route path={ContractType.RwaIdentityRegistry}>
					<Route
						path=":index/:subIndex/*"
						Component={RwaIdentityRegistryContract}
					/>
				</Route>
				<Route path={ContractType.RwaCompliance}>
					<Route
						path=":index/:subIndex/*"
						element={
							<ConcordiumContract
								contractType={ContractType.RwaCompliance}
								entrypoints={rwaComplianceEntrypoints}
								entrypointDisplayNames={rwaComplianceEntrypointNames}
								entrypointUi={rwaComplianceEntrypointsUI}
							/>
						}
					/>
				</Route>
				<Route path={ContractType.RwaSecurityNft}>
					<Route path=":index/:subIndex/*" Component={RwaSecurityNftContract} />
				</Route>
				<Route path={ContractType.RwaSecuritySft}>
					<Route path=":index/:subIndex/*" Component={RwaSecuritySftContract} />
				</Route>
				<Route path={ContractType.RwaComplianceModule}>
					<Route
						path=":index/:subIndex/*"
						element={
							<ConcordiumContract
								contractType={ContractType.RwaComplianceModule}
								entrypoints={rwaComplianceModuleEntrypoints}
								entrypointDisplayNames={rwaComplianceModuleEntrypointNames}
								entrypointUi={rwaComplianceModuleEntrypointsUI}
							/>
						}
					/>
				</Route>
				<Route path={ContractType.RwaMarket}>
					<Route
						path=":index/:subIndex/*"
						element={
							<ConcordiumContract
								contractType={ContractType.RwaMarket}
								entrypoints={rwaMarketEntrypoints}
								entrypointDisplayNames={rwaMarketEntrypointNames}
								entrypointUi={rwaMarketEntrypointsUI}
							/>
						}
					/>
				</Route>
				<Route path="*" element={<ErrorDisplay text="Not Found Path" />} />
			</Routes>
		</Paper>
	);
}
