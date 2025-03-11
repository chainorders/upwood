import React, { useCallback } from "react";
import { useNavigate, useParams } from "react-router";
import { useForm } from "react-hook-form";
import {
	ForestProjectTokenContract,
	ForestProjectService,
	SecurityTokenContractType,
	SystemContractsConfigApiModel,
	UserService,
	ForestProject,
	FilesService,
} from "../../apiClient";
import {
	Button,
	TextField,
	Select,
	MenuItem,
	InputLabel,
	FormControl,
	Box,
	Breadcrumbs,
	Typography,
	Accordion,
	AccordionSummary,
	AccordionDetails,
	Alert,
	CircularProgress,
	IconButton,
} from "@mui/material";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import RefreshIcon from "@mui/icons-material/Refresh";
import { formatDate, parseFinalizedInit } from "../../lib/conversions";
import securitySftMulti from "../../contractClients/generated/securitySftMulti";
import { detectConcordiumProvider, WalletApi } from "@concordium/browser-wallet-api-helpers";
import { AccountAddress, TransactionHash } from "@concordium/web-sdk";
import { useEffect, useState } from "react";
import { User } from "../../lib/user";
import concordiumNodeClient from "../../contractClients/ConcordiumNodeClient";
import { Link } from "react-router";
import TokenMetadataForm from "../components/TokenMetadataForm";
import { TokenMetadata } from "../libs/types";
import { adminUploadJson, hashMetadata } from "../libs/utils";

