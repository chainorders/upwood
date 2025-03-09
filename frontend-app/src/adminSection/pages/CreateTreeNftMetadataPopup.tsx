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
} from "@mui/material";
import { useForm, Controller, SubmitHandler } from "react-hook-form";
import { TreeNftMetadataService } from "../../apiClient/services/TreeNftMetadataService";
import { AddMetadataRequest } from "../../apiClient/models/AddMetadataRequest";

interface CreateTreeNftMetadataPopupProps {
	open: boolean;
	onClose: () => void;
	onSuccess: () => void;
}

interface FormData {
	metadata_url: string;
	metadata_hash?: string;
	probablity_percentage: number;
}

export const CreateTreeNftMetadataPopup: React.FC<CreateTreeNftMetadataPopupProps> = ({ open, onClose, onSuccess }) => {
	const [isSubmitting, setIsSubmitting] = useState(false);
	const [submitError, setSubmitError] = useState<string | null>(null);

	const {
		control,
		handleSubmit,
		reset,
		formState: { errors },
	} = useForm<FormData>({
		defaultValues: {
			metadata_url: "",
			metadata_hash: "",
			probablity_percentage: 1,
		},
		mode: "onBlur", // Validate on blur for better UX
	});

	const onSubmit: SubmitHandler<FormData> = async (data) => {
		setSubmitError(null);
		setIsSubmitting(true);

		try {
			const requestData: AddMetadataRequest = {
				metadata_url: {
					url: data.metadata_url,
					hash: data.metadata_hash || undefined,
				},
				probablity_percentage: data.probablity_percentage,
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
		<Dialog open={open} onClose={handleClose} maxWidth="sm" fullWidth>
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
								helperText={errors.metadata_url?.message}
								required
								disabled={isSubmitting}
							/>
						)}
					/>

					<Controller
						name="metadata_hash"
						control={control}
						render={({ field }) => (
							<TextField {...field} label="Metadata Hash (Optional)" fullWidth margin="normal" disabled={isSubmitting} />
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
