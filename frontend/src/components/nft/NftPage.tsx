import { ContractAddress } from "@concordium/web-sdk";
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
import { DRAWER_WIDTH } from "../common/consts";
import { Token } from "@mui/icons-material";
import TokensList from "./TokensList";
import { useNodeClient } from "../NodeClientProvider";

export default function NftPage() {
	const market_contract_index = import.meta.env.VITE_NFT_MARKET_CONTRACT_INDEX;
	const market_contract_subindex = import.meta.env
		.VITE_NFT_MARKET_CONTRACT_SUBINDEX;
	const sft_contract_index: string = import.meta.env.VITE_NFT_SFT_CONTRACT_INDEX;
	const sft_contract_subindex: string = import.meta.env
		.VITE_NFT_SFT_CONTRACT_SUBINDEX;
	let marketContract: ContractAddress.Type | undefined;
	if (market_contract_index && market_contract_subindex) {
		marketContract = ContractAddress.create(
			BigInt(market_contract_index),
			BigInt(market_contract_subindex),
		);
	}
	let sftContract: ContractAddress.Type | undefined;
	if (sft_contract_index && sft_contract_subindex) {
		sftContract = ContractAddress.create(
			BigInt(sft_contract_index),
			BigInt(sft_contract_subindex),
		);
	}

	const { pathname } = useLocation();
	const paths = pathname.split("/");
	const path = paths[paths.length - 1];

	const { index, subIndex } = useParams();
	const contract = ContractAddress.create(BigInt(index!), BigInt(subIndex!));
	const navigate = useNavigate();
	const { currentAccount, provider: walletApi } = useWallet();
	const { provider: grpcClient } = useNodeClient();
	return (
		<Box sx={{ display: "flex" }}>
			<CssBaseline />
			<Drawer
				variant="permanent"
				anchor="left"
				sx={{
					width: DRAWER_WIDTH,
					flexShrink: 0,
					[`& .MuiDrawer-paper`]: {
						width: DRAWER_WIDTH,
						boxSizing: "border-box",
					},
				}}
			>
				<Toolbar />
				<Divider />
				<Box sx={{ overflow: "auto" }}>
					<List>
						<ListItemButton
							selected={path === "tokens"}
							onClick={() => navigate("tokens")}
						>
							<ListItemIcon>
								<Token />
							</ListItemIcon>
							<ListItemText primary="Tokens" secondary="Your Tokens" />
						</ListItemButton>
					</List>
				</Box>
			</Drawer>
			<Box component="main" sx={{ flexGrow: 1, p: 0 }}>
				<Routes>
					<Route
						path="tokens"
						element={
							<TokensList
								currentAccount={currentAccount!}
								walletApi={walletApi!}
								contract={contract}
								grpcClient={grpcClient}
								marketContract={marketContract}
								sftContract={sftContract}
							/>
						}
					/>
					<Route path="" element={<Navigate to="tokens" replace />} />
				</Routes>
			</Box>
		</Box>
	);
}
