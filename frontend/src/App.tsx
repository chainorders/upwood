import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import { AppBar, Box, Toolbar, Typography } from "@mui/material";
import ContractsPage from "./components/contracts/ContractsPage";
import ConcordiumWalletProvider from "./components/WalletProvider";
import ConcordiumNodeClientProvider from "./components/NodeClientProvider";
import ErrorDisplay from "./components/common/ErrorDisplay";
import MarketPage from "./components/market/MarketPage";
import ContractsApiProvider from "./components/ContractsApiProvider";
import VerifierPage from "./components/verifier/VerifierPage";

// Header component
function Header() {
	return (
		<AppBar
			position="static"
			sx={{ marginBottom: 2, zIndex: (theme) => theme.zIndex.drawer + 1 }}
		>
			<Toolbar sx={{ display: "flex", justifyContent: "space-between" }}>
				<Typography variant="h6">Concordium RWA</Typography>
			</Toolbar>
		</AppBar>
	);
}

function Footer() {
	return (
		<Box
			sx={{
				width: "100%", // full width
				height: "60px", // fixed height
				backgroundColor: "grey.900", // dark background color
				color: "white", // white text color
				position: "fixed", // fixed position
				bottom: 0, // stick to bottom
				display: "flex", // use flexbox for centering content
				alignItems: "center", // center content vertically
				justifyContent: "center", // center content horizontally
				padding: "0 16px", // horizontal padding
				zIndex: (theme) => theme.zIndex.drawer + 1,
			}}
		>
			{/* Footer content goes here */}
		</Box>
	);
}

// Main layout component
function Layout() {
	return (
		<ConcordiumNodeClientProvider>
			<ConcordiumWalletProvider>
				<ContractsApiProvider>
					<Router>
						<Box
							sx={{
								display: "flex",
								flexDirection: "column",
								minHeight: "100vh",
								width: "100%",
							}}
						>
							<Header />
							<Box pb="70px">
								<Routes>
									<Route path="contracts/*" Component={ContractsPage} />
									<Route
										path="market/:index/:subIndex/*"
										Component={MarketPage}
									/>
									<Route path="verifier" Component={VerifierPage} />
									<Route path="*" element={<ErrorDisplay text="Not Found" />} />
								</Routes>
							</Box>
							<Footer />
						</Box>
					</Router>
				</ContractsApiProvider>
			</ConcordiumWalletProvider>
		</ConcordiumNodeClientProvider>
	);
}

export default Layout;
