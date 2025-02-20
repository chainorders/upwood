import { useNavigate, useOutletContext, useParams } from "react-router";
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
} from "@mui/material";
import { formatDate, parseFinalizedInit } from "../../lib/conversions";
import securitySftMulti from "../../contractClients/generated/securitySftMulti";
import { detectConcordiumProvider, WalletApi } from "@concordium/browser-wallet-api-helpers";
import { AccountAddress, TransactionHash } from "@concordium/web-sdk";
import { useEffect, useState } from "react";
import { User } from "../../lib/user";
import concordiumNodeClient from "../../contractClients/ConcordiumNodeClient";
import CircularProgress from "@mui/material/CircularProgress";
import { Link } from "react-router";

export default function ProjectContractCreate() {
	const { id } = useParams<{ id: string }>();
	const { user } = useOutletContext<{ user: User }>();
	const navigate = useNavigate();

	const [contractsConfig, setContractsConfig] = useState<SystemContractsConfigApiModel | null>(null);
	const [walletApi, setWalletApi] = useState<WalletApi>();
	const [txnStatus, setTxnStatus] = useState<"sending" | "waiting" | "success" | "error" | "none">("none");
	const [project, setProject] = useState<ForestProject | null>(null);

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
	}, [walletApi]);

	useEffect(() => {
		if (id) {
			ForestProjectService.getAdminForestProjects(id).then(setProject);
		}
	}, [id]);

	const {
		register,
		handleSubmit,
		setValue,
		formState: { errors },
		watch,
	} = useForm<ForestProjectTokenContract>();

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
				// TODO: wait for indexer to index the contract
				break;
			}
			case "error": {
				setTxnStatus("error");
				alert(`Failed to initialize contract: ${txnResult.value.rejectReason}`);
				break;
			}
		}
	};

	const contractAddress = watch("contract_address");

	if (!project) {
		return <div>Loading...</div>;
	}

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
					<TextField
						label="Metadata URL"
						{...register("metadata_url", { required: true })}
						error={!!errors.metadata_url}
						helperText={errors.metadata_url ? "This field is required" : ""}
					/>
					<TextField
						label="Metadata Hash (optional)"
						{...register("metadata_hash", { setValueAs: (val: string) => val || undefined })}
					/>
					<Button type="submit" variant="contained" color="primary" disabled={!contractsConfig || !walletApi}>
						Create Contract
					</Button>
				</Box>
			</div>
		</>
	);
}
