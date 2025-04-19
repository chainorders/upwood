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
	useTheme,
	alpha,
	Divider,
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
import LandscapeIcon from "@mui/icons-material/Landscape";
import { lazy, Suspense } from "react";
import UpdateMessagesList from "./pages/UpdateMessagesList.tsx";
import TreeMetadataList from "./pages/TreeMetadataList.tsx";
import TokenHolderBalanceUpdateList from "./pages/TokenHolderBalanceUpdateList.tsx";
import TokenIcon from "@mui/icons-material/Token";
import TokenContractsList from "./pages/TokenContractsList.tsx";
import TokenContractDetails from "./pages/TokenContractDetails.tsx";
import ForestTokenContractDetails from "./pages/ForestTokenContractDetails.tsx";

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
	const theme = useTheme();
	return (
		<Box display="flex" alignItems="center">
			<Box
				sx={{
					width: 40,
					height: 40,
					borderRadius: "50%",
					background: `linear-gradient(135deg, ${theme.palette.primary.main} 0%, ${theme.palette.primary.dark} 100%)`,
					display: "flex",
					alignItems: "center",
					justifyContent: "center",
					color: "white",
					marginRight: 1.5,
					boxShadow: "0 3px 5px rgba(0,0,0,0.2)",
					fontWeight: "bold",
				}}
			>
				{user.initialis}
			</Box>
			<Typography fontWeight="500">{user.fullName}</Typography>
		</Box>
	);
}

export default function AdminApp({ user, logout }: { user?: User; logout: () => void }) {
	const location = useLocation();
	const pathname = location.pathname;
	const theme = useTheme();

	const navItems = [
		{
			name: "Projects",
			url: "/admin/projects",
			icon: <ProjectIcon />,
			isActive: pathname.startsWith("/admin/projects"),
		},
		{
			name: "Token Contracts",
			url: "/admin/token-contracts",
			icon: <TokenIcon />,
			isActive: pathname.startsWith("/admin/token-contracts") || pathname.startsWith("/admin/fp-token-contracts"),
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
			<AppBar
				position="fixed"
				elevation={3}
				sx={{
					zIndex: (theme) => theme.zIndex.drawer + 1,
					background: `linear-gradient(90deg, ${theme.palette.primary.dark} 0%, ${theme.palette.primary.main} 100%)`,
				}}
			>
				<Toolbar>
					<IconButton
						edge="start"
						color="inherit"
						aria-label="menu"
						sx={{
							mr: 2,
							background: alpha(theme.palette.common.white, 0.1),
							borderRadius: 1,
							"&:hover": {
								background: alpha(theme.palette.common.white, 0.2),
							},
						}}
					>
						<img src={logoImage} alt="Website Logo" style={{ width: 36, height: 36, display: "block" }} />
					</IconButton>
					<Typography
						variant="h6"
						sx={{
							flexGrow: 1,
							fontWeight: 600,
							letterSpacing: "0.5px",
						}}
					>
						Admin Dashboard
					</Typography>
					<UserAvatar user={user} />
				</Toolbar>
			</AppBar>
			<Drawer
				variant="permanent"
				sx={{
					width: 260,
					flexShrink: 0,
					[`& .MuiDrawer-paper`]: {
						width: 260,
						boxSizing: "border-box",
						boxShadow: "2px 0 10px rgba(0,0,0,0.05)",
						borderRight: `1px solid ${theme.palette.divider}`,
					},
				}}
			>
				<Toolbar />
				<Box sx={{ overflow: "auto", py: 2 }}>
					<List sx={{ px: 1 }}>
						{navItems.map((item) => (
							<ListItem
								button
								key={item.url}
								component={Link}
								to={item.url}
								selected={item.isActive}
								sx={{
									borderRadius: 1,
									mb: 0.5,
									position: "relative",
									transition: "all 0.2s",
									"&.Mui-selected": {
										bgcolor: alpha(theme.palette.primary.main, 0.1),
										color: theme.palette.primary.main,
										"&::before": {
											content: '""',
											position: "absolute",
											left: 0,
											top: 0,
											bottom: 0,
											width: "4px",
											bgcolor: theme.palette.primary.main,
											borderRadius: "0 4px 4px 0",
										},
										"& .MuiListItemIcon-root": {
											color: theme.palette.primary.main,
										},
									},
									"&:hover": {
										bgcolor: alpha(theme.palette.primary.main, 0.05),
									},
								}}
							>
								<ListItemIcon
									sx={{
										minWidth: 40,
										color: item.isActive ? theme.palette.primary.main : alpha(theme.palette.text.primary, 0.7),
									}}
								>
									{item.icon}
								</ListItemIcon>
								<ListItemText
									primary={item.name}
									primaryTypographyProps={{
										fontWeight: item.isActive ? 600 : 400,
										fontSize: "0.9rem",
									}}
								/>
							</ListItem>
						))}
						<Divider sx={{ my: 2 }} />
						<ListItem
							button
							onClick={logout}
							sx={{
								borderRadius: 1,
								mb: 0.5,
								color: theme.palette.error.main,
								"&:hover": {
									bgcolor: alpha(theme.palette.error.main, 0.05),
								},
							}}
						>
							<ListItemIcon sx={{ minWidth: 40, color: theme.palette.error.main }}>
								<LogoutIcon />
							</ListItemIcon>
							<ListItemText primary="Logout" primaryTypographyProps={{ fontWeight: 500, fontSize: "0.9rem" }} />
						</ListItem>
					</List>
				</Box>
			</Drawer>
			<Box
				component="main"
				sx={{
					flexGrow: 1,
					p: 3,
					backgroundColor: alpha(theme.palette.background.default, 0.5),
					minHeight: "100vh",
				}}
			>
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
						<Route
							path="projects/token-holders/balance-updates/:contract/:token_id/:holder"
							element={<TokenHolderBalanceUpdateList />}
						/>
						<Route path="token-contracts" element={<TokenContractsList />} />
						<Route path="token-contracts/:contract_index/*" element={<TokenContractDetails />} />
						<Route path="fp-token-contracts/:contract_index/*" element={<ForestTokenContractDetails />} />
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
