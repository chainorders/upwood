import { Navigate, Outlet, useLocation } from "react-router";
import Header, { NavItem } from "./components/Header";
import navActiveProjectNormal from "./assets/nav-active-project-normal.svg";
import navActiveProjectWhite from "./assets/nav-active-project-white.svg";
import navContractsNormal from "./assets/nav-contracts-normal.svg";
import navContractsWhite from "./assets/nav-contracts-white.svg";
import navFundedProjectNormal from "./assets/nav-funded-project-normal.svg";
import navFundedProjectWhite from "./assets/nav-funded-project-white.svg";
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
import { User } from "./lib/user";

export default function AuthLayout(props: { user?: User; logout: () => void }) {
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

	return props.user ? (
		<div className="auth-layout">
			<Header navItems={navItems} logout={props.logout} />
			<Outlet context={{ user: props.user! }} />
		</div>
	) : (
		<Navigate to="/login" replace state={{ from: location }} />
	);
}
