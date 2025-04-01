import { useNavigate, useParams } from "react-router";
import { useForm } from "react-hook-form";
import {
	ForestProjectTokenContract,
	ForestProjectService,
	SecurityTokenContractType,
	SystemContractsConfigApiModel,
	UserService,
	ForestProject,
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
	CircularProgress,
	Paper,
	Container,
	Grid,
	Divider,
} from "@mui/material";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import { formatDate, parseFinalizedInit } from "../../lib/conversions";
import securitySftMulti from "../../contractClients/generated/securitySftMulti";
import { detectConcordiumProvider, WalletApi } from "@concordium/browser-wallet-api-helpers";
import { AccountAddress, TransactionHash } from "@concordium/web-sdk";
import { useEffect, useState } from "react";
import { User } from "../../lib/user";
import concordiumNodeClient from "../../contractClients/ConcordiumNodeClient";
import { Link } from "react-router";
import { TokenMetadata } from "../libs/types";
import { adminUploadJson, hashMetadata } from "../libs/utils";
import MetadataEditor from "../components/MetadataEditor";
import useCommonStyles from "../../theme/useCommonStyles";
import HomeIcon from "@mui/icons-material/Home";
import ForestIcon from "@mui/icons-material/Folder";
import ContractIcon from "@mui/icons-material/Description";
import TokenIcon from "@mui/icons-material/Token";
import DataObjectIcon from "@mui/icons-material/DataObject";
import TransactionButton from "../../components/TransactionButton";

