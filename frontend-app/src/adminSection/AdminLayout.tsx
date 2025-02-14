import { Link, Navigate, Outlet, useLocation } from "react-router";
import { AppBar, Toolbar, Typography, Box, Drawer, List, ListItem, ListItemText, ListItemIcon, IconButton } from "@mui/material";
import { User } from "../lib/user";
import MenuIcon from "@mui/icons-material/Menu";
import navActiveProjectNormal from "../assets/nav-active-project-normal.svg";
import navActiveProjectWhite from "../assets/nav-active-project-white.svg";

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

export default function AdminLayout(props: { user?: User; logout: () => void }) {
	const location = useLocation();
	const pathname = location.pathname;
	const navItems = [
		{
			name: "PROJECTS",
			url: "/admin/projects",
			iconNormal: navActiveProjectNormal,
			iconActive: navActiveProjectWhite,
			isActive: pathname.startsWith("/admin/projects"),
		},
		{
			name: "Users",
			url: "/admin/users",
			iconNormal: navActiveProjectNormal,
			iconActive: navActiveProjectWhite,
			isActive: pathname.startsWith("/admin/users"),
		},
	];

	return props.user && props.user.isAdmin ? (
		<Box sx={{ display: "flex" }}>
			<AppBar position="fixed">
				<Toolbar>
					<IconButton edge="start" color="inherit" aria-label="menu" sx={{ mr: 2 }}>
						<MenuIcon />
					</IconButton>
					<Typography variant="h6" sx={{ flexGrow: 1 }}>
						Admin
					</Typography>
					<UserAvatar user={props.user} />
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
								<ListItemIcon>
									<img src={item.isActive ? item.iconActive : item.iconNormal} alt={item.name} />
								</ListItemIcon>
								<ListItemText primary={item.name} />
							</ListItem>
						))}
					</List>
				</Box>
			</Drawer>
			<Box component="main" sx={{ flexGrow: 1, p: 3 }}>
				<Toolbar />
				<Outlet context={{ user: props.user! }} />
			</Box>
		</Box>
	) : (
		<Navigate to="/login" replace state={{ from: location }} />
	);
}
