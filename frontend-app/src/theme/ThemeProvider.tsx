import React from "react";
import { ThemeProvider as MuiThemeProvider, CssBaseline } from "@mui/material";
import theme from "./index";

interface ThemeProviderProps {
	children: React.ReactNode;
}

/**
 * ThemeProvider component that wraps the application with the custom theme
 */
const ThemeProvider = ({ children }: ThemeProviderProps) => {
	return (
		<MuiThemeProvider theme={theme}>
			<CssBaseline />
			{children}
		</MuiThemeProvider>
	);
};

export default ThemeProvider;