const ProjectContractCreate = ({ user, fileBaseUrl }: { user: User; fileBaseUrl: string }) => {
	const { id } = useParams<{ id: string }>();
	const navigate = useNavigate();

	const [contractsConfig, setContractsConfig] = useState<SystemContractsConfigApiModel | null>(null);
	const [walletApi, setWalletApi] = useState<WalletApi>();
	const [txnStatus, setTxnStatus] = useState<"sending" | "waiting" | "success" | "error" | "none">("none");
	const [project, setProject] = useState<ForestProject | null>(null);
	const [expanded, setExpanded] = useState<boolean>(false);
	const styles = useCommonStyles();

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
			ForestProjectService.getAdminForestProjects(id).then(setProject);
		}
	}, [id]);

	const onSubmit = (data: ForestProjectTokenContract) => {
		const now = new Date();
		data.created_at = formatDate(now);
		data.updated_at = formatDate(now);
		data.forest_project_id = id!;

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

		try {
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
		} catch {
			setTxnStatus("error");
			alert("Failed to initialize contract");
		}
	};

	if (!project) {
		return <div>Loading...</div>;
	}

	return (
		<>
			<Breadcrumbs aria-label="breadcrumb" sx={{ mb: 2 }}>
				<Link to="/admin" style={styles.breadcrumbLink}>
					<HomeIcon sx={{ mr: 0.5 }} fontSize="small" />
					Admin
				</Link>
				<Link to="/admin/projects" style={styles.breadcrumbLink}>
					<ForestIcon sx={{ mr: 0.5 }} fontSize="small" />
					Projects
				</Link>
				<Link to={`/admin/projects/${id}/details`} style={styles.breadcrumbLink}>
					<ForestIcon sx={{ mr: 0.5 }} fontSize="small" />
					{project.name}
				</Link>
				<Typography color="text.primary" sx={{ display: "flex", alignItems: "center" }}>
					<ContractIcon sx={{ mr: 0.5 }} fontSize="small" />
					Create Contract
				</Typography>
			</Breadcrumbs>

			<Box sx={styles.sectionHeader}>
				<ContractIcon />
				<Typography variant="h4" gutterBottom>
					Create Project Contract
				</Typography>
			</Box>

			<Container maxWidth="lg" disableGutters>
				<Paper component="form" onSubmit={handleSubmit(onSubmit)} sx={styles.formContainer}>
					{/* Contract Initialization Section */}
					<Box sx={styles.formSection}>
						<Box sx={styles.formSectionHeader}>
							<ContractIcon />
							<Typography variant="h6">Contract Initialization</Typography>
						</Box>
						<Grid container spacing={3}>
							<Grid item xs={12}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										variant="outlined"
										label="Contract Address"
										{...register("contract_address", { required: true })}
										error={!!errors.contract_address}
										helperText={errors.contract_address ? "This field is required" : ""}
										value={contractAddress || ""}
										InputLabelProps={{ shrink: !!contractAddress }}
									/>
								</Box>
							</Grid>
							<Grid item xs={12}>
								<TransactionButton
									defaultText="Initialize Contract"
									txnStatus={txnStatus}
									onClick={initializeContract}
									disabled={txnStatus === "sending" || txnStatus === "waiting"}
									startIcon={(txnStatus === "sending" || txnStatus === "waiting") && <CircularProgress size={20} />}
									color={txnStatus === "error" ? "error" : txnStatus === "success" ? "success" : "primary"}
									fullWidth
									sx={{ py: 1 }}
									loadingText="Initializing..."
								/>
							</Grid>
						</Grid>
					</Box>

					<Divider sx={styles.formDivider} />

					{/* Token Configuration Section */}
					<Box sx={styles.formSection}>
						<Box sx={styles.formSectionHeader}>
							<TokenIcon />
							<Typography variant="h6">Token Configuration</Typography>
						</Box>
						<Grid container spacing={3}>
							<Grid item xs={12} md={6}>
								<Box sx={styles.formField}>
									<FormControl fullWidth error={!!errors.contract_type} variant="outlined">
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
										{errors.contract_type && (
											<Typography color="error" variant="caption">
												This field is required
											</Typography>
										)}
									</FormControl>
								</Box>
							</Grid>
							<Grid item xs={12} md={6}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										variant="outlined"
										label="Symbol"
										{...register("symbol", { required: true })}
										error={!!errors.symbol}
										helperText={errors.symbol ? "This field is required" : ""}
									/>
								</Box>
							</Grid>
							<Grid item xs={12} md={6}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										variant="outlined"
										label="Decimals"
										type="number"
										{...register("decimals", { required: true, valueAsNumber: true })}
										error={!!errors.decimals}
										helperText={errors.decimals ? "This field is required" : ""}
									/>
								</Box>
							</Grid>
							<Grid item xs={12} md={6}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										variant="outlined"
										label="Fund Token ID (optional)"
										{...register("fund_token_id", { setValueAs: (val: string) => val || undefined })}
									/>
								</Box>
							</Grid>
							<Grid item xs={12} md={6}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										variant="outlined"
										label="Market Token ID (optional)"
										{...register("market_token_id", { setValueAs: (val: string) => val || undefined })}
									/>
								</Box>
							</Grid>
						</Grid>
					</Box>

					<Divider sx={styles.formDivider} />

					{/* Metadata Section */}
					<Box sx={styles.formSection}>
						<Box sx={styles.formSectionHeader}>
							<DataObjectIcon />
							<Typography variant="h6">Token Metadata</Typography>
						</Box>
						<Grid container spacing={3}>
							<Grid item xs={12}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										variant="outlined"
										label="Metadata URL"
										{...register("metadata_url", { required: true })}
										error={!!errors.metadata_url}
										value={metadataUrl || ""}
										helperText={
											errors.metadata_url ? "This field is required" : "Enter a URL to fetch metadata or generate it below"
										}
										InputLabelProps={{ shrink: !!metadataUrl }}
									/>
								</Box>
							</Grid>
							<Grid item xs={12}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										variant="outlined"
										label="Metadata Hash (optional)"
										{...register("metadata_hash", { setValueAs: (val: string) => val || undefined })}
										InputLabelProps={{ shrink: !!watch("metadata_hash") }}
									/>
								</Box>
							</Grid>
							<Grid item xs={12}>
								<Accordion
									expanded={expanded}
									onChange={() => setExpanded(!expanded)}
									sx={{
										mb: 2,
										boxShadow: "none",
										border: (theme) => `1px solid ${theme.palette.divider}`,
										"&:before": {
											display: "none",
										},
										borderRadius: "4px",
									}}
								>
									<AccordionSummary
										expandIcon={<ExpandMoreIcon />}
										aria-controls="token-metadata-form-content"
										id="token-metadata-form-header"
										sx={{
											backgroundColor: (theme) => theme.palette.background.default,
											borderRadius: "4px",
										}}
									>
										<Typography sx={{ display: "flex", alignItems: "center" }}>
											<DataObjectIcon sx={{ mr: 1 }} fontSize="small" />
											Token Metadata Editor
										</Typography>
									</AccordionSummary>
									<AccordionDetails>
										<MetadataEditor
											defaultMetadata={{
												name: project.name,
												symbol: symbol,
												decimals: decimals,
												description: project.desc_long,
											}}
											metadataUrl={metadataUrl}
											fileBaseUrl={fileBaseUrl}
											onMetadataSubmit={handleMetadataSubmit}
										/>
									</AccordionDetails>
								</Accordion>
							</Grid>
						</Grid>
					</Box>

					<Box sx={styles.formActions}>
						<Button
							type="submit"
							variant="contained"
							color="primary"
							sx={styles.formSubmitButton}
							disabled={!contractsConfig || !walletApi}
						>
							Create Contract
						</Button>
					</Box>
				</Paper>
			</Container>
		</>
	);
};

export default ProjectContractCreate;
