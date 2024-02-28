import {
	AppBar,
	Icon,
	IconButton,
	ListItemIcon,
	Menu,
	MenuItem,
	Paper,
	Toolbar,
	Typography,
} from "@mui/material";
import { Route, Routes, useNavigate } from "react-router-dom";
import { Contract, ContractType } from "./ContractTypes";
import { useReducer, useState } from "react";
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
import {
	EventType,
	WalletApi,
	detectConcordiumProvider,
} from "@concordium/browser-wallet-api-helpers";
import { AccountAddress } from "@concordium/web-sdk";
import {
	AccountCircle,
	HomeRounded,
	Login,
	Logout,
	Error,
} from "@mui/icons-material";
import { grey } from "@mui/material/colors";
import InfoDisplay from "../common/InfoDisplay";
import ConcordiumWalletProvider from "../WalletProvider";

const ContractsAppBar = (props: {
	onLogin: (account: AccountAddress.Type, wallet: WalletApi) => void;
	onLogout: () => void;
}) => {
	const navigate = useNavigate();
	const [error, setError] = useState("");
	const [account, setAccount] = useState<AccountAddress.Type>();
	const isLoggedIn = account !== undefined;

	const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);

	const handleMenu = (event: React.MouseEvent<HTMLElement>) => {
		setAnchorEl(event.currentTarget);
	};

	const handleClose = () => {
		setAnchorEl(null);
	};

	const login = async () => {
		const provider = await detectConcordiumProvider();
		if (!provider) {
			setError("No Concordium Wallet detected");
			console.error("No Concordium Wallet detected");
			return;
		}
		provider.addListener(EventType.AccountChanged, (newAccount) => {
			props.onLogout();
			setAccount(AccountAddress.fromBase58(newAccount));
			props.onLogin(AccountAddress.fromBase58(newAccount), provider);
		});
		provider.addListener(EventType.AccountDisconnected, () => {
			setAccount(undefined);
			props.onLogout();
		});
		//login process

		const currentAccount = await provider.getMostRecentlySelectedAccount();
		if (!currentAccount) {
			const accounts = await provider.requestAccounts();
			if (accounts.length === 0) {
				setError("No account selected");
				console.error("No account selected");
				return;
			}
			const account = AccountAddress.fromBase58(accounts[0]);
			setAccount(account);
			props.onLogin(account, provider);
		} else {
			const account = AccountAddress.fromBase58(currentAccount);
			setAccount(account);
			props.onLogin(account, provider);
		}
	};

	const logout = async () => {
		//logout process
		setAccount(undefined);
		handleClose();
		props.onLogout();
	};

	return (
		<AppBar position="static" sx={{ bgcolor: grey[800] }}>
			<Toolbar>
				<IconButton onClick={() => navigate("")}>
					<Icon sx={{ fontSize: 30 }}>
						<HomeRounded sx={{ fontSize: 30, color: grey[50] }} />
					</Icon>
				</IconButton>
				<Typography fontSize={30} component="div" sx={{ flexGrow: 1 }}>
					Global Admin
				</Typography>
				{error && (
					<IconButton
						title={error}
						aria-label="login"
						aria-controls="menu-appbar"
						onClick={() => login()}
						color="inherit"
					>
						<Error />
					</IconButton>
				)}
				{!isLoggedIn && (
					<IconButton
						size="large"
						title="Login"
						aria-label="login"
						aria-controls="menu-appbar"
						onClick={() => login()}
						color="inherit"
					>
						<Login />
					</IconButton>
				)}
				{isLoggedIn && (
					<>
						<IconButton
							size="large"
							aria-label="account of current user"
							aria-controls="menu-appbar"
							aria-haspopup="true"
							onClick={handleMenu}
							color="inherit"
							title={account!.address}
						>
							<AccountCircle />
						</IconButton>
						<Menu
							id="menu-appbar"
							anchorEl={anchorEl}
							anchorOrigin={{
								vertical: "top",
								horizontal: "right",
							}}
							keepMounted
							transformOrigin={{
								vertical: "top",
								horizontal: "right",
							}}
							open={Boolean(anchorEl)}
							onClose={handleClose}
						>
							<MenuItem onClick={logout}>
								<ListItemIcon>
									<Logout fontSize="small" />
								</ListItemIcon>
								<Typography variant="inherit" noWrap>
									Logout
								</Typography>
							</MenuItem>
						</Menu>
					</>
				)}
			</Toolbar>
		</AppBar>
	);
};

const ConnectedContent = (props: {
	wallet: WalletApi;
	account: AccountAddress.Type;
}) => {
	const [state, dispatch] = useReducer(reducer, initialState());
	const navigate = useNavigate();

	const onInitClicked = (contractType: ContractType) => {
		navigate(`${contractType}/init`);
	};
	const onContractInitialized = (contract: Contract) => {
		dispatch({ type: ActionTypes.AddContract, contract });
	};
	const onDeleteContract = (contract: Contract) => {
		dispatch({ type: ActionTypes.RemoveContract, contract });
	};

	const { wallet, account } = props;
	return (
		<Paper variant="outlined" sx={{ p: 2, m: 1 }}>
			<Routes>
				<Route
					path=""
					element={
						<ContractsList
							contracts={state.contracts}
							onDelete={onDeleteContract}
							onInit={onInitClicked}
						/>
					}
				/>
				<Route path={ContractType.RwaIdentityRegistry}>
					<Route
						path="init"
						element={
							<IdentityRegistryInitialize
								wallet={wallet}
								currentAccount={account}
								onSuccess={onContractInitialized}
							/>
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
								wallet={wallet}
								currentAccount={account}
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
								wallet={wallet}
								currentAccount={account}
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
								wallet={wallet}
								currentAccount={account}
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
								wallet={wallet}
								currentAccount={account}
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
								wallet={wallet}
								currentAccount={account}
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
};

export default function ContractsPage() {
	const navigate = useNavigate();
	const [wallet, setWallet] = useState<{
		wallet: WalletApi;
		account: AccountAddress.Type;
	}>();
	const isLoggedIn = wallet !== undefined;
	const onLogout = () => {
		setWallet(undefined);
		navigate("");
	};

	const DisconnectedContent = () => {
		return InfoDisplay({
			text: "Please connect to Concordium Wallet to use the marketplace",
		});
	};

	return (
		<>
			<ContractsAppBar
				onLogin={(account, wallet) => setWallet({ account, wallet })}
				onLogout={onLogout}
			/>
			{isLoggedIn ? (
				<ConcordiumWalletProvider>
					<ConnectedContent wallet={wallet!.wallet} account={wallet!.account} />
				</ConcordiumWalletProvider>
			) : (
				<DisconnectedContent />
			)}
		</>
	);
}
