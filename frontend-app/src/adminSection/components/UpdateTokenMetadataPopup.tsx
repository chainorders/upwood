import React, { useState, useCallback, useEffect } from "react";
import {
	Dialog,
	DialogTitle,
	DialogContent,
	DialogActions,
	Button,
	TextField,
	Box,
	IconButton,
	Collapse,
	Typography,
	CircularProgress,
	Alert,
	Paper,
	alpha,
} from "@mui/material";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import ExpandLessIcon from "@mui/icons-material/ExpandLess";
import RefreshIcon from "@mui/icons-material/Refresh";
import EditIcon from "@mui/icons-material/Edit";
import { useForm, Controller } from "react-hook-form";
import MetadataEditor from "./MetadataEditor";
import { TokenMetadata } from "../libs/types";
import { adminUploadJson, hashMetadata } from "../libs/utils";
import TransactionButton from "../../components/TransactionButton";
import { User } from "../../lib/user";
import { TxnStatus, updateContract } from "../../lib/concordium";
import securitySftMulti from "../../contractClients/generated/securitySftMulti";
import securitySftSingle from "../../contractClients/generated/securitySftSingle";
import { toTokenId } from "../../lib/conversions";

interface UpdateTokenMetadataPopupProps {
	open: boolean;
	onClose: () => void;
	initialUrl?: string;
	initialHash?: string;
	fileBaseUrl: string;
	onUpdate?: (url: string, hash: string) => void;
	contractAddress: string;
	tokenId: string;
	user: User;
	method: typeof securitySftMulti.setTokenMetadata | typeof securitySftSingle.setTokenMetadata;
	tokenIdSize: number;
}

interface FormValues {
	metadataUrl: string;
	metadataHash: string;
}

