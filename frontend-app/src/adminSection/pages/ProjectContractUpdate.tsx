import { useCallback } from "react";
import { useParams } from "react-router";
import { useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { Link } from "react-router";
import {
	Breadcrumbs,
	Typography,
	Box,
	Button,
	TextField,
	Select,
	MenuItem,
	InputLabel,
	FormControl,
	CircularProgress,
	Accordion,
	AccordionSummary,
	AccordionDetails,
	Alert,
	IconButton,
} from "@mui/material";
import {
	ForestProject,
	ForestProjectService,
	ForestProjectTokenContract,
	SecurityTokenContractType,
} from "../../apiClient";
import { User } from "../../lib/user.ts";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import RefreshIcon from "@mui/icons-material/Refresh";
import TokenMetadataForm from "../components/TokenMetadataForm";
import { TokenMetadata } from "../libs/types";
import { adminUploadJson, hashMetadata } from "../libs/utils";

const ProjectContractUpdate = ({ fileBaseUrl }: { user: User; fileBaseUrl: string }) => {
	const { contract_address } = useParams<{ contract_address?: string }>();
	const [loading, setLoading] = useState(true);
	const [contract, setContract] = useState<ForestProjectTokenContract | null>(null);
	const [project, setProject] = useState<ForestProject | null>(null);

	// Metadata related state
	const [expanded, setExpanded] = useState<boolean>(false);
	const [metadata, setMetadata] = useState<TokenMetadata>();
	const [isMetadataLoading, setIsMetadataLoading] = useState<boolean>(false);
	const [metadataError, setMetadataError] = useState<string | null>(null);

	const {
		register,
		handleSubmit,
		setValue,
		formState: { errors },
		watch,
	} = useForm<ForestProjectTokenContract>();

	// Get current values for metadata
	const metadataUrl = watch("metadata_url");
	const symbol = watch("symbol");
	const decimals = watch("decimals");

	// Fetch metadata from URL
	const fetchMetadata = useCallback(
		async (url: string) => {
			if (!url || url.trim() === "") {
				// Reset to default metadata if URL is empty
				// setMetadata();
				setValue("metadata_hash", undefined);
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
				hashMetadata(data).then((hash) => setValue("metadata_hash", hash));
			} catch (error) {
				console.error("Error fetching metadata:", error);
				setMetadataError(error instanceof Error ? error.message : "Failed to fetch metadata");

				// Set default values on error
				// setMetadata(defaultMetadata);
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

	useEffect(() => {
		if (contract_address) {
			ForestProjectService.getAdminForestProjectsContract(contract_address)
				.then((data) => {
					setContract(data);
					setLoading(false);
					Object.keys(data).forEach((key) => {
						setValue(key as keyof ForestProjectTokenContract, data[key as keyof ForestProjectTokenContract]);
					});

					// Fetch metadata if URL is available
					if (data.metadata_url) {
						fetchMetadata(data.metadata_url);
					}
				})
				.catch(() => {
					alert("Failed to fetch contract details");
					setLoading(false);
				});
		}
	}, [contract_address, setValue, fetchMetadata]);

	useEffect(() => {
		if (contract) {
			ForestProjectService.getAdminForestProjects(contract.forest_project_id).then(setProject);
		}
	}, [contract]);

	const onSubmit = (data: ForestProjectTokenContract) => {
		ForestProjectService.putAdminForestProjectsContract(data)
			.then(() => {
				alert("Contract updated successfully");
			})
			.catch(() => {
				alert("Failed to update contract");
			});
	};

	const handleMetadataSubmit = async (data: TokenMetadata) => {
		const jsonData = JSON.stringify(data);
		const url = await adminUploadJson(fileBaseUrl, "metadata", jsonData);
		setValue("metadata_url", url);
		const jsonDataHash = await hashMetadata(data);
		setValue("metadata_hash", jsonDataHash);
		setExpanded(false);
	};

	// Render metadata form or loading state
	const renderMetadataContent = () => {
		if (isMetadataLoading) {
			return (
				<Box sx={{ display: "flex", justifyContent: "center", p: 4 }}>
					<CircularProgress />
				</Box>
			);
		}

		if (metadataError) {
			return (
				<>
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
					<TokenMetadataForm
						initialData={{
							...metadata,
							symbol: symbol || metadata?.symbol,
							decimals: decimals || metadata?.decimals,
						}}
						onSubmit={handleMetadataSubmit}
						submitButtonText="Generate Metadata URL"
						noForm={true}
						fileBaseUrl={fileBaseUrl}
					/>
				</>
			);
		}

		return (
			<TokenMetadataForm
				initialData={{
					...metadata,
					symbol: symbol || metadata?.symbol,
					decimals: decimals || metadata?.decimals,
				}}
				onSubmit={handleMetadataSubmit}
				submitButtonText="Generate Metadata URL"
				noForm={true}
				fileBaseUrl={fileBaseUrl}
			/>
		);
	};

	if (loading) {
		return <CircularProgress />;
	}

	if (!contract || !project) {
		return <Typography>No contract found</Typography>;
	}

	return (
		<>
			<Breadcrumbs aria-label="breadcrumb">
				<Link to="/admin">Admin</Link>
				<Link to="/admin/projects">Projects</Link>
				<Link to={`/admin/projects/${project.id}/details`}>{project.name}</Link>
				<Link to={`/admin/projects/${project.id}/contract/${contract_address}/details`}>
					{contract.contract_type} - {contract.contract_address}
				</Link>
				<Typography color="textPrimary">Contract Update</Typography>
			</Breadcrumbs>
			<div>
				<Box component="form" onSubmit={handleSubmit(onSubmit)} sx={{ display: "flex", flexDirection: "column", gap: 2 }}>
					<TextField
						label="Contract Address"
						{...register("contract_address", { required: true })}
						error={!!errors.contract_address}
						helperText={errors.contract_address ? "This field is required" : ""}
						disabled
					/>
					<FormControl error={!!errors.contract_type}>
						<InputLabel id="contract-type-label">Contract Type</InputLabel>
						<Select
							labelId="contract-type-label"
							{...register("contract_type", { required: true })}
							label="Contract Type"
							defaultValue={contract.contract_type}
						>
							<MenuItem value={SecurityTokenContractType.PROPERTY}>Property</MenuItem>
							<MenuItem value={SecurityTokenContractType.BOND}>Bond</MenuItem>
							<MenuItem value={SecurityTokenContractType.PROPERTY_PRE_SALE}>Property Pre Sale</MenuItem>
							<MenuItem value={SecurityTokenContractType.BOND_PRE_SALE}>Bond Pre Sale</MenuItem>
						</Select>
						{errors.contract_type && <span>This field is required</span>}
					</FormControl>
					<TextField
						label="Fund Token ID (optional)"
						{...register("fund_token_id", { setValueAs: (val: string) => val || undefined })}
					/>
					<TextField label="Symbol (optional)" {...register("symbol", { setValueAs: (val: string) => val || undefined })} />
					<TextField label="Decimals (optional)" type="number" {...register("decimals", { valueAsNumber: true })} />

					{/* Metadata URL with refresh button */}
					<Box sx={{ display: "flex", alignItems: "center" }}>
						<TextField
							label="Metadata URL"
							{...register("metadata_url", { setValueAs: (val: string) => val || undefined })}
							value={metadataUrl || ""}
							helperText="Enter a URL to fetch metadata or generate it below"
							fullWidth
							sx={{ mr: 1 }}
							InputLabelProps={{ shrink: !!metadataUrl }}
						/>
						{metadataUrl && (
							<IconButton onClick={() => fetchMetadata(metadataUrl)} disabled={isMetadataLoading} sx={{ mt: -1 }}>
								<RefreshIcon />
							</IconButton>
						)}
					</Box>

					{/* Metadata Editor Accordion */}
					<Accordion expanded={expanded} onChange={() => setExpanded(!expanded)} sx={{ mb: 2 }}>
						<AccordionSummary
							expandIcon={<ExpandMoreIcon />}
							aria-controls="token-metadata-form-content"
							id="token-metadata-form-header"
						>
							<Typography>Token Metadata Editor</Typography>
						</AccordionSummary>
						<AccordionDetails>{renderMetadataContent()}</AccordionDetails>
					</Accordion>

					<TextField
						label="Metadata Hash (optional)"
						{...register("metadata_hash", { setValueAs: (val: string) => val || undefined })}
						InputLabelProps={{ shrink: !!watch("metadata_hash") }}
					/>

					<Button type="submit" variant="contained" color="primary">
						Update Contract
					</Button>
				</Box>
			</div>
		</>
	);
};

export default ProjectContractUpdate;
