import { useTheme } from "@mui/material/styles";
import commonStyles from "./commonStyles";

/**
 * Hook to use common styles with the current theme
 */
const useCommonStyles = () => {
	const theme = useTheme();

	return {
		cardStyles: commonStyles.cardStyles(theme),
		sectionHeader: commonStyles.sectionHeader(theme),
		actionContainer: commonStyles.actionContainer(theme),
		tableContainer: commonStyles.tableContainer(theme),
		userAvatar: commonStyles.userAvatar(theme),
		tableActionButton: (color?: string) => commonStyles.tableActionButton(theme, color),
		filterContainer: commonStyles.filterContainer(theme),
		breadcrumbLink: commonStyles.breadcrumbLink(theme),

		// Form styles
		formContainer: commonStyles.formContainer(theme),
		formSection: commonStyles.formSection(theme),
		formSectionHeader: commonStyles.formSectionHeader(theme),
		formDivider: commonStyles.formDivider(theme),
		formField: commonStyles.formField(theme),
		formActions: commonStyles.formActions(theme),
		formSubmitButton: commonStyles.formSubmitButton(),
		imageUploaderContainer: commonStyles.imageUploaderContainer(theme),

		// Dialog form styles - optimized for forms inside dialogs
		dialogFormContainer: {
			padding: 0, // Remove padding as DialogContent already has padding
			boxShadow: "none", // Remove shadow as Dialog already has one
			backgroundColor: "transparent", // Transparent background
		},
		dialogFormSection: {
			padding: theme.spacing(1, 0),
		},
		dialogFormField: {
			marginBottom: theme.spacing(2),
		},
		dialogFormActions: {
			marginTop: theme.spacing(2),
			marginBottom: theme.spacing(1),
		},

		// Detail page styles
		detailsContainer: {
			padding: theme.spacing(3),
			marginBottom: theme.spacing(3),
			borderRadius: theme.shape.borderRadius,
			boxShadow: theme.shadows[2],
			border: `1px solid ${theme.palette.divider}`,
		},
		detailsHeader: {
			display: "flex",
			justifyContent: "space-between",
			alignItems: "center",
			marginBottom: theme.spacing(3),
			borderBottom: `1px solid ${theme.palette.divider}`,
			paddingBottom: theme.spacing(2),
		},
		detailsTitle: {
			fontWeight: 600,
			color: theme.palette.primary.main,
		},
		detailsActions: {
			display: "flex",
			gap: theme.spacing(1),
		},
		detailsSection: {
			marginBottom: theme.spacing(3),
		},
		detailsSectionTitle: {
			fontWeight: 500,
			marginBottom: theme.spacing(2),
			color: theme.palette.text.primary,
			borderBottom: `1px solid ${theme.palette.divider}`,
			paddingBottom: theme.spacing(1),
		},
		detailsGrid: {
			marginBottom: theme.spacing(2),
		},
		detailsRow: {
			marginBottom: theme.spacing(1),
			display: "flex",
			flexDirection: "column",
			[theme.breakpoints.up("sm")]: {
				flexDirection: "row",
			},
		},
		detailsLabel: {
			fontWeight: 600,
			minWidth: "180px",
			color: theme.palette.text.secondary,
			[theme.breakpoints.up("sm")]: {
				marginRight: theme.spacing(2),
			},
		},
		detailsValue: {
			wordBreak: "break-word",
		},
		detailsAlert: {
			marginBottom: theme.spacing(2),
		},
		detailsTable: {
			marginTop: theme.spacing(2),
		},

		// Table and list styles
		tableHeaderCell: {
			fontWeight: 600,
			backgroundColor: theme.palette.mode === "light" ? theme.palette.grey[100] : theme.palette.grey[800],
		},
		tableRow: {
			"&:nth-of-type(even)": {
				backgroundColor: theme.palette.mode === "light" ? theme.palette.grey[50] : theme.palette.grey[900],
			},
			"&:hover": {
				backgroundColor: theme.palette.mode === "light" ? theme.palette.action.hover : theme.palette.grey[800],
			},
		},
		filterFormSection: {
			padding: theme.spacing(3),
			marginBottom: theme.spacing(3),
			backgroundColor: theme.palette.background.paper,
			borderRadius: theme.shape.borderRadius,
			boxShadow: theme.shadows[1],
			border: `1px solid ${theme.palette.divider}`,
		},

		// Existing styles
		statusChip: (status: string) => commonStyles.statusChip(theme, status),
	};
};

export default useCommonStyles;