const ProjectContractCreate = ({ user, fileBaseUrl }: { user: User; fileBaseUrl: string }) => {
	const { id } = useParams<{ id: string }>();
	const navigate = useNavigate();

	const [contractsConfig, setContractsConfig] = useState<SystemContractsConfigApiModel | null>(null);
	const [walletApi, setWalletApi] = useState<WalletApi>();
	const [txnStatus, setTxnStatus] = useState<"sending" | "waiting" | "success" | "error" | "none">("none");
	const [project, setProject] = useState<ForestProject | null>(null);
	const [expanded, setExpanded] = useState<boolean>(false);
	const [metadata, setMetadata] = useState<TokenMetadata>({
		name: "",
		symbol: "",
		decimals: 0,
		description: "",
	});
	const [isMetadataLoading, setIsMetadataLoading] = useState<boolean>(false);
	const [metadataError, setMetadataError] = useState<string | null>(null);

	const {
		register,
		handleSubmit,
		setValue,
		formState: { errors },
		watch,
	} = useForm<ForestProjectTokenContract>();

	// Get current values
	const contractAddress = watch("contract_address");
	const metadataUrl = watch("metadata_url");
	const symbol = watch("symbol");
	const decimals = watch("decimals");

	// Fetch metadata from URL
	const fetchMetadata = useCallback(
		async (url: string) => {
			if (!url || url.trim() === "") {
				// Reset to default metadata if URL is empty
				setMetadata({
					name: project?.name || "",
					symbol: symbol || "",
					decimals: decimals || 0,
					description: project?.desc_long || "",
				});
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
				setMetadata({
					name: project?.name || "",
					symbol: symbol || "",
					decimals: decimals || 0,
					description: project?.desc_long || "",
				});
			} finally {
				setIsMetadataLoading(false);
			}
		},
		[decimals, project?.desc_long, project?.name, symbol, setValue],
	);

	// Trigger metadata fetch when URL changes
	useEffect(() => {
		fetchMetadata(metadataUrl);
	}, [decimals, fetchMetadata, metadataUrl]);

	useEffect(() => {
		UserService.getSystemConfig()
			.then((config) => {
				setContractsConfig(config);
			})
			.catch(() => {
				alert("Failed to fetch system config");
			});
		detectConcordiumProvider().then((walletApi) => {
			setWalletApi(walletApi);
		});
	}, []);

	useEffect(() => {
		if (id) {
			ForestProjectService.getAdminForestProjects(id).then((project) => {
				setProject(project);

				// Initialize metadata with project data
				setMetadata((prev) => ({
					...prev,
					name: project.name || prev.name,
					description: project.desc_long || prev.description,
				}));
			});
		}
	}, [id]);

	const onSubmit = (data: ForestProjectTokenContract) => {
		const now = new Date();
		data.created_at = formatDate(now);
		data.updated_at = formatDate(now);
		data.forest_project_id = id!;
		console.log(data);

		ForestProjectService.postAdminForestProjectsContract(data)
			.then(() => {
				alert("Contract created successfully");
				navigate(-1);
			})
			.catch(() => {
				alert("Failed to create contract");
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

	const initializeContract = async () => {
		if (txnStatus == "sending" || txnStatus == "waiting") {
			alert("Transaction already in progress");
			return;
		}
		if (!contractsConfig) {
			alert("No system config detected");
			return;
		}
		if (!walletApi) {
			alert("No wallet detected");
			return;
		}
		let account = await walletApi.getMostRecentlySelectedAccount();
		if (!account || account != user.concordiumAccountAddress) {
			const accounts = await walletApi.requestAccounts();
			account = accounts.find((account) => account == user.concordiumAccountAddress);
		}
		if (!account) {
			alert("No account selected or account is not the same as the user account");
			return;
		}
		const accountAddress = AccountAddress.fromBase58(account);
		setTxnStatus("sending");
		const txnHash = await securitySftMulti.init.init(walletApi, accountAddress, {
			security: {
				Some: [
					{
						identity_registry: {
							index: Number(contractsConfig.identity_registry_contract_index),
							subindex: 0,
						},
						compliance: {
							index: Number(contractsConfig.compliance_contract_index),
							subindex: 0,
						},
					},
				],
			},
			agents: [
				{
					address: {
						Contract: [
							{
								index: Number(contractsConfig.mint_funds_contract_index),
								subindex: 0,
							},
						],
					},
					roles: [
						{
							Mint: {},
						},
						{
							Operator: {},
						},
						{
							ForcedBurn: {},
						},
					],
				},
				{
					address: {
						Contract: [
							{
								index: Number(contractsConfig.yielder_contract_index),
								subindex: 0,
							},
						],
					},
					roles: [
						{
							Mint: {},
						},
						{
							Operator: {},
						},
					],
				},
				{
					address: {
						Contract: [
							{
								index: Number(contractsConfig.trading_contract_index),
								subindex: 0,
							},
						],
					},
					roles: [
						{
							Mint: {},
						},
						{
							Operator: {},
						},
					],
				},
			],
		});
		setTxnStatus("waiting");
		const outcome = await concordiumNodeClient.waitForTransactionFinalization(TransactionHash.fromHexString(txnHash));
		const txnResult = parseFinalizedInit(outcome);
		switch (txnResult.tag) {
			case "success": {
				setTxnStatus("success");
				setValue("contract_address", txnResult.value.index.toString());
				break;
			}
			case "error": {
				setTxnStatus("error");
				alert(`Failed to initialize contract: ${txnResult.value.rejectReason}`);
				break;
			}
		}
	};

	if (!project) {
		return <div>Loading...</div>;
	}

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
							symbol: symbol || metadata.symbol,
							decimals: decimals || metadata.decimals,
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
					symbol: symbol || metadata.symbol,
					decimals: decimals || metadata.decimals,
				}}
				onSubmit={handleMetadataSubmit}
				submitButtonText="Generate Metadata URL"
				noForm={true}
				fileBaseUrl={fileBaseUrl}
			/>
		);
	};

	return (
		<>
			<Breadcrumbs aria-label="breadcrumb">
				<Link to="/admin">Admin</Link>
				<Link to="/admin/projects">Projects</Link>
				<Link to={`/admin/projects/${id}/details`}>{project.name}</Link>
				<Typography color="textPrimary">Contract Create</Typography>
			</Breadcrumbs>
			<div>
				<Box component="form" onSubmit={handleSubmit(onSubmit)} sx={{ display: "flex", flexDirection: "column", gap: 2 }}>
					<TextField
						label="Contract Address"
						{...register("contract_address", { required: true })}
						error={!!errors.contract_address}
						helperText={errors.contract_address ? "This field is required" : ""}
						value={contractAddress || ""}
					/>
					<Button
						variant="outlined"
						onClick={initializeContract}
						disabled={txnStatus === "sending" || txnStatus === "waiting"}
						startIcon={(txnStatus === "sending" || txnStatus === "waiting") && <CircularProgress size={20} />}
						color={txnStatus === "error" ? "error" : txnStatus === "success" ? "success" : "primary"}
					>
						{txnStatus === "sending" || txnStatus === "waiting" ? "Initializing..." : "Initialize Contract"}
					</Button>
					<FormControl error={!!errors.contract_type}>
						<InputLabel id="contract-type-label">Contract Type</InputLabel>
						<Select
							labelId="contract-type-label"
							{...register("contract_type", { required: true })}
							label="Contract Type"
							defaultValue={SecurityTokenContractType.PROPERTY}
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
					<TextField
						label="Market Token ID (optional)"
						{...register("market_token_id", { setValueAs: (val: string) => val || undefined })}
					/>
					<TextField
						label="Symbol"
						{...register("symbol", { required: true })}
						error={!!errors.symbol}
						helperText={errors.symbol ? "This field is required" : ""}
					/>
					<TextField
						label="Decimals"
						type="number"
						{...register("decimals", { required: true, valueAsNumber: true })}
						error={!!errors.decimals}
						helperText={errors.decimals ? "This field is required" : ""}
					/>

					<Box sx={{ display: "flex", alignItems: "center" }}>
						<TextField
							label="Metadata URL"
							{...register("metadata_url", { required: true })}
							error={!!errors.metadata_url}
							value={metadataUrl || ""}
							helperText={
								errors.metadata_url ? "This field is required" : "Enter a URL to fetch metadata or generate it below"
							}
							fullWidth
							sx={{ mr: 1 }}
							InputLabelProps={{ shrink: !!metadataUrl }}
						/>
						{metadataUrl && (
							<IconButton
								onClick={() => fetchMetadata(metadataUrl)}
								disabled={isMetadataLoading}
								sx={{ mt: errors.metadata_url ? -3 : -1 }}
							>
								<RefreshIcon />
							</IconButton>
						)}
					</Box>

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
					<Button type="submit" variant="contained" color="primary" disabled={!contractsConfig || !walletApi}>
						Create Contract
					</Button>
				</Box>
			</div>
		</>
	);
};

export default ProjectContractCreate;
