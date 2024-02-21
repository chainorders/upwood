import {
	BrowserRouter as Router,
	Route,
	Routes,
	Navigate,
} from "react-router-dom";
import { Box } from "@mui/material";
import ConcordiumNodeClientProvider from "./components/NodeClientProvider";
import MarketPage from "./components/market/MarketPage";
import ContractsApiProvider from "./components/ContractsApiProvider";
import ContractsPage from "./components/contracts/ContractsPage";

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
						<Box pb="70px">
							<Routes>
								<Route path="contracts/*" Component={ContractsPage} />
								<Route
									path="market/:index/:subIndex/*"
									Component={MarketPage}
								/>
								<Route path="*" element={<Navigate to="contracts" replace />} />
							</Routes>
						</Box>
						<Footer />
					</Box>
				</Router>
			</ContractsApiProvider>
		</ConcordiumNodeClientProvider>
	);
}

export default Layout;
