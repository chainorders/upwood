import { Paper } from "@mui/material";
import { Route, Routes } from "react-router-dom";
import { Contract, ContractType } from "./ContractTypes";
import { useReducer } from "react";
import { ActionTypes, initialState, reducer } from "../../AppState";
import ContractsList from "./ContractsList";
import ConcordiumContract from "./ConcordiumContract";
import ContractLayout from "./ContractLayout";
import ErrorDisplay from "../common/ErrorDisplay";
import { default as RwaSecurityNftInitialize } from "./RwaSecurityNftInitialize";
import { default as IdentityRegistryInitialize } from "./RwaIdentityRegistryInitialize";
import { default as ComplianceInitialize } from "./RwaComplianceInitialize";
import {
	ENTRYPOINT_DISPLAY_NAMES as rwaComplianceEntrypointNames,
	ENTRYPOINTS as rwaComplianceEntrypoints,
} from "../../lib/rwaCompliance";
import { ENTRYPOINTS_UI as rwaComplianceEntrypointsUI } from "../../lib/rwaComplianceUi";
import { default as RWAComplianceModuleInitialize } from "./RwaComplianceModuleAllowedNationalitiesInitialize";
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
import RwaMarketInitialize from "./RwaMarketInitialize";
import RwaSecurityNftContract from "./rwaSecurityNft/RwaSecurityNftContract";
import RwaSecuritySftContract from "./rwaSecuritySft/RwaSecuritySftContract";
import RwaSecuritySftInitialize from "./RwaSecuritySftInitialize";
import RwaIdentityRegistryContract from "./rwaIdentityRegistry/RwaIdentityRegistryContract";

export default function ContractsPage() {
	const [state, dispatch] = useReducer(reducer, initialState());
	const onContractInitialized = (contract: Contract) => {
		dispatch({ type: ActionTypes.AddContract, contract });
	};
	const onDeleteContract = (contract: Contract) => {
		dispatch({ type: ActionTypes.RemoveContract, contract });
	};
	return (
		<Paper variant="outlined" sx={{ p: 2, m: 1 }}>
			<Routes>
				<Route
					path=""
					element={
						<ContractsList
							contracts={state.contracts}
							onDelete={onDeleteContract}
						/>
					}
				/>
				<Route path={ContractType.RwaIdentityRegistry}>
					<Route
						path="init"
						element={
							<IdentityRegistryInitialize onSuccess={onContractInitialized} />
						}
					/>
					<Route
						path=":index/:subIndex/*"
						element={<ContractLayout contracts={state.contracts} />}
					>
						<Route path="*" Component={RwaIdentityRegistryContract} />
					</Route>
				</Route>
				<Route path={ContractType.RwaCompliance}>
					<Route
						path="init"
						element={
							<ComplianceInitialize
								onSuccess={onContractInitialized}
								complianceModules={state.contracts.filter(
									(c) => c.type == ContractType.RwaComplianceModule,
								)}
							/>
						}
					/>
					<Route
						path=":index/:subIndex/*"
						element={<ContractLayout contracts={state.contracts} />}
					>
						<Route
							path="*"
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
				</Route>
				<Route path={ContractType.RwaSecurityNft}>
					<Route
						path="init"
						element={
							<RwaSecurityNftInitialize
								onSuccess={onContractInitialized}
								identityRegistries={state.contracts.filter(
									(contract) =>
										contract.type === ContractType.RwaIdentityRegistry,
								)}
								complianceContracts={state.contracts.filter(
									(contract) => contract.type === ContractType.RwaCompliance,
								)}
							/>
						}
					/>
					<Route
						path=":index/:subIndex/*"
						element={<ContractLayout contracts={state.contracts} />}
					>
						<Route path="*" Component={RwaSecurityNftContract} />
					</Route>
				</Route>
				<Route path={ContractType.RwaSecuritySft}>
					<Route
						path="init"
						element={
							<RwaSecuritySftInitialize
								onSuccess={onContractInitialized}
								identityRegistries={state.contracts.filter(
									(contract) =>
										contract.type === ContractType.RwaIdentityRegistry,
								)}
								complianceContracts={state.contracts.filter(
									(contract) => contract.type === ContractType.RwaCompliance,
								)}
							/>
						}
					/>
					<Route
						path=":index/:subIndex/*"
						element={<ContractLayout contracts={state.contracts} />}
					>
						<Route path="*" Component={RwaSecuritySftContract} />
					</Route>
				</Route>
				<Route path={ContractType.RwaComplianceModule}>
					<Route
						path="init"
						element={
							<RWAComplianceModuleInitialize
								onSuccess={onContractInitialized}
								identityRegistries={state.contracts.filter(
									(contract) =>
										contract.type === ContractType.RwaIdentityRegistry,
								)}
							/>
						}
					/>
					<Route
						path=":index/:subIndex/*"
						element={<ContractLayout contracts={state.contracts} />}
					>
						<Route
							path="*"
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
				</Route>
				<Route path={ContractType.RwaMarket}>
					<Route
						path="init"
						element={
							<RwaMarketInitialize
								onSuccess={onContractInitialized}
								existingTokenContracts={state.contracts.filter(
									(contract) =>
										contract.type === ContractType.RwaSecurityNft ||
										contract.type === ContractType.RwaSecuritySft,
								)}
							/>
						}
					/>
					<Route
						path=":index/:subIndex/*"
						element={<ContractLayout contracts={state.contracts} />}
					>
						<Route
							path="*"
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
				</Route>
				<Route path="*" element={<ErrorDisplay text="Not Found Path" />} />
			</Routes>
		</Paper>
	);
}
