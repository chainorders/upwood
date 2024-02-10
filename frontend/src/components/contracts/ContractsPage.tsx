import {
	Grid,
	List,
	ListItem,
	ListItemButton,
	ListItemText,
	Paper,
	Stack,
	Typography,
} from "@mui/material";
import { Link, Route, Routes, useNavigate } from "react-router-dom";
import { Contract, ContractType } from "./ContractTypes";
import { capitalCase } from "change-case";
import { useReducer } from "react";
import { ActionTypes, initialState, reducer } from "../../AppState";
import ContractsList from "./ContractsList";
import ConcordiumContract from "./ConcordiumContract";
import ContractLayout from "./ContractLayout";
import ErrorDisplay from "../common/ErrorDisplay";
import { default as RwaSecurityNftInitialize } from "./RwaSecurityNftInitialize";
import { default as IdentityRegistryInitialize } from "./RwaIdentityRegistryInitialize";
import {
	ENTRYPOINT_DISPLAY_NAMES as rwaIdentityRegistryEntrypointNames,
	ENTRYPOINTS as rwaIdentityRegistryEntrypoints,
} from "../../lib/rwaIdentityRegistry";
import { ENTRYPOINTS_UI as rwaIdentityRegistryEntrypointsUI } from "../../lib/rwaIdentityRegistryUi";
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

const contractTypes: Record<string, ContractType> = {
	rwaIdentityRegistry: ContractType.RwaIdentityRegistry,
	complianceModule: ContractType.RwaComplianceModule,
	compliance: ContractType.RwaCompliance,
	rwaSecurityNft: ContractType.RwaSecurityNft,
	rwaSecuritySft: ContractType.RwaSecuritySft,
	sponsor: ContractType.RwaSponsor,
	market: ContractType.RwaMarket,
};

export default function ContractsPage() {
	const navigate = useNavigate();
	const [state, dispatch] = useReducer(reducer, initialState());
	const onContractInitialized = (contract: Contract) => {
		dispatch({ type: ActionTypes.AddContract, contract });
	};
	const onDeleteContract = (contract: Contract) => {
		dispatch({ type: ActionTypes.RemoveContract, contract });
	};
	return (
		<Grid container spacing={0}>
			<Grid item xs={2} md={2}>
				<Paper variant="outlined" sx={{ pt: 2, m: 1 }}>
					<Stack m={0} p={0}>
						<Typography variant="h1" fontSize={28} m={0} p={2}>
							<Link to="/contracts">Contracts</Link>
						</Typography>
						<List>
							{Object.keys(contractTypes).map((key) => {
								const contractType = contractTypes[key];
								return (
									<ListItem disablePadding disableGutters key={contractType}>
										<ListItemButton
											onClick={() => navigate(`${contractType}/init`)}
										>
											<ListItemText
												primary={capitalCase(contractType)}
												secondary="Initialize"
											/>
										</ListItemButton>
									</ListItem>
								);
							})}
						</List>
					</Stack>
				</Paper>
			</Grid>
			<Grid item xs={10} md={10}>
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
									<IdentityRegistryInitialize
										onSuccess={onContractInitialized}
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
											entrypoints={rwaIdentityRegistryEntrypoints}
											entrypointDisplayNames={
												rwaIdentityRegistryEntrypointNames
											}
											entrypointUi={rwaIdentityRegistryEntrypointsUI}
										/>
									}
								/>
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
											(contract) =>
												contract.type === ContractType.RwaCompliance,
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
											(contract) =>
												contract.type === ContractType.RwaCompliance,
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
											entrypoints={rwaComplianceModuleEntrypoints}
											entrypointDisplayNames={
												rwaComplianceModuleEntrypointNames
											}
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
			</Grid>
		</Grid>
	);
}
