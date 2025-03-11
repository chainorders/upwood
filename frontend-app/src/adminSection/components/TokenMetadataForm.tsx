import React, { useEffect } from "react";
import { useForm, Controller } from "react-hook-form";
import {
	Box,
	TextField,
	Button,
	Typography,
	Switch,
	FormControlLabel,
	Grid,
	Card,
	CardContent,
	Divider,
} from "@mui/material";
import { TokenMetadata } from "../libs/types";
import ImageUploader from "./ImageUploader/ImageUploader";
import { adminUploadImage } from "../libs/utils";

interface TokenMetadataFormProps {
	initialData?: TokenMetadata;
	onSubmit: (data: TokenMetadata) => void;
	submitButtonText?: string;
	noForm?: boolean;
	fileBaseUrl: string;
}

const TokenMetadataForm: React.FC<TokenMetadataFormProps> = ({
	initialData,
	onSubmit,
	submitButtonText = "Save",
	noForm = false,
	fileBaseUrl,
}) => {
	const {
		control,
		handleSubmit,
		formState: { errors },
		setValue,
		watch,
		reset,
	} = useForm<TokenMetadata>({
		defaultValues: initialData || {
			name: "",
			symbol: "",
			unique: false,
			decimals: 0,
			description: "",
			thumbnail: { url: "" },
			display: { url: "" },
			artifact: { url: "" },
		},
	});

	// Update form values when initialData changes
	useEffect(() => {
		if (initialData) {
			// Use reset to update all form values when initialData changes
			reset({
				name: initialData.name || "",
				symbol: initialData.symbol || "",
				unique: initialData.unique || false,
				decimals: initialData.decimals || 0,
				description: initialData.description || "",
				thumbnail: {
					url: initialData.thumbnail?.url || "",
					mimeType: initialData.thumbnail?.mimeType || "",
				},
				display: {
					url: initialData.display?.url || "",
					mimeType: initialData.display?.mimeType || "",
				},
				artifact: {
					url: initialData.artifact?.url || "",
					mimeType: initialData.artifact?.mimeType || "",
				},
			});
		}
	}, [initialData, reset]);

	// Watch values for thumbnail and display to pass to ImageUploader
	const thumbnailUrl = watch("thumbnail.url");
	const thumbnailMimeType = watch("thumbnail.mimeType");
	const displayUrl = watch("display.url");
	const displayMimeType = watch("display.mimeType");
	const artifactUrl = watch("artifact.url");
	const artifactMimeType = watch("artifact.mimeType");

	// Create a submission handler that can be used directly with a button
	const handleFormSubmit = () => {
		handleSubmit(onSubmit)();
	};

	// Handle image uploads for thumbnail
	const handleThumbnailChange = (imageData?: string, mimeType?: string) => {
		if (!imageData) {
			setValue("thumbnail.url", "");
			setValue("thumbnail.mimeType", "");
			return;
		}

		adminUploadImage(fileBaseUrl, "thumbnail", imageData).then((url) => {
			setValue("thumbnail.url", url);
			setValue("thumbnail.mimeType", mimeType || "");
		});
	};

	// Handle image uploads for display
	const handleDisplayChange = (imageData?: string, mimeType?: string) => {
		if (!imageData) {
			setValue("display.url", "");
			setValue("display.mimeType", "");
			return;
		}

		adminUploadImage(fileBaseUrl, "display", imageData).then((url) => {
			setValue("display.url", url);
			setValue("display.mimeType", mimeType || "");
		});
	};

	// Content of the form stays the same regardless of whether it's wrapped in a form element
	const formContent = (
		<Grid container spacing={3}>
			{/* Basic Token Info */}
			<Grid item xs={12}>
				<Typography variant="h6">Basic Information</Typography>
				<Divider sx={{ my: 1 }} />
			</Grid>

			<Grid item xs={12} md={6}>
				<Controller
					name="name"
					control={control}
					rules={{ required: "Name is required" }}
					render={({ field }) => (
						<TextField {...field} label="Name" fullWidth error={!!errors.name} helperText={errors.name?.message} />
					)}
				/>
			</Grid>

			<Grid item xs={12} md={6}>
				<Controller
					name="symbol"
					control={control}
					render={({ field }) => (
						<TextField {...field} label="Symbol" fullWidth error={!!errors.symbol} helperText={errors.symbol?.message} />
					)}
				/>
			</Grid>

			<Grid item xs={12} md={6}>
				<Controller
					name="decimals"
					control={control}
					render={({ field }) => (
						<TextField
							{...field}
							label="Decimals"
							type="number"
							fullWidth
							inputProps={{ min: 0 }}
							error={!!errors.decimals}
							helperText={errors.decimals?.message}
						/>
					)}
				/>
			</Grid>

			<Grid item xs={12} md={6}>
				<Controller
					name="unique"
					control={control}
					render={({ field }) => (
						<FormControlLabel
							control={<Switch checked={!!field.value} onChange={(e) => field.onChange(e.target.checked)} />}
							label="Is Unique Token?"
						/>
					)}
				/>
			</Grid>

			<Grid item xs={12}>
				<Controller
					name="description"
					control={control}
					render={({ field }) => (
						<TextField
							{...field}
							label="Description"
							fullWidth
							multiline
							rows={3}
							error={!!errors.description}
							helperText={errors.description?.message}
						/>
					)}
				/>
			</Grid>

			{/* URL Objects with Image Uploaders */}
			<Grid item xs={12}>
				<Typography variant="h6" sx={{ mt: 2 }}>
					Media
				</Typography>
				<Divider sx={{ my: 1 }} />
			</Grid>

			{/* Thumbnail with ImageUploader */}
			<Grid item xs={12} md={6}>
				<ImageUploader
					label="Thumbnail"
					url={thumbnailUrl}
					mimeType={thumbnailMimeType}
					onChange={handleThumbnailChange}
					aspectRatio={1} // Square thumbnail is common
					height={200}
				/>
			</Grid>

			{/* Display with ImageUploader */}
			<Grid item xs={12} md={6}>
				<ImageUploader
					label="Display Image"
					url={displayUrl}
					mimeType={displayMimeType}
					onChange={handleDisplayChange}
					aspectRatio={16 / 9} // Wider display image
					height={200}
				/>
			</Grid>

			{/* Artifact - Keep as manual input for non-image files */}
			<Grid item xs={12}>
				<Card variant="outlined">
					<CardContent>
						<Typography variant="subtitle1" gutterBottom>
							Artifact
						</Typography>
						<Typography variant="caption" color="text.secondary" gutterBottom>
							Reference to the actual content represented by this token (e.g., document, video, etc.)
						</Typography>

						<Grid container spacing={2} sx={{ mt: 1 }}>
							<Grid item xs={12} md={6}>
								<Controller
									name="artifact.url"
									control={control}
									render={({ field }) => <TextField {...field} label="URL" fullWidth size="small" sx={{ mb: 2 }} />}
								/>
							</Grid>
							<Grid item xs={12} md={6}>
								<Controller
									name="artifact.mimeType"
									control={control}
									render={({ field }) => (
										<TextField {...field} label="MIME Type" fullWidth size="small" placeholder="e.g., application/pdf" />
									)}
								/>
							</Grid>
						</Grid>
					</CardContent>
				</Card>
			</Grid>

			<Grid item xs={12}>
				<Box sx={{ display: "flex", justifyContent: "flex-end", mt: 2 }}>
					<Button
						onClick={noForm ? handleFormSubmit : undefined}
						type={noForm ? "button" : "submit"}
						variant="contained"
						color="primary"
						size="large"
					>
						{submitButtonText}
					</Button>
				</Box>
			</Grid>
		</Grid>
	);

	// Return either a form element or just the content based on the noForm prop
	return noForm ? (
		<Box sx={{ width: "100%" }}>{formContent}</Box>
	) : (
		<Box component="form" onSubmit={handleSubmit(onSubmit)} sx={{ width: "100%" }}>
			{formContent}
		</Box>
	);
};

export default TokenMetadataForm;
