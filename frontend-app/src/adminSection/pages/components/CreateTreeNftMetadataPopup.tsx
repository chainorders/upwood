import React, { useState } from "react";
import {
	Dialog,
	DialogTitle,
	DialogContent,
	DialogActions,
	Button,
	TextField,
	CircularProgress,
	Alert,
	Accordion,
	AccordionSummary,
	AccordionDetails,
	Typography,
} from "@mui/material";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import { useForm, Controller, SubmitHandler } from "react-hook-form";
import { TreeNftMetadataService } from "../../../apiClient/services/TreeNftMetadataService";
import { AddMetadataRequest } from "../../../apiClient/models/AddMetadataRequest";
import MetadataEditor from "../../components/MetadataEditor";
import { TokenMetadata } from "../../libs/types";
import { adminUploadJson, hashMetadata } from "../../libs/utils";

interface CreateTreeNftMetadataPopupProps {
	open: boolean;
	onClose: () => void;
	onSuccess: () => void;
	fileBaseUrl: string;
}

interface FormData {
	metadata_url: string;
	metadata_hash?: string;
	probablity_percentage: number;
}

export const CreateTreeNftMetadataPopup: React.FC<CreateTreeNftMetadataPopupProps> = ({
	open,
	onClose,
	onSuccess,
	fileBaseUrl,
}) => {
	const [isSubmitting, setIsSubmitting] = useState(false);
	const [submitError, setSubmitError] = useState<string | null>(null);
	const [expanded, setExpanded] = useState<boolean>(true);

	const {
		control,
		handleSubmit,
		reset,
		setValue,
		watch,
		formState: { errors },
	} = useForm<FormData>({
		defaultValues: {
			metadata_url: "",
			metadata_hash: "",
			probablity_percentage: 1,
		},
		mode: "onBlur",
	});

	const metadataUrl = watch("metadata_url");

	const handleMetadataSubmit = async (data: TokenMetadata) => {
		setSubmitError(null);

		try {
			const jsonData = JSON.stringify(data);
			const url = await adminUploadJson(fileBaseUrl, "nft-metadata", jsonData);
			setValue("metadata_url", url);
			const jsonDataHash = await hashMetadata(data);
			setValue("metadata_hash", jsonDataHash);
			setExpanded(false);
		} catch (error) {
			console.error("Error uploading metadata:", error);
			setSubmitError("Failed to upload metadata. Please try again.");
		}
	};

	const onSubmit: SubmitHandler<FormData> = async (data) => {
		setSubmitError(null);
		setIsSubmitting(true);

		try {
			const requestData: AddMetadataRequest = {
				metadata_url: {
					url: data.metadata_url,
					hash: data.metadata_hash || undefined,
				},
				probability_percentage: Number(data.probablity_percentage),
			};

			await TreeNftMetadataService.postAdminTreeNftMetadata(requestData);

			// Reset form
			reset({
				metadata_url: "",
				metadata_hash: "",
				probablity_percentage: 1,
			});

			onSuccess();
		} catch (err) {
			console.error("Error creating metadata:", err);
			setSubmitError("Failed to create metadata. Please try again.");
		} finally {
			setIsSubmitting(false);
		}
	};

	const handleClose = () => {
		if (!isSubmitting) {
			reset();
			onClose();
		}
	};

	return (
		<Dialog open={open} onClose={handleClose} maxWidth="md" fullWidth>
			<DialogTitle>Create New Tree NFT Metadata</DialogTitle>
			<form onSubmit={handleSubmit(onSubmit)}>
				<DialogContent>
					{submitError && (
						<Alert severity="error" sx={{ mb: 2 }}>
							{submitError}
						</Alert>
					)}

					<Controller
						name="metadata_url"
						control={control}
						rules={{
							required: "Metadata URL is required",
							pattern: {
								value: /^https?:\/\/.+/,
								message: "Metadata URL must be a valid URL starting with http:// or https://",
							},
						}}
						render={({ field }) => (
							<TextField
								{...field}
								label="Metadata URL"
								fullWidth
								margin="normal"
								error={Boolean(errors.metadata_url)}
								helperText={errors.metadata_url?.message || "Enter a URL to fetch metadata or generate it below"}
								required
								disabled={isSubmitting}
								InputLabelProps={{ shrink: !!field.value }}
							/>
						)}
					/>

					<Accordion expanded={expanded} onChange={() => setExpanded(!expanded)} sx={{ mb: 2, mt: 2 }}>
						<AccordionSummary
							expandIcon={<ExpandMoreIcon />}
							aria-controls="metadata-editor-content"
							id="metadata-editor-header"
						>
							<Typography>Metadata Editor</Typography>
						</AccordionSummary>
						<AccordionDetails>
							<MetadataEditor
								defaultMetadata={{
									name: "Tree NFT",
									symbol: "$TREE",
									decimals: 0,
									description: "Tree NFT",
									unique: true,
								}}
								metadataUrl={metadataUrl}
								fileBaseUrl={fileBaseUrl}
								onMetadataSubmit={handleMetadataSubmit}
							/>
						</AccordionDetails>
					</Accordion>

					<Controller
						name="metadata_hash"
						control={control}
						render={({ field }) => (
							<TextField
								{...field}
								label="Metadata Hash (Optional)"
								fullWidth
								margin="normal"
								disabled={isSubmitting}
								InputLabelProps={{ shrink: !!field.value }}
							/>
						)}
					/>

					<Controller
						name="probablity_percentage"
						control={control}
						rules={{
							required: "Probability percentage is required",
							min: {
								value: 1,
								message: "Probability must be at least 1%",
							},
							max: {
								value: 100,
								message: "Probability cannot exceed 100%",
							},
						}}
						render={({ field }) => (
							<TextField
								{...field}
								label="Probability Percentage"
								type="number"
								fullWidth
								margin="normal"
								error={Boolean(errors.probablity_percentage)}
								helperText={errors.probablity_percentage?.message || "Enter a value between 1 and 100"}
								required
								disabled={isSubmitting}
								inputProps={{ min: 1, max: 100 }}
							/>
						)}
					/>
				</DialogContent>

				<DialogActions>
					<Button onClick={handleClose} disabled={isSubmitting}>
						Cancel
					</Button>
					<Button type="submit" variant="contained" color="primary" disabled={isSubmitting}>
						{isSubmitting ? <CircularProgress size={24} /> : "Create"}
					</Button>
				</DialogActions>
			</form>
		</Dialog>
	);
};
