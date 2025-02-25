import { Navigate, Route, Routes, useLocation, useNavigate } from "react-router";
import { lazy, Suspense, useCallback, useState } from "react";
import { ForestProjectState, OpenAPI } from "./apiClient/index.ts";
import { User } from "./lib/user.ts";

const AuthLayout = lazy(() => import("./AuthLayout.tsx"));
const UnAuthLayout = lazy(() => import("./UnAuthLayout.tsx"));
const ActiveForestProjectsList = lazy(() => import("./pages/ActiveForestProjectsList.tsx"));
const InvestmentPortfolio = lazy(() => import("./pages/InvestmentPortfolio.tsx"));
const Login = lazy(() => import("./pages/Login.tsx"));
const Wallet = lazy(() => import("./pages/Wallet.tsx"));
const News = lazy(() => import("./pages/News.tsx"));
const NewsDetails = lazy(() => import("./pages/NewsDetails.tsx"));
const ForgotPassword = lazy(() => import("./pages/ForgotPassword.tsx"));
const Register = lazy(() => import("./pages/Register.tsx"));
const Support = lazy(() => import("./pages/Support.tsx"));
const RegisterDownloadWallet = lazy(() => import("./pages/RegisterDownloadWallet.tsx"));
const LoginInviteSuccess = lazy(() => import("./pages/LoginInviteSuccess.tsx"));
const FundedForestProjectsList = lazy(() => import("./pages/FundedForestProjectsList.tsx"));
const Contracts = lazy(() => import("./pages/Contracts.tsx"));
const ContractsDetails = lazy(() => import("./pages/ContractsDetails.tsx"));
const Settings = lazy(() => import("./pages/Settings.tsx"));
const AdminLayout = lazy(() => import("./adminSection/AdminLayout.tsx"));
const ProjectList = lazy(() => import("./adminSection/pages/ProjectList.tsx"));
const ProjectCreate = lazy(() => import("./adminSection/pages/ProjectCreate.tsx"));
const ProjectUpdate = lazy(() => import("./adminSection/pages/ProjectUpdate.tsx"));
const ProjectDetails = lazy(() => import("./adminSection/pages/ProjectDetails.tsx"));
const ProjectContractCreate = lazy(() => import("./adminSection/pages/ProjectContractCreate.tsx"));
const ProjectContractDetails = lazy(() => import("./adminSection/pages/ProjectContractDetails.tsx"));
const ProjectContractUpdate = lazy(() => import("./adminSection/pages/ProjectContractUpdate.tsx"));
const ProjectTokenDetails = lazy(() => import("./adminSection/pages/ProjectTokenDetails.tsx"));
const UserList = lazy(() => import("./adminSection/pages/UserList.tsx"));
const UserInvitations = lazy(() => import("./adminSection/pages/UserInvitations.tsx"));
const BondForestProjectsList = lazy(() => import("./pages/BondForestProjectsList.tsx"));
const ForestProjectDetails = lazy(() => import("./pages/ForestProjectDetails.tsx"));

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
		</Suspense>
	);
}
