import {
	Typography,
	Button,
	Dialog,
	DialogActions,
	DialogContent,
	DialogTitle,
	TextField,
	Box,
	Paper,
	Table,
	TableBody,
	TableCell,
	TableContainer,
	TableHead,
	TableRow,
	TablePagination,
	List,
	Alert,
	ListItem,
	ListItemText,
	IconButton,
	Icon,
} from "@mui/material";
import Grid from "@mui/material/Grid";
import DeleteIcon from "@mui/icons-material/Delete";
import WarningIcon from "@mui/icons-material/Warning";
import CheckCircleIcon from "@mui/icons-material/CheckCircle";
import RefreshIcon from "@mui/icons-material/Refresh";
import { useEffect, useState } from "react";
import {
	SecurityMintFund,
	TokenMetadata,
	ForestProjectService,
	ForestProjectTokenContract,
	SecurityMintFundState,
	PagedResponse_ForestProjectFundInvestor,
	ForestProjectFundInvestor,
	IndexerService,
	Agent,
} from "../../../apiClient";
import TransactionButton from "../../../components/TransactionButton";
import { TxnStatus, updateContract } from "../../../lib/concordium";
import { User } from "../../../lib/user";
import securityMintFund from "../../../contractClients/generated/securityMintFund";
import { toTokenId } from "../../../lib/conversions";
import { useForm } from "react-hook-form";
import securitySftMulti from "../../../contractClients/generated/securitySftMulti";

interface FundDetailsProps {
	fund: SecurityMintFund;
	// Security Token Contract
	investmentTokenContract?: ForestProjectTokenContract;
	// Presale Token Contract
	tokenContract?: ForestProjectTokenContract;
	currencyMetadata?: TokenMetadata;
	user: User;
	onRefresh: () => void;
}

