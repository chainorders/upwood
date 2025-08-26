import React, { useState, useCallback } from "react";
import {
	Box,
	Button,
	Typography,
	Paper,
	IconButton,
	CircularProgress,
	Slider,
	Dialog,
	DialogActions,
	DialogContent,
	DialogTitle,
} from "@mui/material";
import EditIcon from "@mui/icons-material/Edit";
import DeleteIcon from "@mui/icons-material/Delete";
import CloudUploadIcon from "@mui/icons-material/CloudUpload";
import ZoomInIcon from "@mui/icons-material/ZoomIn";
import AspectRatioIcon from "@mui/icons-material/AspectRatio";
import getCroppedImg from "./cropImage";
import { useDropzone } from "react-dropzone";
import Cropper, { Area } from "react-easy-crop";

interface ImageUploaderProps {
	url?: string;
	mimeType?: string;
	onChange: (url: string | undefined, mimeType?: string) => void; // Updated to include mimeType
	aspectRatio?: number;
	maxSizeMB?: number;
	label?: string;
	className?: string;
	height?: number | string;
	width?: number | string;
}

const ImageUploader: React.FC<ImageUploaderProps> = ({
	url,
	mimeType,
	onChange,
	aspectRatio = 1,
	maxSizeMB = 5,
	label = "Image",
	className,
	height = 200,
	width = "100%",
}) => {
	// States
	const [isEditMode, setIsEditMode] = useState<boolean>(false);
	const [, setImageFile] = useState<File | null>(null);
	const [imageDataUrl, setImageDataUrl] = useState<string | null>(null);
	const [currentMimeType, setCurrentMimeType] = useState<string | undefined>(mimeType);
	const [isLoading, setIsLoading] = useState<boolean>(false);
	const [isCropperOpen, setIsCropperOpen] = useState<boolean>(false);

	// Cropper states
	const [crop, setCrop] = useState({ x: 0, y: 0 });
	const [zoom, setZoom] = useState(1);
	const [croppedAreaPixels, setCroppedAreaPixels] = useState<Area>();

	// Handlers
	const handleDelete = () => {
		onChange(undefined, undefined); // Pass undefined for both url and mimeType
		setImageFile(null);
		setImageDataUrl(null);
		setCurrentMimeType(undefined);
	};

	const handleEdit = () => {
		setIsEditMode(true);
	};

	const onDrop = useCallback(
		(acceptedFiles: File[]) => {
			if (acceptedFiles && acceptedFiles.length > 0) {
				const file = acceptedFiles[0];

				// Check file size
				if (file.size > maxSizeMB * 1024 * 1024) {
					alert(`File is too large. Maximum size is ${maxSizeMB}MB.`);
					return;
				}

				setImageFile(file);
				setCurrentMimeType(file.type); // Store the file's MIME type

				// Create a preview
				const reader = new FileReader();
				reader.onload = () => {
					setImageDataUrl(reader.result as string);
					setIsCropperOpen(true);
				};
				reader.readAsDataURL(file);
			}
		},
		[maxSizeMB],
	);

	const { getRootProps, getInputProps } = useDropzone({
		onDrop,
		accept: { "image/*": [] },
		maxFiles: 1,
	});

	// Cropper handlers
	const onCropComplete = useCallback((_croppedArea: Area, croppedAreaPixels: Area) => {
		setCroppedAreaPixels(croppedAreaPixels);
	}, []);

	const showCroppedImage = async () => {
		try {
			setIsLoading(true);

			if (!imageDataUrl || !croppedAreaPixels) {
				return;
			}

			const croppedImageUrl = await getCroppedImg(imageDataUrl, croppedAreaPixels);

			setIsLoading(false);
			setIsCropperOpen(false);

			// Pass both the URL and MIME type to the onChange handler
			onChange(croppedImageUrl, currentMimeType);
			setIsEditMode(false);
		} catch (e) {
			console.error("Error cropping image:", e);
			setIsLoading(false);
		}
	};

	const handleCancelCrop = () => {
		setIsCropperOpen(false);
		if (!url) {
			setIsEditMode(true);
		}
	};

	// Render functions
	const renderPreview = () => (
		<Box sx={{ position: "relative", height, width }}>
			<img
				src={url}
				alt={label}
				style={{
					width: "100%",
					height: "100%",
					objectFit: "contain",
					display: "block",
				}}
			/>
			<Box
				sx={{
					position: "absolute",
					bottom: 8,
					right: 8,
					display: "flex",
					gap: 1,
				}}
			>
				<IconButton color="primary" onClick={handleEdit} size="small" sx={{ bgcolor: "rgba(255,255,255,0.8)" }}>
					<EditIcon />
				</IconButton>
				<IconButton color="error" onClick={handleDelete} size="small" sx={{ bgcolor: "rgba(255,255,255,0.8)" }}>
					<DeleteIcon />
				</IconButton>
			</Box>
		</Box>
	);

	const renderDropzone = () => (
		<Paper
			{...getRootProps()}
			sx={{
				height,
				width,
				display: "flex",
				flexDirection: "column",
				alignItems: "center",
				justifyContent: "center",
				border: "2px dashed #ccc",
				borderRadius: 1,
				cursor: "pointer",
				p: 2,
				"&:hover": {
					borderColor: "primary.main",
					bgcolor: "rgba(0, 0, 0, 0.04)",
				},
			}}
		>
			<input {...getInputProps()} />
			<CloudUploadIcon color="primary" sx={{ fontSize: 40, mb: 1 }} />
			<Typography variant="body1" align="center" gutterBottom>
				Drag & drop an image here, or click to select
			</Typography>
			<Typography variant="caption" align="center" color="textSecondary">
				Supported formats: JPG, PNG, GIF (Max {maxSizeMB}MB)
			</Typography>
		</Paper>
	);

	// Render cropper dialog
	const renderCropperDialog = () => (
		<Dialog open={isCropperOpen} onClose={handleCancelCrop} maxWidth="md" fullWidth>
			<DialogTitle>Crop Image</DialogTitle>
			<DialogContent dividers>
				<Box sx={{ position: "relative", height: 400, width: "100%", mb: 2 }}>
					{imageDataUrl && (
						<Cropper
							image={imageDataUrl}
							crop={crop}
							zoom={zoom}
							aspect={aspectRatio}
							onCropChange={setCrop}
							onCropComplete={onCropComplete}
							onZoomChange={setZoom}
							showGrid={true}
							cropShape="rect"
							style={{
								cropAreaStyle: {
									border: "2px solid #fff",
									boxShadow: "0 0 0 9999em rgba(0, 0, 0, 0.5)",
								},
							}}
						/>
					)}
				</Box>
				<Box sx={{ display: "flex", alignItems: "center", gap: 2, mb: 1 }}>
					<ZoomInIcon />
					<Slider value={zoom} min={1} max={3} step={0.1} onChange={(_e, newValue) => setZoom(newValue as number)} />
				</Box>
				<Box sx={{ display: "flex", alignItems: "center", gap: 1 }}>
					<AspectRatioIcon fontSize="small" />
					<Typography variant="body2" color="text.secondary">
						Aspect Ratio: {aspectRatio === 1 ? "1:1 (Square)" : aspectRatio.toFixed(2)}
					</Typography>
				</Box>

				{currentMimeType && (
					<Box sx={{ display: "flex", alignItems: "center", gap: 1, mt: 1 }}>
						<Typography variant="body2" color="text.secondary">
							Type: {currentMimeType}
						</Typography>
					</Box>
				)}
			</DialogContent>
			<DialogActions>
				<Button onClick={handleCancelCrop}>Cancel</Button>
				<Button
					onClick={showCroppedImage}
					variant="contained"
					color="primary"
					disabled={isLoading}
					startIcon={isLoading ? <CircularProgress size={20} /> : null}
				>
					{isLoading ? "Processing..." : "Apply"}
				</Button>
			</DialogActions>
		</Dialog>
	);

	// Main render
	return (
		<Box className={className} sx={{ width }} component={Paper} p={2}>
			{label && (
				<Typography variant="subtitle1" gutterBottom>
					{label}
				</Typography>
			)}

			{url && !isEditMode ? renderPreview() : renderDropzone()}
			{renderCropperDialog()}

			{isEditMode && !isCropperOpen && (
				<Box sx={{ mt: 1, display: "flex", justifyContent: "flex-end" }}>
					<Button variant="outlined" color="primary" onClick={() => setIsEditMode(false)} size="small">
						Cancel
					</Button>
				</Box>
			)}

			{url && mimeType && (
				<Typography variant="caption" color="text.secondary" sx={{ mt: 1, display: "block" }}>
					{mimeType}
				</Typography>
			)}
		</Box>
	);
};

export default ImageUploader;
