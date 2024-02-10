import { ContractAddress } from "@concordium/web-sdk";
import { SellRounded, SendRounded, ShopRounded } from "@mui/icons-material";
import {
	Box,
	CssBaseline,
	Divider,
	Drawer,
	List,
	ListItemButton,
	ListItemIcon,
	ListItemText,
	Toolbar,
} from "@mui/material";
import {
	Navigate,
	Route,
	Routes,
	useLocation,
	useNavigate,
	useParams,
} from "react-router-dom";
import { useWallet } from "../WalletProvider";
import WithdrawToken from "./WithdrawToken";
import UnListedTokens from "./UnListedTokens";
import ListToken from "./ListToken";
import ListedTokens from "./ListedTokens";
import Exchange from "./Exchange";
import TransferList from "./TransferList";
import rwaMarket from "../../lib/rwaMarket";
const drawerWidth = 240;

export default function MarketPage() {
	const { pathname } = useLocation();
	const paths = pathname.split("/");
	const path = paths[paths.length - 1];

	const { index, subIndex } = useParams();
	const contract = ContractAddress.create(BigInt(index!), BigInt(subIndex!));
	const navigate = useNavigate();
	const { currentAccount, provider: walletApi } = useWallet();

	return (
		<Box sx={{ display: "flex" }}>
			<CssBaseline />
			<Drawer
				variant="permanent"
				anchor="left"
				sx={{
					width: drawerWidth,
					flexShrink: 0,
					[`& .MuiDrawer-paper`]: {
						width: drawerWidth,
						boxSizing: "border-box",
					},
				}}
			>
				<Toolbar />
				<Divider />
				<Box sx={{ overflow: "auto" }}>
					<List>
						<ListItemButton
							selected={path === "listed-tokens"}
							onClick={() => navigate("listed-tokens")}
						>
							<ListItemIcon>
								<ShopRounded />
							</ListItemIcon>
							<ListItemText
								primary="Listed Tokens"
								secondary="Tokens availed to be bought / exchanged"
							/>
						</ListItemButton>
						<ListItemButton
							selected={path === "un-listed-tokens"}
							onClick={() => navigate("un-listed-tokens")}
						>
							<ListItemIcon>
								<SellRounded />
							</ListItemIcon>
							<ListItemText
								primary="Un Listed Tokens"
								secondary="Tokens which have been deposited to Market Contract but not listed to sell"
							/>
						</ListItemButton>
						<ListItemButton
							selected={path === "transferList"}
							onClick={() => navigate("transferList")}
						>
							<ListItemIcon>
								<SendRounded />
							</ListItemIcon>
							<ListItemText
								primary="Transfer & List"
								secondary="Transfer tokens from any CIS2 contract to Market contract"
							/>
						</ListItemButton>
					</List>
				</Box>
			</Drawer>
			<Box component="main" sx={{ flexGrow: 1, p: 0 }}>
				<Routes>
					<Route
						path="listed-tokens"
						element={
							<ListedTokens
								contract={contract}
								currentAccount={currentAccount!}
								onDeList={(token) =>
									rwaMarket.deList.update(
										walletApi!,
										currentAccount!,
										contract,
										{
											owner: token.owner,
											token_id: {
												id: token.token_id,
												contract: token.token_contract,
											},
										},
									)
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
								owner={currentAccount!}
								onWithdraw={(token) => navigate("withdraw", { state: token })}
								onList={(token) => navigate("list", { state: token })}
							/>
						}
					/>
					<Route
						path="withdraw"
						element={<WithdrawToken contract={contract} />}
					/>
					<Route path="list" element={<ListToken contract={contract} />} />
					<Route
						path="transferList"
						element={<TransferList contract={contract} />}
					/>
					<Route path="de-list" element={<div>De List</div>} />
					<Route
						path="exchange"
						element={
							<Exchange
								contract={contract}
								walletApi={walletApi!}
								currentAccount={currentAccount!}
							/>
						}
					/>
					<Route path="" element={<Navigate to="listed-tokens" replace />} />
				</Routes>
			</Box>
		</Box>
	);
}
