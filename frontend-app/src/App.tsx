import { Route, Routes, useLocation, useNavigate } from "react-router";
import { lazy, Suspense, useCallback, useEffect, useState } from "react";
import { OpenAPI, SystemContractsConfigApiModel, UserService } from "./apiClient/index.ts";
import { User } from "./lib/user.ts";

const UserApp = lazy(() => import("./UserApp.tsx"));
const UnAuthLayout = lazy(() => import("./UnAuthLayout.tsx"));
const Login = lazy(() => import("./pages/Login.tsx"));
const ForgotPassword = lazy(() => import("./pages/ForgotPassword.tsx"));
const Register = lazy(() => import("./pages/Register.tsx"));
const RegisterDownloadWallet = lazy(() => import("./pages/RegisterDownloadWallet.tsx"));
const LoginInviteSuccess = lazy(() => import("./pages/LoginInviteSuccess.tsx"));
const AdminApp = lazy(() => import("./adminSection/AdminApp.tsx"));

export default function App() {
	OpenAPI.BASE = import.meta.env.VITE_API_BASE_URL;
	const location = useLocation();

	const [user, setUser] = useState<User>();
	const navigate = useNavigate();
	const login = useCallback(
		(user: User) => {
			setUser(user);
			OpenAPI.TOKEN = user.idToken;
			console.log("login", location.state?.from);
			navigate(location.state?.from ? location.state.from : "/projects/active");
		},
		[location.state?.from, navigate],
	);
	const logout = () => {
		user?.cognitoUser.signOut(() => {
			OpenAPI.TOKEN = undefined;
			setUser(undefined);
			navigate("/login");
		});
	};

	return (
		<Suspense fallback={<div>Loading...</div>}>
			<Routes>
				<Route element={<UnAuthLayout />}>
					<Route path="/login/invite-success" element={<LoginInviteSuccess />} />
					<Route path="/login/:affiliateAccountAddress?" element={<Login setUser={login} />} />
					<Route path="/forgot-password" element={<ForgotPassword />} />
					<Route path="/register" element={<Register />} />
					<Route path="/register/download-wallet" element={<RegisterDownloadWallet />} />
				</Route>
				<Route path="admin/*" element={<AdminApp user={user} logout={logout} />} />
				<Route path="*" element={<UserApp user={user} logout={logout} />} />
			</Routes>
		</Suspense>
	);
}
