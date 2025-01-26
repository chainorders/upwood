import { Route, Routes, useNavigate } from "react-router";
import AuthLayout from "./AuthLayout.tsx";
import ActiveForestProjectsList from "./pages/ActiveForestProjectsList.tsx";
import ActiveForestProjectDetails from "./pages/ActiveForestProjectDetails.tsx";
import InvestmentPortfolio from "./pages/InvestmentPortfolio.tsx";
import Login from "./pages/Login.tsx";
import { LoginRes } from "./apiClient/index.ts";
import { useState } from "react";
import Wallet from "./pages/Wallet.tsx";
import News from "./pages/News.tsx";
import NewsDetails from "./pages/NewsDetails.tsx";
import ForgotPassword from "./pages/ForgotPassword.tsx";
import Register from "./pages/Register.tsx";
import Support from "./pages/Support.tsx";
import NotFound from "./pages/NotFound.tsx";
import UnAuthLayout from "./UnAuthLayout.tsx";
import RegisterDownloadWallet from "./pages/RegisterDownloadWallet.tsx";
import LoginInviteSuccess from "./pages/LoginInviteSuccess.tsx";
export default function App() {
	const [user, setUser] = useState<LoginRes>();
	const navigate = useNavigate();
	const login = (user: LoginRes) => {
		setUser(user);
		sessionStorage.setItem("token", user.id_token);
		navigate("/projects/active");
	};
	const logout = () => {
		setUser(undefined);
		sessionStorage.removeItem("token");
		navigate("/login");
	};

	return (
		<Routes>
			<Route element={<UnAuthLayout />}>
				<Route path="/login/invite-success" element={<LoginInviteSuccess />} />
				<Route path="/login/:affiliateAccountAddress?" element={<Login setUser={login} />} />
				<Route path="/forgot-password" element={<ForgotPassword />} />
				<Route path="/register" element={<Register />} />
				<Route path="/register/download-wallet" element={<RegisterDownloadWallet />} />
			</Route>
			<Route element={<AuthLayout user={user} logout={logout} />}>
				<Route index path="/" element={<ActiveForestProjectsList />} />
				<Route path="projects/active" element={<ActiveForestProjectsList />} />
				<Route path="projects/active/:id" element={<ActiveForestProjectDetails />} />
				<Route path="portfolio" element={<InvestmentPortfolio />} />
				<Route path="wallet" element={<Wallet />} />
				<Route path="news" element={<News />} />
				<Route path="news/:id" element={<NewsDetails />} />
				<Route path="support" element={<Support />} />
				<Route path="*" element={<NotFound />} />
			</Route>
		</Routes>
	);
}
