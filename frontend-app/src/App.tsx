import { Navigate, Route, Routes, useLocation, useNavigate } from "react-router";
import AuthLayout from "./AuthLayout.tsx";
import ActiveForestProjectsList from "./pages/ActiveForestProjectsList.tsx";
import InvestmentPortfolio from "./pages/InvestmentPortfolio.tsx";
import Login from "./pages/Login.tsx";
import { ForestProjectState, OpenAPI } from "./apiClient/index.ts";
import { useCallback, useState } from "react";
import Wallet from "./pages/Wallet.tsx";
import News from "./pages/News.tsx";
import NewsDetails from "./pages/NewsDetails.tsx";
import ForgotPassword from "./pages/ForgotPassword.tsx";
import Register from "./pages/Register.tsx";
import Support from "./pages/Support.tsx";
import UnAuthLayout from "./UnAuthLayout.tsx";
import RegisterDownloadWallet from "./pages/RegisterDownloadWallet.tsx";
import LoginInviteSuccess from "./pages/LoginInviteSuccess.tsx";
import FundedForestProjectsList from "./pages/FundedForestProjectsList.tsx";
import Contracts from "./pages/Contracts.tsx";
import ContractsDetails from "./pages/ContractsDetails.tsx";
import Settings from "./pages/Settings.tsx";
import AdminLayout from "./adminSection/AdminLayout.tsx";
import ProjectList from "./adminSection/pages/ProjectList.tsx";
import ProjectCreate from "./adminSection/pages/ProjectCreate.tsx";
import ProjectUpdate from "./adminSection/pages/ProjectUpdate.tsx";
import ProjectDetails from "./adminSection/pages/ProjectDetails.tsx";
import ProjectContractCreate from "./adminSection/pages/ProjectContractCreate.tsx";
import { User } from "./lib/user.ts";
import ProjectContractDetails from "./adminSection/pages/ProjectContractDetails.tsx";
import ProjectContractUpdate from "./adminSection/pages/ProjectContractUpdate.tsx";
import ProjectTokenDetails from "./adminSection/pages/ProjectTokenDetails.tsx";
import UserList from "./adminSection/pages/UserList.tsx";
import UserInvitations from "./adminSection/pages/UserInvitations.tsx";
import BondForestProjectsList from "./pages/BondForestProjectsList.tsx";
import ForestProjectDetails from "./pages/ForestProjectDetails.tsx";

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
				<Route path="projects/active/:id" element={<ForestProjectDetails source={ForestProjectState.ACTIVE} />} />
				<Route path="projects/funded" element={<FundedForestProjectsList />} />
				<Route path="projects/funded/:id" element={<ForestProjectDetails source={ForestProjectState.FUNDED} />} />
				<Route path="projects/bond" element={<BondForestProjectsList />} />
				<Route path="projects/bond/:id" element={<ForestProjectDetails source={ForestProjectState.BOND} />} />
				<Route path="portfolio" element={<InvestmentPortfolio />} />
				<Route path="wallet" element={<Wallet />} />
				<Route path="news" element={<News />} />
				<Route path="news/:id" element={<NewsDetails />} />
				<Route path="contracts" element={<Contracts />} />
				<Route path="contracts/:id" element={<ContractsDetails />} />
				<Route path="support" element={<Support />} />
				<Route path="settings" element={<Settings />} />
				<Route path="*" element={<Navigate to="/" />} />
			</Route>
			<Route path="admin/*" element={<AdminLayout user={user} logout={logout} />}>
				<Route path="projects/*">
					<Route index element={<ProjectList />} />
					<Route path="list" element={<ProjectList />} />
					<Route path="create" element={<ProjectCreate />} />
					<Route path=":id">
						<Route path="update" element={<ProjectUpdate />} />
						<Route path="details" element={<ProjectDetails />} />
						<Route path="contract">
							<Route path="create" element={<ProjectContractCreate />} />
							<Route path=":contract_address">
								<Route path="details" element={<ProjectContractDetails />} />
								<Route path="update" element={<ProjectContractUpdate />} />
								<Route path="token">
									<Route path=":token_id">
										<Route path="details" element={<ProjectTokenDetails />} />
									</Route>
								</Route>
							</Route>
						</Route>
					</Route>
				</Route>
				<Route path="users/*">
					<Route index element={<UserList />} />
					<Route path="list" element={<UserList />} />
					<Route path="invitations" element={<UserInvitations />} />
				</Route>
			</Route>
		</Routes>
	);
}