const UpdateTokenMetadataPopup: React.FC<UpdateTokenMetadataPopupProps> = ({
	open,
	onClose,
	initialUrl = "",
	initialHash = "",
	fileBaseUrl,
	onUpdate,
	contractAddress,
	tokenId,
	user,
	method,
	tokenIdSize,
}) => {
	const { control, handleSubmit, watch, setValue } = useForm<FormValues>({
		defaultValues: {
			metadataUrl: initialUrl,
			metadataHash: initialHash,
		},
	});
	const [showMetadataEditor, setShowMetadataEditor] = useState(false);
	const [metadata, setMetadata] = useState<TokenMetadata>();
	const [isMetadataLoading, setIsMetadataLoading] = useState<boolean>(false);
	const [metadataError, setMetadataError] = useState<string | null>(null);
	const [txnStatus, setTxnStatus] = useState<TxnStatus>("none");
	const [txnError, setTxnError] = useState<string>();

	// Get current value for metadata URL
	const metadataUrl = watch("metadataUrl");
	const metadataHash = watch("metadataHash");

	// Fetch metadata from URL
	const fetchMetadata = useCallback(
		async (url: string) => {
			if (!url || url.trim() === "") {
				setMetadata(undefined);
				setValue("metadataHash", "");
				setMetadataError(null);
				setIsMetadataLoading(false);
				return;
			}

			setIsMetadataLoading(true);
			setMetadataError(null);

			try {
				const response = await fetch(url);

				if (!response.ok) {
					throw new Error(`Failed to fetch metadata: ${response.status} ${response.statusText}`);
				}

				const data = await response.json();
				setMetadata(data);
				hashMetadata(data).then((hash) => setValue("metadataHash", hash));
			} catch (error) {
				console.error("Error fetching metadata:", error);
				setMetadataError(error instanceof Error ? error.message : "Failed to fetch metadata");
				setMetadata(undefined);
			} finally {
				setIsMetadataLoading(false);
			}
		},
		[setValue],
	);

	// Trigger metadata fetch when URL changes
	useEffect(() => {
		if (metadataUrl) {
			fetchMetadata(metadataUrl);
		}
	}, [fetchMetadata, metadataUrl]);

	const handleUpdateMetadata = async () => {
		setTxnError(undefined);
		try {
			await updateContract(
				user.concordiumAccountAddress,
				contractAddress,
				method,
				{
					params: [
						{
							token_id: toTokenId(BigInt(tokenId), tokenIdSize),
							token_metadata: {
								url: metadataUrl,
								hash: metadataHash ? { Some: [metadataHash] } : { None: {} },
							},
						},
					],
				},
				setTxnStatus,
			);

			setTxnStatus("success");
			if (onUpdate) {
				onUpdate(metadataUrl, metadataHash);
			}
			setTimeout(() => {
				handleClose();
			}, 1000);
		} catch (error) {
			if (error instanceof Error) {
				setTxnError(error.message);
			} else if (typeof error === "string") {
				setTxnError(error);
			} else {
				setTxnError("An unknown error occurred.");
			}
			setTxnStatus("error");
		}
	};

	const handleClose = () => {
		setTxnError(undefined);
		setTxnStatus("none");
		onClose();
	};

	const handleMetadataSubmit = async (data: TokenMetadata) => {
		const jsonData = JSON.stringify(data);
		const url = await adminUploadJson(fileBaseUrl, "metadata", jsonData);
		setValue("metadataUrl", url);
		const jsonDataHash = await hashMetadata(data);
		setValue("metadataHash", jsonDataHash);
		setShowMetadataEditor(false);
	};

	// Default metadata to provide to the editor when no metadata is loaded
	const defaultMetadata: TokenMetadata = {
		name: "",
		symbol: "",
		decimals: 0,
		description: "",
		thumbnail: { url: "" },
		display: { url: "" },
	};

	return (
		<Dialog open={open} onClose={handleClose} maxWidth="md" fullWidth>
			<DialogTitle>Update Token Metadata</DialogTitle>
			<DialogContent>
				<Box sx={{ mt: 1 }}>
					<Paper
						elevation={0}
						sx={{
							p: 2,
							mb: 2,
							backgroundColor: (theme) => alpha(theme.palette.primary.main, 0.05),
							borderRadius: 1,
							border: (theme) => `1px solid ${alpha(theme.palette.primary.main, 0.2)}`,
						}}
					>
						<Box display="flex" alignItems="center" mb={2}>
							<EditIcon color="primary" sx={{ mr: 1 }} />
							<Typography variant="subtitle2" color="primary.main" fontWeight={600}>
								Update token metadata
							</Typography>
						</Box>

						<Typography variant="body2" color="text.secondary" sx={{ mb: 2 }}>
							You are updating the metadata for token ID {tokenId} on contract {contractAddress}
						</Typography>
					</Paper>

					<Box sx={{ display: "flex", alignItems: "center", mb: 2 }}>
						<Controller
							name="metadataUrl"
							control={control}
							rules={{ required: "Metadata URL is required" }}
							render={({ field, fieldState }) => (
								<TextField
									{...field}
									label="Metadata URL"
									fullWidth
									margin="normal"
									error={!!fieldState.error}
									helperText={fieldState.error?.message || "Enter a URL to fetch metadata or generate it below"}
								/>
							)}
						/>
						{metadataUrl && (
							<IconButton
								onClick={() => fetchMetadata(metadataUrl)}
								disabled={isMetadataLoading}
								sx={{ ml: 1, mt: 1 }}
								aria-label="Refresh metadata"
							>
								<RefreshIcon />
							</IconButton>
						)}
					</Box>

					<Controller
						name="metadataHash"
						control={control}
						render={({ field }) => (
							<TextField
								{...field}
								label="Metadata Hash"
								fullWidth
								margin="normal"
								InputProps={{
									readOnly: true,
								}}
								InputLabelProps={{
									shrink: !!field.value,
								}}
							/>
						)}
					/>

					<Box display="flex" alignItems="center" mt={2} mb={1}>
						<Typography variant="subtitle1" sx={{ flexGrow: 1 }}>
							Token Metadata Editor
						</Typography>
						<IconButton
							size="small"
							onClick={() => setShowMetadataEditor((v) => !v)}
							aria-label={showMetadataEditor ? "Hide editor" : "Show editor"}
						>
							{showMetadataEditor ? <ExpandLessIcon /> : <ExpandMoreIcon />}
						</IconButton>
					</Box>

					<Collapse in={showMetadataEditor}>
						<Box mb={2}>
							{isMetadataLoading ? (
								<Box sx={{ display: "flex", justifyContent: "center", p: 4 }}>
									<CircularProgress />
								</Box>
							) : metadataError ? (
								<Alert
									severity="error"
									sx={{ mb: 2 }}
									action={
										<IconButton color="inherit" size="small" onClick={() => metadataUrl && fetchMetadata(metadataUrl)}>
											<RefreshIcon />
										</IconButton>
									}
								>
									{metadataError}
								</Alert>
							) : null}

							<MetadataEditor
								defaultMetadata={metadata || defaultMetadata}
								metadataUrl={metadataUrl}
								fileBaseUrl={fileBaseUrl}
								onMetadataSubmit={handleMetadataSubmit}
							/>
						</Box>
					</Collapse>

					{txnError && (
						<Paper
							elevation={0}
							sx={{
								p: 2,
								mb: 2,
								backgroundColor: (theme) => alpha(theme.palette.error.main, 0.1),
								borderLeft: (theme) => `4px solid ${theme.palette.error.main}`,
								borderRadius: 1,
							}}
						>
							<Typography variant="body2" color="error">
								{txnError}
							</Typography>
						</Paper>
					)}
				</Box>
			</DialogContent>
			<DialogActions>
				<Button onClick={handleClose} color="secondary" variant="outlined">
					Cancel
				</Button>
				<TransactionButton
					onClick={handleSubmit(handleUpdateMetadata)}
					type="button"
					variant="contained"
					color="primary"
					startIcon={<EditIcon />}
					txnStatus={txnStatus}
					defaultText="Update Metadata"
					loadingText="Updating Metadata..."
					size="medium"
				/>
			</DialogActions>
		</Dialog>
	);
};

export default UpdateTokenMetadataPopup;
