import { Link, Navigate, Route, Routes, useLocation } from "react-router";
import {
	AppBar,
	Toolbar,
	Typography,
	Box,
	Drawer,
	List,
	ListItem,
	ListItemText,
	ListItemIcon,
	IconButton,
} from "@mui/material";
import { User } from "../lib/user.ts";
import ProjectIcon from "@mui/icons-material/Folder";
import UsersIcon from "@mui/icons-material/Group";
import NewspaperIcon from "@mui/icons-material/Newspaper";
import logoImage from "../assets/logo.svg";
import LogoutIcon from "@mui/icons-material/Logout";
import HandymanIcon from "@mui/icons-material/Handyman";
import UpdateIcon from "@mui/icons-material/Update";
import HelpOutlineIcon from "@mui/icons-material/HelpOutline";
import SupportAgentIcon from "@mui/icons-material/SupportAgent";
import ForestIcon from "@mui/icons-material/Forest";
import LandscapeIcon from "@mui/icons-material/Landscape";
import WalletIcon from "@mui/icons-material/Wallet";
import CurrencyExchangeIcon from "@mui/icons-material/CurrencyExchange";
import StoreIcon from "@mui/icons-material/Store";
import { lazy, Suspense } from "react";
import UpdateMessagesList from "./pages/UpdateMessagesList.tsx";
import TreeFungibleHoldersList from "./pages/TreeFungibleHoldersList.tsx";
import TreeMetadataList from "./pages/TreeMetadataList.tsx";
import InvestorsList from "./pages/InvestorsList.tsx";
import TradersList from "./pages/TradersList.tsx";
import TokenHoldersList from "./pages/TokenHoldersList.tsx";

const ProjectList = lazy(() => import("./pages/ProjectList.tsx"));
const ProjectCreate = lazy(() => import("./pages/ProjectCreate.tsx"));
const ProjectUpdate = lazy(() => import("./pages/ProjectUpdate.tsx"));
const ProjectDetails = lazy(() => import("./pages/ProjectDetails.tsx"));
const ProjectContractCreate = lazy(() => import("./pages/ProjectContractCreate.tsx"));
const ProjectContractDetails = lazy(() => import("./pages/ProjectContractDetails.tsx"));
const ProjectContractUpdate = lazy(() => import("./pages/ProjectContractUpdate.tsx"));
const ProjectTokenDetails = lazy(() => import("./pages/ProjectTokenDetails.tsx"));
const UserList = lazy(() => import("./pages/UserList.tsx"));
const UserInvitations = lazy(() => import("./pages/UserInvitations.tsx"));
const NewsList = lazy(() => import("./pages/NewsList.tsx"));
const MaintenanceMessageList = lazy(() => import("./pages/MaintenanceMessageList.tsx"));
const GuideList = lazy(() => import("./pages/GuideList.tsx"));
const QuestionsList = lazy(() => import("./pages/QuestionsList.tsx"));

function UserAvatar({ user }: { user: User }) {
	return (
		<Box display="flex" alignItems="center">
			<Box
				sx={{
					width: 40,
					height: 40,
					borderRadius: "50%",
					backgroundColor: "primary.main",
					display: "flex",
					alignItems: "center",
					justifyContent: "center",
					color: "white",
					marginRight: 1,
				}}
			>
				{user.initialis}
			</Box>
			<Typography>{user.fullName}</Typography>
		</Box>
	);
}

