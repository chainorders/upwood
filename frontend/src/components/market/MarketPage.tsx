import { ContractAddress } from "@concordium/web-sdk";
import { AppBar, Icon, IconButton, Toolbar, Typography } from "@mui/material";
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
import { HomeRounded, SellRounded, ShopRounded } from "@mui/icons-material";
import { grey } from "@mui/material/colors";

export default function MarketPage() {
	const { pathname } = useLocation();
	const paths = pathname.split("/");
	const path = paths[paths.length - 1];

	const { index, subIndex } = useParams();
	const contract = ContractAddress.create(BigInt(index!), BigInt(subIndex!));
	const navigate = useNavigate();
	const { currentAccount, provider: walletApi } = useWallet();

	return (
		<>
			<AppBar position="static" sx={{ bgcolor: grey[800], mt: -2 }}>
				<Toolbar>
					<IconButton onClick={() => navigate("")}>
						<Icon sx={{ fontSize: 30 }}>
							<HomeRounded sx={{ fontSize: 30, color: grey[50] }} />
						</Icon>
					</IconButton>
					<Typography fontSize={30} component="div" sx={{ flexGrow: 1 }}>
						Marketplace
					</Typography>
					<IconButton
						sx={{ color: grey[50], m: 2 }}
						onClick={() => navigate("un-listed-tokens")}
					>
						<Icon>
							<SellRounded sx={{ color: grey[50] }} />
						</Icon>
						<Typography ml={1}>Un Listed Tokens</Typography>
					</IconButton>
					<IconButton
						sx={{ color: grey[50], m: 2 }}
						onClick={() => navigate("transferList")}
					>
						<Icon>
							<ShopRounded sx={{ color: grey[50] }} />
						</Icon>
						<Typography ml={1}>List</Typography>
					</IconButton>
				</Toolbar>
			</AppBar>
			<Routes>
				<Route
					path="listed-tokens"
					element={
						<ListedTokens
							contract={contract}
							currentAccount={currentAccount!}
							onDeList={(token) =>
								rwaMarket.deList.update(walletApi!, currentAccount!, contract, {
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
				<Route
					path="transferList/:listContractIndex/:listContractSubIndex/:listTokenId/:listAmount"
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
		</>
	);
}