export default function FundDetails({ fund, user, onRefresh }: FundDetailsProps) {
	const [fundTokenMetadata, setFundTokenMetadata] = useState<ForestProjectTokenContract>();
	const [fundCurrencyMetadata, setFundCurrencyMetadata] = useState<TokenMetadata>();
	const [deleteTxnStatus, setDeleteTxnStatus] = useState<TxnStatus>("none");
	const [markFailedTxnStatus, setMarkFailedTxnStatus] = useState<TxnStatus>("none");
	const [markSuccessTxnStatus, setMarkSuccessTxnStatus] = useState<TxnStatus>("none");
	const [openSuccessPopup, setOpenSuccessPopup] = useState(false);
	const [investors, setInvestors] = useState<PagedResponse_ForestProjectFundInvestor>();
	const [investorsPage, setInvestorsPage] = useState(0);
	const [investorsRowsPerPage, setInvestorsRowsPerPage] = useState(10);
	const [claimTxnStatus, setClaimTxnStatus] = useState<TxnStatus>("none");
	const [refreshCounter, setRefreshCounter] = useState(0);
	const [agentPresaleContract, setAgentPresaleContract] = useState<Agent>();
	const [agentInvestmentContract, setAgentInvestmentContract] = useState<Agent>();

	const [addAgentPreSaleTxnStatus, setAddAgentPreSaleTxnStatus] = useState<TxnStatus>("none");
	const [addAgentInvestmentTxnStatus, setAddAgentInvestmentTxnStatus] = useState<TxnStatus>("none");
	const [removeAgentPreSaleTxnStatus, setRemoveAgentPreSaleTxnStatus] = useState<TxnStatus>("none");
	const [removeAgentInvestmentTxnStatus, setRemoveAgentInvestmentTxnStatus] = useState<TxnStatus>("none");

	const {
		register,
		handleSubmit,
		formState: { errors },
	} = useForm<{ receiverAccountAddress: string }>({
		defaultValues: {
			receiverAccountAddress: user.concordiumAccountAddress,
		},
	});

	useEffect(() => {
		ForestProjectService.getAdminForestProjectsContract(fund.token_contract_address)
			.then(setFundTokenMetadata)
			.catch(() => {
				setFundTokenMetadata(undefined);
			});
		ForestProjectService.getAdminTokenMetadata(fund.currency_token_contract_address, fund.currency_token_id)
			.then(setFundCurrencyMetadata)
			.catch(() => {
				setFundCurrencyMetadata(undefined);
			});
		IndexerService.getAdminIndexerCis2Agent(fund.token_contract_address, fund.contract_address, true).then(
			setAgentPresaleContract,
		);
		IndexerService.getAdminIndexerCis2Agent(fund.investment_token_contract_address, fund.contract_address, true).then(
			setAgentInvestmentContract,
		);
	}, [fund, refreshCounter]);
	useEffect(() => {
		if (fund) {
			ForestProjectService.getAdminForestProjectsFundInvestorList(
				investorsPage,
				undefined,
				fund.investment_token_id,
				fund.investment_token_contract_address,
				investorsRowsPerPage,
			).then(setInvestors);
		}
	}, [fund, investorsPage, investorsRowsPerPage, refreshCounter]);

	const markFailed = async () => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				fund.contract_address,
				securityMintFund.updateFundState,
				{
					state: {
						Fail: {},
					},
					security_token: {
						id: toTokenId(BigInt(fund.investment_token_id), 8),
						contract: {
							index: Number(fund.investment_token_contract_address),
							subindex: 0,
						},
					},
				},
				setMarkFailedTxnStatus,
			);
			setMarkFailedTxnStatus("success");
			alert("Marked failed successfully");
			onRefresh();
		} catch (e) {
			console.error(e);
			setMarkFailedTxnStatus("error");
			alert(`Failed to mark failed: ${JSON.stringify(e)}`);
		}
	};

	const deleteFund = async () => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				fund.contract_address,
				securityMintFund.removeFund,
				{
					id: toTokenId(BigInt(fund.investment_token_id), 8),
					contract: {
						index: Number(fund.investment_token_contract_address),
						subindex: 0,
					},
				},
				setDeleteTxnStatus,
			);
			setDeleteTxnStatus("success");
			alert("Deletion successfully");
			onRefresh();
		} catch (e) {
			console.error(e);
			setDeleteTxnStatus("error");
			alert(`Failed to delete: ${JSON.stringify(e)}`);
		}
	};

	const markSuccess = async (data: { receiverAccountAddress: string }) => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				fund.contract_address,
				securityMintFund.updateFundState,
				{
					state: {
						Success: [
							{
								Account: [data.receiverAccountAddress],
							},
						],
					},
					security_token: {
						id: toTokenId(BigInt(fund.investment_token_id), 8),
						contract: {
							index: Number(fund.investment_token_contract_address),
							subindex: 0,
						},
					},
				},
				setMarkSuccessTxnStatus,
			);
			setMarkSuccessTxnStatus("success");
			alert("Marked success successfully");
			setOpenSuccessPopup(false);
			onRefresh();
		} catch (e) {
			console.error(e);
			setMarkSuccessTxnStatus("error");
			alert(`Failed to mark success: ${JSON.stringify(e)}`);
		}
	};

	const handleChangePage = (_event: unknown, newPage: number) => {
		setInvestorsPage(newPage);
	};

	const handleChangeRowsPerPage = (event: React.ChangeEvent<HTMLInputElement>) => {
		setInvestorsRowsPerPage(parseInt(event.target.value, 10));
		setInvestorsPage(0);
	};

	const claimInvestment = async (investor: ForestProjectFundInvestor) => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				fund.contract_address,
				securityMintFund.claimInvestment,
				{
					investments: [
						{
							investor: investor.investor_account_address,
							security_token: {
								id: toTokenId(BigInt(fund.investment_token_id), 8),
								contract: {
									index: Number(fund.investment_token_contract_address),
									subindex: 0,
								},
							},
						},
					],
				},
				setClaimTxnStatus,
			);
			setClaimTxnStatus("success");
			alert("Investment claimed successfully");
			setRefreshCounter((c) => c + 1);
		} catch (e) {
			console.error(e);
			setClaimTxnStatus("error");
			alert(`Failed to claim investment: ${JSON.stringify(e)}`);
		}
	};

	const handleCloseSuccessPopup = () => {
		setOpenSuccessPopup(false);
		setRefreshCounter((c) => c + 1);
	};

	const addAgentPresaleContract = async () => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				fund.token_contract_address,
				securitySftMulti.addAgent,
				{
					address: { Contract: [{ index: Number(fund.contract_address), subindex: 0 }] },
					roles: [{ Mint: {} }, { Operator: {} }, { ForcedBurn: {} }],
				},
				setAddAgentPreSaleTxnStatus,
			);
			setAddAgentPreSaleTxnStatus("success");
			alert("Agent added successfully");
			setRefreshCounter((c) => c + 1);
		} catch (e) {
			console.error(e);
			setAddAgentPreSaleTxnStatus("error");
			alert(`Failed to add agent`);
		}
	};
	const removeAgentPresaleContract = async () => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				fund.token_contract_address,
				securitySftMulti.removeAgent,
				{ Contract: [{ index: Number(fund.contract_address), subindex: 0 }] },
				setRemoveAgentPreSaleTxnStatus,
			);
			setRemoveAgentPreSaleTxnStatus("success");
			alert("Agent removed successfully");
			setRefreshCounter((c) => c + 1);
		} catch (e) {
			console.error(e);
			setRemoveAgentPreSaleTxnStatus("error");
			alert(`Failed to remove agent`);
		}
	};
	const addAgentInvestmentContract = async () => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				fund.investment_token_contract_address,
				securitySftMulti.addAgent,
				{
					address: { Contract: [{ index: Number(fund.contract_address), subindex: 0 }] },
					roles: [{ Mint: {} }, { Operator: {} }, { ForcedBurn: {} }],
				},
				setAddAgentInvestmentTxnStatus,
			);
			setAddAgentInvestmentTxnStatus("success");
			alert("Agent added successfully");
			setRefreshCounter((c) => c + 1);
		} catch (e) {
			console.error(e);
			setAddAgentInvestmentTxnStatus("error");
			alert(`Failed to add agent`);
		}
	};
	const removeAgentInvestmentContract = async () => {
		try {
			await updateContract(
				user.concordiumAccountAddress,
				fund.investment_token_contract_address,
				securitySftMulti.removeAgent,
				{ Contract: [{ index: Number(fund.contract_address), subindex: 0 }] },
				setRemoveAgentInvestmentTxnStatus,
			);
			setRemoveAgentInvestmentTxnStatus("success");
			alert("Agent removed successfully");
			setRefreshCounter((c) => c + 1);
		} catch (e) {
			console.error(e);
			setRemoveAgentInvestmentTxnStatus("error");
			alert(`Failed to remove agent`);
		}
	};

	return (
		<>
			<Paper variant="outlined" sx={{ padding: 2, marginBottom: 2 }}>
				<Grid container spacing={2}>
					<Grid item xs={12} md={4}>
						<Typography variant="h6">Fund Details</Typography>
					</Grid>
					<Grid item xs={12} md={8} justifyContent="flex-end" display="flex">
						<TransactionButton
							variant="outlined"
							color="warning"
							startIcon={<WarningIcon />}
							txnStatus={markFailedTxnStatus}
							defaultText="Mark Failed"
							loadingText="Marking Failed..."
							onClick={markFailed}
							disabled={!(fund.fund_state === SecurityMintFundState.OPEN)}
						/>
						<Button
							variant="outlined"
							color="success"
							startIcon={<CheckCircleIcon />}
							onClick={() => setOpenSuccessPopup(true)}
							disabled={!(fund.fund_state === SecurityMintFundState.OPEN)}
						>
							Mark Success
						</Button>
						<TransactionButton
							variant="outlined"
							color="error"
							startIcon={<DeleteIcon />}
							txnStatus={deleteTxnStatus}
							defaultText="Delete"
							loadingText="Deleting..."
							onClick={deleteFund}
						/>
						<IconButton onClick={onRefresh} color="primary">
							<Icon>
								<RefreshIcon />
							</Icon>
						</IconButton>
					</Grid>
					<Grid item xs={12} md={6}>
						<Typography>
							<strong>Contract Address:</strong> {fund.contract_address}
						</Typography>
						<Typography>
							<strong>Investment Token ID:</strong> {fund.investment_token_id}
						</Typography>
						<Typography>
							<strong>Investment Token Contract Address:</strong> {fund.investment_token_contract_address}
						</Typography>
						<Typography>
							<strong>Token ID:</strong> {fund.token_id}
						</Typography>
						<Typography>
							<strong>Token Contract Address:</strong> {fund.token_contract_address}
						</Typography>
						{fundTokenMetadata && (
							<>
								<Typography>
									<strong>Token Symbol:</strong> {fundTokenMetadata.symbol}
								</Typography>
								<Typography>
									<strong>Token Decimals:</strong> {fundTokenMetadata.decimals}
								</Typography>
							</>
						)}
						<Typography>
							<strong>Token Amount:</strong> {fund.token_amount}
						</Typography>
						<Typography>
							<strong>Currency Token ID:</strong> {fund.currency_token_id}
						</Typography>
						<Typography>
							<strong>Currency Token Contract Address:</strong> {fund.currency_token_contract_address}
						</Typography>
						<Typography>
							<strong>Currency Amount:</strong> {fund.currency_amount}
						</Typography>
						{fundCurrencyMetadata && (
							<>
								<Typography>
									<strong>Currency Symbol:</strong> {fundCurrencyMetadata.symbol}
								</Typography>
								<Typography>
									<strong>Currency Decimals:</strong> {fundCurrencyMetadata.decimals}
								</Typography>
							</>
						)}
						<Typography>
							<strong>Token Amount:</strong> {fund.token_amount}
						</Typography>
						<Typography>
							<strong>Receiver Address:</strong> {fund.receiver_address || "N/A"}
						</Typography>
						<Typography>
							<strong>Rate:</strong> {fund.rate_numerator} / {fund.rate_denominator}
						</Typography>
						<Typography>
							<strong>Fund State:</strong> {fund.fund_state}
						</Typography>
						<Typography>
							<strong>Create Time:</strong> {fund.create_time}
						</Typography>
						<Typography>
							<strong>Update Time:</strong> {fund.update_time}
						</Typography>
					</Grid>
					<Grid item xs={12} md={6} id="fund-checks-section">
						<Grid container spacing={2} direction={"row-reverse"}>
							<Grid item xs={12} md={12} lg={6}>
								{agentPresaleContract && (
									<Alert severity="success">
										<Typography>
											Fund contract is an agent of the presale token contract. With the roles{" "}
											{agentPresaleContract.roles.join(", ")}
										</Typography>
										<TransactionButton
											txnStatus={removeAgentPreSaleTxnStatus}
											defaultText="Remove Agent"
											loadingText="Removing Agent..."
											variant="outlined"
											color="primary"
											onClick={removeAgentPresaleContract}
											sx={{ mt: 2 }}
										/>
									</Alert>
								)}
								{!agentPresaleContract && (
									<Alert severity="warning" sx={{ mb: 2 }}>
										<Typography>Fund contract is not an agent of the presale token contract.</Typography>
										<TransactionButton
											txnStatus={addAgentPreSaleTxnStatus}
											defaultText="Add Agent"
											loadingText="Adding Agent..."
											variant="outlined"
											color="primary"
											onClick={addAgentPresaleContract}
											sx={{ mt: 2 }}
										/>
									</Alert>
								)}
							</Grid>
							<Grid item xs={12} md={12} lg={6}>
								{agentInvestmentContract && (
									<Alert severity="success">
										<Typography>
											Fund contract is an agent of the investment token contract. With the roles{" "}
											{agentInvestmentContract.roles.join(", ")}
										</Typography>
										<TransactionButton
											txnStatus={removeAgentInvestmentTxnStatus}
											defaultText="Remove Agent"
											loadingText="Removing Agent..."
											variant="outlined"
											color="primary"
											onClick={removeAgentInvestmentContract}
											sx={{ mt: 2 }}
										/>
									</Alert>
								)}

								{!agentInvestmentContract && (
									<Alert severity="warning">
										<Typography>Fund contract is not an agent of the investment token contract.</Typography>
										<TransactionButton
											txnStatus={addAgentInvestmentTxnStatus}
											defaultText="Add Agent"
											loadingText="Adding Agent..."
											variant="outlined"
											color="primary"
											onClick={addAgentInvestmentContract}
											sx={{ mt: 2 }}
										/>
									</Alert>
								)}
							</Grid>
							<Grid item xs={12} md={12} lg={6}>
								{
									{
										[SecurityMintFundState.OPEN]: (
											<Alert severity="info">
												<Typography>This fund is open for investment.</Typography>
											</Alert>
										),
										[SecurityMintFundState.SUCCESS]: (
											<Alert severity="success">
												<Typography>This fund has been marked as successful.</Typography>
											</Alert>
										),
										[SecurityMintFundState.FAIL]: (
											<Alert severity="error">
												<Typography>This fund has been marked as failed.</Typography>
											</Alert>
										),
									}[fund.fund_state]
								}
							</Grid>
						</Grid>
					</Grid>
				</Grid>
			</Paper>
			{/* Table of investors */}
			<Paper variant="outlined" sx={{ padding: 2, marginBottom: 2 }}>
				<Typography variant="h6">Investors</Typography>
				<TableContainer component={Paper}>
					<Table>
						<TableHead>
							<TableRow>
								<TableCell>Investor Email</TableCell>
								<TableCell>Investor Account Address</TableCell>
								<TableCell>Investment Token Amount</TableCell>
								<TableCell>Investment Currency Amount</TableCell>
								<TableCell>Return/Claim Investment</TableCell>
							</TableRow>
						</TableHead>
						<TableBody>
							{investors?.data.map((investor) => (
								<TableRow key={investor.investor_account_address}>
									<TableCell>{investor.investor_email}</TableCell>
									<TableCell>{investor.investor_account_address}</TableCell>
									<TableCell>{investor.investment_token_amount}</TableCell>
									<TableCell>{investor.investment_currency_amount}</TableCell>
									<TableCell>
										<TransactionButton
											txnStatus={claimTxnStatus}
											defaultText="Return/Claim"
											loadingText="Processing"
											disabled={fund.fund_state === SecurityMintFundState.OPEN}
											onClick={() => claimInvestment(investor)}
										/>
									</TableCell>
								</TableRow>
							))}
						</TableBody>
					</Table>
				</TableContainer>
				<TablePagination
					component="div"
					count={investors?.page_count || 0}
					page={investorsPage}
					onPageChange={handleChangePage}
					rowsPerPage={investorsRowsPerPage}
					onRowsPerPageChange={handleChangeRowsPerPage}
				/>
			</Paper>
			<Dialog open={openSuccessPopup} onClose={handleCloseSuccessPopup} fullWidth>
				<DialogTitle>Mark Fund as Success</DialogTitle>
				<DialogContent>
					<Box
						component="form"
						onSubmit={handleSubmit(markSuccess)}
						sx={{ display: "flex", flexDirection: "column", gap: 2, pt: 2 }}
					>
						<TextField
							label="Receiver Account Address"
							{...register("receiverAccountAddress", { required: true })}
							defaultValue={user.concordiumAccountAddress}
							error={!!errors.receiverAccountAddress}
							helperText={errors.receiverAccountAddress ? "This field is required" : ""}
						/>
						<DialogActions>
							<Button onClick={handleCloseSuccessPopup} color="primary">
								Cancel
							</Button>
							<TransactionButton
								variant="contained"
								color="success"
								startIcon={<CheckCircleIcon />}
								txnStatus={markSuccessTxnStatus}
								defaultText="Mark Success"
								loadingText="Marking Success..."
								type="submit"
							/>
						</DialogActions>
					</Box>
				</DialogContent>
			</Dialog>
		</>
	);
}
