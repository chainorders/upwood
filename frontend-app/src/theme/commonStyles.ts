import { Theme, alpha } from "@mui/material/styles";

/**
 * Common styles that can be reused across components
 */
const commonStyles = {
	// Card and Paper styles
	cardStyles: (theme: Theme) => ({
		borderRadius: 2,
		boxShadow: "0 3px 10px rgba(0,0,0,0.08)",
		padding: theme.spacing(3),
		height: "100%",
	}),

	// Section header styles
	sectionHeader: (theme: Theme) => ({
		display: "flex",
		alignItems: "center",
		marginBottom: theme.spacing(3),
		"& .MuiSvgIcon-root": {
			marginRight: theme.spacing(1),
			color: theme.palette.primary.main,
		},
	}),

	// Action button container (usually at the top-right of sections)
	actionContainer: (theme: Theme) => ({
		display: "flex",
		justifyContent: "flex-end",
		gap: theme.spacing(1),
		marginBottom: theme.spacing(2),
	}),

	// Table container styles
	tableContainer: (theme: Theme) => ({
		borderRadius: 2,
		overflow: "hidden",
		marginBottom: theme.spacing(3),
		"& .MuiTableRow-root:hover": {
			backgroundColor: alpha(theme.palette.primary.main, 0.04),
		},
	}),

	// Avatar styles for user display
	userAvatar: (theme: Theme) => ({
		width: 40,
		height: 40,
		borderRadius: "50%",
		background: `linear-gradient(135deg, ${theme.palette.primary.main} 0%, ${theme.palette.primary.dark} 100%)`,
		display: "flex",
		alignItems: "center",
		justifyContent: "center",
		color: "white",
		marginRight: theme.spacing(1.5),
		boxShadow: "0 3px 5px rgba(0,0,0,0.2)",
		fontWeight: "bold",
	}),

	// Action buttons in tables
	tableActionButton: (theme: Theme, color: string = theme.palette.primary.main) => ({
		color: color,
		bgcolor: alpha(color, 0.1),
		"&:hover": {
			bgcolor: alpha(color, 0.2),
		},
	}),

	// Filter section styles
	filterContainer: (theme: Theme) => ({
		padding: theme.spacing(2),
		marginBottom: theme.spacing(3),
		display: "flex",
		justifyContent: "space-between",
		alignItems: "center",
		flexWrap: "wrap",
		gap: theme.spacing(2),
	}),

	// Breadcrumb link styles
	breadcrumbLink: (theme: Theme) => ({
		display: "flex",
		alignItems: "center",
		textDecoration: "none",
		color: theme.palette.text.secondary,
	}),

	// Form container styles
	formContainer: (theme: Theme) => ({
		backgroundColor: theme.palette.background.paper,
		borderRadius: theme.shape.borderRadius * 2,
		padding: theme.spacing(4),
		boxShadow: theme.shadows[1],
		width: "100%",
		marginBottom: theme.spacing(4),
	}),

	// Form section styles
	formSection: (theme: Theme) => ({
		marginBottom: theme.spacing(4),
		"&:last-child": {
			marginBottom: 0,
		},
	}),

	// Form section header
	formSectionHeader: (theme: Theme) => ({
		display: "flex",
		alignItems: "center",
		marginBottom: theme.spacing(3),
		"& .MuiSvgIcon-root": {
			marginRight: theme.spacing(1),
			color: theme.palette.primary.main,
		},
		"& .MuiTypography-root": {
			fontWeight: 600,
			color: theme.palette.text.primary,
		},
	}),

	// Form divider
	formDivider: (theme: Theme) => ({
		margin: theme.spacing(3, 0),
		backgroundColor: alpha(theme.palette.divider, 0.7),
	}),

	// Form field wrapper
	formField: (theme: Theme) => ({
		marginBottom: theme.spacing(2.5),
	}),

	// Form action buttons container
	formActions: (theme: Theme) => ({
		display: "flex",
		justifyContent: "flex-end",
		gap: theme.spacing(2),
		marginTop: theme.spacing(4),
	}),

	// Form submit button
	formSubmitButton: () => ({
		minWidth: 120,
		height: 48,
		fontWeight: 600,
	}),

	// Form image uploader container
	imageUploaderContainer: (theme: Theme) => ({
		border: `1px dashed ${alpha(theme.palette.primary.main, 0.5)}`,
		borderRadius: theme.shape.borderRadius,
		padding: theme.spacing(2),
		marginBottom: theme.spacing(3),
		backgroundColor: alpha(theme.palette.primary.main, 0.03),
	}),

	// Status chip styles based on state
	statusChip: (theme: Theme, status: string) => {
		const statusMap: Record<string, { color: string; backgroundColor: string }> = {
			active: { color: theme.palette.success.main, backgroundColor: alpha(theme.palette.success.main, 0.1) },
			pending: { color: theme.palette.warning.main, backgroundColor: alpha(theme.palette.warning.main, 0.1) },
			archived: { color: theme.palette.error.main, backgroundColor: alpha(theme.palette.error.main, 0.1) },
			draft: { color: theme.palette.text.secondary, backgroundColor: alpha(theme.palette.text.secondary, 0.1) },
			funded: { color: theme.palette.primary.main, backgroundColor: alpha(theme.palette.primary.main, 0.1) },
			bond: { color: theme.palette.info.main, backgroundColor: alpha(theme.palette.info.main, 0.1) },
		};

		const statusStyle = statusMap[status.toLowerCase()] || statusMap.draft;

		return {
			backgroundColor: statusStyle.backgroundColor,
			color: statusStyle.color,
			fontWeight: 500,
			borderRadius: "4px",
			textTransform: "capitalize",
		};
	},
};

export default commonStyles;