export default function AdminApp({ user, logout }: { user?: User; logout: () => void }) {
	const location = useLocation();
	const pathname = location.pathname;
	const navItems = [
		{
			name: "Projects",
			url: "/admin/projects",
			icon: <ProjectIcon />,
			isActive: pathname.startsWith("/admin/projects"),
		},
		{
			name: "Fund Investors",
			url: "/admin/investors",
			icon: <CurrencyExchangeIcon />,
			isActive: pathname.startsWith("/admin/investors"),
		},
		{
			name: "Market Traders",
			url: "/admin/traders",
			icon: <StoreIcon />,
			isActive: pathname.startsWith("/admin/traders"),
		},
		{
			name: "Project Token Holders",
			url: "/admin/projects/token-holders",
			icon: <WalletIcon />,
			isActive: pathname.startsWith("/admin/projects/token-holders"),
		},
		{
			name: "Tree Fungible Token Holders",
			url: "/admin/tree/fungible/holders",
			icon: <ForestIcon />,
			isActive: pathname.startsWith("/admin/tree/fungible/holders"),
		},
		{
			name: "Users",
			url: "/admin/users",
			icon: <UsersIcon />,
			isActive: pathname.startsWith("/admin/users"),
		},
		{
			name: "News Articles",
			url: "/admin/news",
			icon: <NewspaperIcon />,
			isActive: pathname.startsWith("/admin/news"),
		},
		{
			name: "Maintenance Messages",
			url: "/admin/maintenance",
			icon: <HandymanIcon />,
			isActive: pathname.startsWith("/admin/maintenance"),
		},
		{
			name: "Platform Updates",
			url: "/admin/updates",
			icon: <UpdateIcon />,
			isActive: pathname.startsWith("/admin/updates"),
		},
		{
			name: "Guides",
			url: "/admin/guides",
			icon: <HelpOutlineIcon />,
			isActive: pathname.startsWith("/admin/guides"),
		},
		{
			name: "Support Questions",
			url: "/admin/support",
			icon: <SupportAgentIcon />,
			isActive: pathname.startsWith("/admin/support"),
		},
		{
			name: "Tree metadatas",
			url: "/admin/tree/metadata",
			icon: <LandscapeIcon />,
			isActive: pathname.startsWith("/admin/tree/metadata"),
		},
	];

	return user && user.isAdmin ? (
		<Box sx={{ display: "flex" }}>
			<AppBar position="fixed" sx={{ zIndex: (theme) => theme.zIndex.drawer + 1 }}>
				<Toolbar>
					<IconButton edge="start" color="inherit" aria-label="menu" sx={{ mr: 2 }}>
						<img src={logoImage} alt="Website Logo" style={{ width: 40, height: 40, display: "block" }} />
					</IconButton>
					<Typography variant="h6" sx={{ flexGrow: 1 }}>
						Admin
					</Typography>
					<UserAvatar user={user} />
				</Toolbar>
			</AppBar>
			<Drawer
				variant="permanent"
				sx={{
					width: 240,
					flexShrink: 0,
					[`& .MuiDrawer-paper`]: { width: 240, boxSizing: "border-box" },
				}}
			>
				<Toolbar />
				<Box sx={{ overflow: "auto" }}>
					<List>
						{navItems.map((item) => (
							<ListItem button key={item.url} component={Link} to={item.url} selected={item.isActive}>
								<ListItemIcon>{item.icon}</ListItemIcon>
								<ListItemText primary={item.name} />
							</ListItem>
						))}
						<Box sx={{ flexGrow: 1 }} />
						<ListItem button onClick={logout}>
							<ListItemIcon>
								<LogoutIcon />
							</ListItemIcon>
							<ListItemText primary="Logout" />
						</ListItem>
					</List>
				</Box>
			</Drawer>
			<Box component="main" sx={{ flexGrow: 1, p: 3 }}>
				<Toolbar />
				<Suspense fallback={<div>Loading...</div>}>
					<Routes>
						<Route index element={<Navigate to="projects/list" />} />
						<Route path="projects/*">
							<Route index element={<ProjectList />} />
							<Route path="list" element={<ProjectList />} />
							<Route path="create" element={<ProjectCreate fileBaseUrl={import.meta.env.VITE_FILES_BASE_URL} />} />
							<Route path=":id">
								<Route path="update" element={<ProjectUpdate fileBaseUrl={import.meta.env.VITE_FILES_BASE_URL} />} />
								<Route path="details" element={<ProjectDetails fileBaseUrl={import.meta.env.VITE_FILES_BASE_URL} />} />
								<Route path="contract">
									<Route
										path="create"
										element={<ProjectContractCreate user={user} fileBaseUrl={import.meta.env.VITE_FILES_BASE_URL} />}
									/>
									<Route path=":contract_address">
										<Route path="details" element={<ProjectContractDetails user={user} />} />
										<Route
											path="update"
											element={<ProjectContractUpdate user={user} fileBaseUrl={import.meta.env.VITE_FILES_BASE_URL} />}
										/>
										<Route path="token">
											<Route path=":token_id">
												<Route path="details" element={<ProjectTokenDetails user={user} />} />
											</Route>
										</Route>
									</Route>
								</Route>
							</Route>
						</Route>
						<Route path="investors" element={<InvestorsList />} />
						<Route path="traders" element={<TradersList />} />
						<Route path="projects/token-holders" element={<TokenHoldersList />} />
						<Route path="users/*">
							<Route index element={<UserList user={user} />} />
							<Route path="list" element={<UserList user={user} />} />
							<Route path="invitations" element={<UserInvitations user={user} />} />
						</Route>
						<Route path="news/*">
							<Route index element={<Navigate to="list" />} />
							<Route path="list" element={<NewsList />} />
						</Route>
						<Route path="maintenance/*">
							<Route index element={<Navigate to="list" />} />
							<Route path="list" element={<MaintenanceMessageList />} />
						</Route>
						<Route path="updates/*">
							<Route index element={<Navigate to="list" />} />
							<Route path="list" element={<UpdateMessagesList />} />
						</Route>
						<Route path="guides/*">
							<Route index element={<Navigate to="list" />} />
							<Route path="list" element={<GuideList />} />
						</Route>
						<Route path="support/*">
							<Route index element={<Navigate to="list" />} />
							<Route path="list" element={<QuestionsList />} />
						</Route>
						<Route path="tree/fungible/holders" element={<TreeFungibleHoldersList user={user} />} />
						<Route
							path="tree/metadata"
							element={<TreeMetadataList user={user} fileBaseUrl={import.meta.env.VITE_FILES_BASE_URL} />}
						/>
					</Routes>
				</Suspense>
			</Box>
		</Box>
	) : (
		<Navigate to="/login" replace state={{ from: location }} />
	);
}
