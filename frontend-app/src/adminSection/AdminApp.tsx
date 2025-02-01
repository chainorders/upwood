import { Route, Routes, Outlet, Navigate, useLocation } from "react-router-dom";
import AdminLayout from "./AdminLayout";
import { LoginRes } from "../apiClient";
import ProjectList from "./pages/ProjectList";
import ProjectCreate from "./pages/ProjectCreate";
import NotFound from "../pages/NotFound";
import Header, { NavItem } from "../components/Header";
import navActiveProjectNormal from "../assets/nav-active-project-normal.svg";
import navActiveProjectWhite from "../assets/nav-active-project-white.svg";

export default function AdminApp(props: { user?: LoginRes; logout: () => void }) {
	const { user, logout } = props;
	const location = useLocation();
	const pathname = location.pathname;
	const navItems: NavItem[] = [
		{
			name: "PROJECTS",
			url: "/admin/projects",
			iconNormal: navActiveProjectNormal,
			iconActive: navActiveProjectWhite,
			isActive: pathname.startsWith("/admin/projects"),
		},
	];

	return props.user && props.user.user.is_admin ? (
		<>
			<Header navItems={navItems} logout={props.logout} />
			<Routes>
				<Route path="/admin/projects" element={<AdminLayout user={user} logout={logout} />}>
					<Route index path="" element={<ProjectList />} />
					<Route path="list" element={<ProjectList />} />
					<Route path="create" element={<ProjectCreate />} />
					<Route path="*" element={<NotFound />} />
				</Route>
				<Route path="*" element={<NotFound />} />
			</Routes>
			<Outlet context={{ user: props.user! }} />
		</>
	) : (
		<Navigate to="/login" replace state={{ from: location }} />
	);
}
