import { AccountAddress, ContractAddress } from "@concordium/web-sdk";
import {
	AppBar,
	Divider,
	Icon,
	IconButton,
	ListItemIcon,
	Menu,
	MenuItem,
	Toolbar,
	Typography,
} from "@mui/material";
import {
	Navigate,
	Route,
	Routes,
	useNavigate,
	useParams,
} from "react-router-dom";
import WithdrawToken from "./WithdrawToken";
import UnListedTokens from "./UnListedTokens";
import ListToken from "./ListToken";
import ListedTokens from "./ListedTokens";
import Exchange from "./Exchange";
import TransferList from "./TransferList";
import rwaMarket from "../../lib/rwaMarket";
import {
	AccountCircle,
	AdminPanelSettings,
	AppRegistrationRounded,
	Error,
	HomeRounded,
	Login,
	Logout,
	SellRounded,
	ShopRounded,
	Token,
} from "@mui/icons-material";
import { grey } from "@mui/material/colors";
import { useState } from "react";
import {
	EventType,
	WalletApi,
	detectConcordiumProvider,
} from "@concordium/browser-wallet-api-helpers";
import InfoDisplay from "../common/InfoDisplay";
import UserOwnedTokens from "./UserOwnedTokens";
import Admin from "./Admin";
import Registration from "../verifier/Registration";
import VerifierApiProvider from "../VerifierApiProvider";
import SponsorApiProvider from "../SponsorApiProvider";
import { SPONSOR_CONTRACT_INDEX, SPONSOR_CONTRACT_SUBINDEX } from "./consts";

const MarketAppBar = (props: {
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
		<AppBar position="static" sx={{ bgcolor: grey[900] }}>
			<Toolbar>
				<IconButton onClick={() => navigate("")}>
					<Icon sx={{ fontSize: 30 }}>
						<HomeRounded sx={{ fontSize: 30, color: grey[50] }} />
					</Icon>
				</IconButton>
				<Typography fontSize={30} component="div" sx={{ flexGrow: 1 }}>
					Marketplace
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
							<MenuItem
								onClick={() => {
									handleClose();
									navigate("");
								}}
							>
								<ListItemIcon>
									<HomeRounded fontSize="small" />
								</ListItemIcon>
								<Typography variant="inherit" noWrap>
									Market
								</Typography>
							</MenuItem>
							<MenuItem
								onClick={() => {
									handleClose();
									navigate("un-listed-tokens");
								}}
							>
								<ListItemIcon>
									<SellRounded fontSize="small" />
								</ListItemIcon>
								<Typography variant="inherit" noWrap>
									Un Listed Tokens
								</Typography>
							</MenuItem>
							<MenuItem
								onClick={() => {
									handleClose();
									navigate("transferList");
								}}
							>
								<ListItemIcon>
									<ShopRounded fontSize="small" />
								</ListItemIcon>
								<Typography variant="inherit" noWrap>
									List
								</Typography>
							</MenuItem>
							<MenuItem
								onClick={() => {
									handleClose();
									navigate("admin");
								}}
							>
								<ListItemIcon>
									<AdminPanelSettings fontSize="small" />
								</ListItemIcon>
								<Typography variant="inherit" noWrap>
									Admin
								</Typography>
							</MenuItem>
							<Divider />
							<MenuItem
								onClick={() => {
									handleClose();
									navigate("tokens");
								}}
							>
								<ListItemIcon>
									<Token fontSize="small" />
								</ListItemIcon>
								<Typography variant="inherit" noWrap>
									Tokens
								</Typography>
							</MenuItem>
							<Divider />
							<Divider />
							<MenuItem
								onClick={() => {
									handleClose();
									navigate("register");
								}}
							>
								<ListItemIcon>
									<AppRegistrationRounded fontSize="small" />
								</ListItemIcon>
								<Typography variant="inherit" noWrap>
									Register
								</Typography>
							</MenuItem>
							<Divider />
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

export default function MarketPage() {
	const { index, subIndex } = useParams();
	const contract = ContractAddress.create(BigInt(index!), BigInt(subIndex!));
	const navigate = useNavigate();

	let sponsorContract: ContractAddress.Type | undefined = undefined;
	if (SPONSOR_CONTRACT_INDEX && SPONSOR_CONTRACT_SUBINDEX) {
		sponsorContract = ContractAddress.create(
			BigInt(SPONSOR_CONTRACT_INDEX!),
			BigInt(SPONSOR_CONTRACT_SUBINDEX!),
		);
	}
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

	const ConnectedContent = (props: {
		wallet: WalletApi;
		account: AccountAddress.Type;
	}) => {
		const { wallet, account } = props;
		return (
			<Routes>
				<Route
					path="listed-tokens"
					element={
						<ListedTokens
							contract={contract}
							currentAccount={account}
							onDeList={(token) =>
								rwaMarket.deList.update(wallet, account, contract, {
									owner: token.owner,
									token_id: {
										id: token.token_id,
										contract: token.token_contract,
									},
								})
							}
							onExchange={(token) => navigate("exchange", { state: token })}
							onList={(token) => navigate("list", { state: token })}
						/>
					}
				/>
				<Route
					path="un-listed-tokens"
					element={
						<UnListedTokens
							contract={contract}
							owner={account}
							onWithdraw={(token) => navigate("withdraw", { state: token })}
							onList={(token) => navigate("list", { state: token })}
						/>
					}
				/>
				<Route
					path="withdraw"
					element={
						<WithdrawToken
							wallet={wallet}
							currentAccount={account}
							contract={contract}
						/>
					}
				/>
				<Route
					path="list"
					element={
						<ListToken
							wallet={wallet}
							currentAccount={account}
							contract={contract}
						/>
					}
				/>
				<Route
					path="transferList"
					element={
						<SponsorApiProvider>
							<TransferList
								wallet={wallet}
								currentAccount={account}
								contract={contract}
								sponsorContract={sponsorContract}
							/>
						</SponsorApiProvider>
					}
				/>
				<Route
					path="transferList/:listContractIndex/:listContractSubIndex/:listTokenId/:listAmount"
					element={
						<SponsorApiProvider>
							<TransferList
								wallet={wallet}
								currentAccount={account}
								contract={contract}
								sponsorContract={sponsorContract}
							/>
						</SponsorApiProvider>
					}
				/>
				<Route path="de-list" element={<div>De List</div>} />
				<Route
					path="exchange"
					element={
						<Exchange
							contract={contract}
							walletApi={wallet}
							currentAccount={account}
						/>
					}
				/>
				<Route
					path="tokens"
					element={
						<SponsorApiProvider>
							<UserOwnedTokens
								contract={contract}
								currentAccount={account}
								wallet={wallet}
								sponsorContract={sponsorContract}
							/>
						</SponsorApiProvider>
					}
				/>
				<Route
					path="admin/*"
					element={
						<Admin
							contract={contract}
							currentAccount={account}
							wallet={wallet}
						/>
					}
				/>
				<Route
					path="register"
					element={
						<VerifierApiProvider>
							<Registration wallet={wallet} currentAccount={account} />
						</VerifierApiProvider>
					}
				/>
				<Route path="" element={<Navigate to="listed-tokens" replace />} />
			</Routes>
		);
	};

	return (
		<>
			<MarketAppBar
				onLogin={(account, wallet) => setWallet({ account, wallet })}
				onLogout={onLogout}
			/>
			{isLoggedIn ? (
				<ConnectedContent wallet={wallet!.wallet} account={wallet!.account} />
			) : (
				<DisconnectedContent />
			)}
		</>
	);
}
