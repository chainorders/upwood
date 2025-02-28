import { Navigate, Route, Routes, useLocation } from "react-router";
import Header, { NavItem } from "./components/Header.tsx";
import navActiveProjectNormal from "./assets/nav-active-project-normal.svg";
import navActiveProjectWhite from "./assets/nav-active-project-white.svg";
import navContractsNormal from "./assets/nav-contracts-normal.svg";
import navContractsWhite from "./assets/nav-contracts-white.svg";
import navFundedProjectNormal from "./assets/nav-funded-project-normal.svg";
import navFundedProjectWhite from "./assets/nav-funded-project-white.svg";
import navBondNormal from "./assets/nav-bond-normal.svg";
import navBondWhite from "./assets/nav-bond-white.svg";
import navInvestmentPortfolioNormal from "./assets/nav-investment-portfolio-normal.svg";
import navInvestmentPortfolioWhite from "./assets/nav-investment-portfolio-white.svg";
import navNewsUpdatesNormal from "./assets/nav-news-updates-normal.svg";
import navNewsUpdatesWhite from "./assets/nav-news-updates-white.svg";
import navSettingsNormal from "./assets/nav-settings-normal.svg";
import navSettingsWhite from "./assets/nav-settings-white.svg";
import navSupportNormal from "./assets/nav-support-normal.svg";
import navSupportWhite from "./assets/nav-support-white.svg";
import navWalletManagementNormal from "./assets/nav-wallet-management-normal.svg";
import navWalletManagementWhite from "./assets/nav-wallet-management-white.svg";
import { User } from "./lib/user.ts";
import { lazy, Suspense } from "react";
import { ForestProjectState } from "./apiClient/index.ts";

const ActiveForestProjectsList = lazy(() => import("./pages/ActiveForestProjectsList.tsx"));
const InvestmentPortfolio = lazy(() => import("./pages/InvestmentPortfolio.tsx"));
const Wallet = lazy(() => import("./pages/Wallet.tsx"));
const News = lazy(() => import("./pages/News.tsx"));
const NewsDetails = lazy(() => import("./pages/NewsDetails.tsx"));
const FundedForestProjectsList = lazy(() => import("./pages/FundedForestProjectsList.tsx"));
const Contracts = lazy(() => import("./pages/Contracts.tsx"));
const ContractsDetails = lazy(() => import("./pages/ContractsDetails.tsx"));
const Support = lazy(() => import("./pages/Support.tsx"));
const Settings = lazy(() => import("./pages/Settings.tsx"));
const BondForestProjectsList = lazy(() => import("./pages/BondForestProjectsList.tsx"));
const ForestProjectDetails = lazy(() => import("./pages/ForestProjectDetails.tsx"));

export default function UserApp({ user, logout }: { user?: User; logout: () => void }) {
	const location = useLocation();
	const pathname = location.pathname;
	const navItems: NavItem[] = [
		{
			name: "ACTIVE PROJECTS",
			url: "/projects/active",
			iconNormal: navActiveProjectNormal,
			iconActive: navActiveProjectWhite,
			isActive: pathname.startsWith("/projects/active") || pathname === "/",
		},
		{
			name: "FUNDED PROJECTS",
			url: "/projects/funded",
			iconNormal: navFundedProjectNormal,
			iconActive: navFundedProjectWhite,
			isActive: pathname.startsWith("/projects/funded"),
		},
		{
			name: "INVESTMENT BONDS",
			url: "/projects/bond",
			iconNormal: navBondNormal,
			iconActive: navBondWhite,
			isActive: pathname.startsWith("/projects/bond"),
		},
		{
			name: "INVESTMENT PORTFOLIO",
			url: "/portfolio",
			iconNormal: navInvestmentPortfolioNormal,
			iconActive: navInvestmentPortfolioWhite,
			isActive: pathname.startsWith("/portfolio"),
		},
		{
			name: "CONTRACTS",
			url: "/contracts",
			iconNormal: navContractsNormal,
			iconActive: navContractsWhite,
			isActive: pathname.startsWith("/contracts"),
		},
		{
			name: "WALLET MANAGEMENT",
			url: "/wallet",
			iconNormal: navWalletManagementNormal,
			iconActive: navWalletManagementWhite,
			isActive: pathname.startsWith("/wallet"),
		},
		{
			name: "NEWS & UPDATES",
			url: "/news",
			iconNormal: navNewsUpdatesNormal,
			iconActive: navNewsUpdatesWhite,
			isActive: pathname.startsWith("/news"),
		},
		{
			name: "SUPPORT",
			url: "/support",
			iconNormal: navSupportNormal,
			iconActive: navSupportWhite,
			isActive: pathname.startsWith("/support"),
		},
		{
			name: "SETTINGS",
			url: "/settings",
			iconNormal: navSettingsNormal,
			iconActive: navSettingsWhite,
			isActive: pathname.startsWith("/settings"),
		},
	];

	return user ? (
		<div className="auth-layout">
			<Header navItems={navItems} logout={logout} />
			<Suspense fallback={<div>Loading...</div>}>
				<Routes>
					<Route index path="/" element={<ActiveForestProjectsList user={user} />} />
					<Route path="projects/active" element={<ActiveForestProjectsList user={user} />} />
					<Route
						path="projects/active/:id"
						element={<ForestProjectDetails user={user} source={ForestProjectState.ACTIVE} />}
					/>
					<Route path="projects/funded" element={<FundedForestProjectsList user={user} />} />
					<Route
						path="projects/funded/:id"
						element={<ForestProjectDetails user={user} source={ForestProjectState.FUNDED} />}
					/>
					<Route path="projects/bond" element={<BondForestProjectsList user={user} />} />
					<Route path="projects/bond/:id" element={<ForestProjectDetails user={user} source={ForestProjectState.BOND} />} />
					<Route path="portfolio" element={<InvestmentPortfolio user={user} />} />
					<Route path="wallet" element={<Wallet user={user} />} />
					<Route path="news" element={<News user={user} />} />
					<Route path="news/:id" element={<NewsDetails user={user} />} />
					<Route path="contracts" element={<Contracts user={user} />} />
					<Route path="contracts/:id" element={<ContractsDetails user={user} />} />
					<Route path="support" element={<Support user={user} />} />
					<Route path="settings" element={<Settings user={user} />} />
					<Route path="*" element={<Navigate to="/" />} />
				</Routes>
			</Suspense>
		</div>
	) : (
		<Navigate to="/login" replace state={{ from: location }} />
	);
}
