import { Box, Typography, SxProps, Theme } from "@mui/material";
import React from "react";

interface DetailRowProps {
	label: React.ReactNode;
	value: React.ReactNode;
	labelSx?: SxProps<Theme>;
	valueSx?: SxProps<Theme>;
}

/**
 * A reusable component for displaying a label-value pair in detail views
 */
const DetailRow: React.FC<DetailRowProps> = ({ label, value, labelSx = {}, valueSx = {} }) => {
	return (
		<Box
			sx={{
				display: "grid",
				gridTemplateColumns: { xs: "1fr", sm: "220px 1fr" },
				alignItems: "baseline",
				marginBottom: 1,
			}}
		>
			<Typography
				sx={{
					fontWeight: 600,
					color: "text.secondary",
					display: "inline-block",
					mb: { xs: 0.5, sm: 0 },
					...labelSx,
				}}
			>
				{label}:
			</Typography>
			<Typography
				sx={{
					wordBreak: "break-word",
					display: "inline-block",
					...valueSx,
				}}
			>
				{value}
			</Typography>
		</Box>
	);
};

export default DetailRow;
