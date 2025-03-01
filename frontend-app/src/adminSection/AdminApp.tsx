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
import { lazy, Suspense } from "react";

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
							<Route path="create" element={<ProjectCreate />} />
							<Route path=":id">
								<Route path="update" element={<ProjectUpdate />} />
								<Route path="details" element={<ProjectDetails />} />
								<Route path="contract">
									<Route path="create" element={<ProjectContractCreate user={user} />} />
									<Route path=":contract_address">
										<Route path="details" element={<ProjectContractDetails user={user} />} />
										<Route path="update" element={<ProjectContractUpdate user={user} />} />
										<Route path="token">
											<Route path=":token_id">
												<Route path="details" element={<ProjectTokenDetails user={user} />} />
											</Route>
										</Route>
									</Route>
								</Route>
							</Route>
						</Route>
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
					</Routes>
				</Suspense>
			</Box>
		</Box>
	) : (
		<Navigate to="/login" replace state={{ from: location }} />
	);
}
