import { createTheme, alpha, ThemeOptions } from "@mui/material/styles";

// Define the theme configuration
const themeOptions: ThemeOptions = {
	palette: {
		primary: {
			main: "#1976d2",
			dark: "#115293",
			light: "#4791db",
		},
		secondary: {
			main: "#dc004e",
			dark: "#9a0036",
			light: "#e33371",
		},
		error: {
			main: "#f44336",
			dark: "#d32f2f",
			light: "#e57373",
		},
		warning: {
			main: "#ff9800",
			dark: "#f57c00",
			light: "#ffb74d",
		},
		success: {
			main: "#4caf50",
			dark: "#388e3c",
			light: "#81c784",
		},
		info: {
			main: "#2196f3",
			dark: "#0d47a1",
			light: "#64b5f6",
		},
		background: {
			default: "#f5f5f5",
			paper: "#ffffff",
		},
	},
	typography: {
		fontFamily: '"Roboto", "Helvetica", "Arial", sans-serif',
		h4: {
			fontWeight: 500,
		},
		h6: {
			fontWeight: 600,
			letterSpacing: "0.5px",
		},
		subtitle1: {
			fontWeight: 500,
		},
		button: {
			textTransform: "none",
			fontWeight: 500,
		},
	},
	shape: {
		borderRadius: 4,
	},
	components: {
		MuiAppBar: {
			defaultProps: {
				elevation: 3,
			},
			styleOverrides: {
				root: ({ theme }) => ({
					background: `linear-gradient(90deg, ${theme.palette.primary.dark} 0%, ${theme.palette.primary.main} 100%)`,
				}),
			},
		},
		MuiDrawer: {
			styleOverrides: {
				paper: ({ theme }) => ({
					width: 260,
					boxSizing: "border-box",
					boxShadow: "2px 0 10px rgba(0,0,0,0.05)",
					borderRight: `1px solid ${theme.palette.divider}`,
				}),
			},
		},
		MuiPaper: {
			defaultProps: {
				elevation: 2,
			},
			styleOverrides: {
				root: {
					borderRadius: 8,
				},
				elevation1: {
					boxShadow: "0px 2px 4px rgba(0,0,0,0.05)",
				},
				elevation2: {
					boxShadow: "0px 3px 6px rgba(0,0,0,0.08)",
				},
				elevation3: {
					boxShadow: "0px 5px 15px rgba(0,0,0,0.1)",
				},
			},
		},
		MuiButton: {
			styleOverrides: {
				root: {
					borderRadius: 8,
					textTransform: "none",
					fontSize: "0.9rem",
				},
				contained: () => ({
					boxShadow: "0 3px 5px rgba(0,0,0,0.1)",
					"&:hover": {
						boxShadow: "0 5px 8px rgba(0,0,0,0.15)",
					},
				}),
				outlined: ({ theme }) => ({
					borderWidth: "1px",
					"&:hover": {
						backgroundColor: alpha(theme.palette.primary.main, 0.04),
					},
				}),
				startIcon: {
					marginRight: 6,
				},
			},
		},
		MuiIconButton: {
			styleOverrides: {
				root: ({ theme }) => ({
					"&:hover": {
						backgroundColor: alpha(theme.palette.action.active, 0.12),
					},
				}),
			},
		},
		MuiListItem: {
			styleOverrides: {
				root: ({ theme }) => ({
					borderRadius: 4,
					marginBottom: 4,
					transition: "all 0.2s",
					"&.Mui-selected": {
						backgroundColor: alpha(theme.palette.primary.main, 0.1),
						color: theme.palette.primary.main,
						"&::before": {
							content: '""',
							position: "absolute",
							left: 0,
							top: 0,
							bottom: 0,
							width: "4px",
							backgroundColor: theme.palette.primary.main,
							borderRadius: "0 4px 4px 0",
						},
						"& .MuiListItemIcon-root": {
							color: theme.palette.primary.main,
						},
					},
					"&:hover": {
						backgroundColor: alpha(theme.palette.primary.main, 0.05),
					},
				}),
			},
		},
		MuiListItemIcon: {
			styleOverrides: {
				root: {
					minWidth: 40,
				},
			},
		},
		MuiListItemText: {
			styleOverrides: {
				primary: {
					fontSize: "0.9rem",
				},
			},
		},
		MuiTable: {
			styleOverrides: {
				root: {
					borderCollapse: "separate",
					borderSpacing: 0,
				},
			},
		},
		MuiTableHead: {
			styleOverrides: {
				root: ({ theme }) => ({
					"& .MuiTableRow-root": {
						backgroundColor: alpha(theme.palette.primary.main, 0.08),
					},
					"& .MuiTableCell-root": {
						fontWeight: "bold",
						color: theme.palette.text.primary,
					},
				}),
			},
		},
		MuiTableRow: {
			styleOverrides: {
				root: ({ theme }) => ({
					"&:hover": {
						backgroundColor: alpha(theme.palette.primary.main, 0.04),
					},
				}),
			},
		},
		MuiTableCell: {
			styleOverrides: {
				root: {
					padding: "12px 16px",
					borderBottom: "1px solid rgba(224, 224, 224, 1)",
				},
				head: {
					fontWeight: "bold",
				},
			},
		},
		MuiTablePagination: {
			styleOverrides: {
				root: {
					overflow: "visible",
				},
			},
		},
		MuiChip: {
			styleOverrides: {
				root: {
					fontWeight: 500,
				},
				sizeSmall: {
					height: 24,
					fontSize: "0.75rem",
				},
			},
		},
		MuiPagination: {
			styleOverrides: {
				root: {
					"& .MuiPaginationItem-root": {
						borderRadius: 4,
					},
				},
			},
		},
		MuiAccordion: {
			styleOverrides: {
				root: ({ theme }) => ({
					borderRadius: 8,
					overflow: "hidden",
					"&:before": {
						display: "none",
					},
					"&.Mui-expanded": {
						margin: theme.spacing(2, 0),
					},
				}),
			},
		},
		MuiAccordionSummary: {
			styleOverrides: {
				root: ({ theme }) => ({
					borderBottom: `1px solid ${theme.palette.divider}`,
					minHeight: 56,
					"&.Mui-expanded": {
						minHeight: 56,
						backgroundColor: alpha(theme.palette.primary.main, 0.04),
					},
				}),
				content: {
					"&.Mui-expanded": {
						margin: "12px 0",
					},
				},
			},
		},
		MuiDivider: {
			styleOverrides: {
				root: {
					margin: "16px 0",
				},
			},
		},
		MuiBreadcrumbs: {
			styleOverrides: {
				root: {
					marginBottom: 16,
				},
				li: {
					fontSize: "0.875rem",
				},
			},
		},
	},
};

// Create and export the theme
const theme = createTheme(themeOptions);

export default theme;
