import {
	ButtonGroup,
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
} from "@mui/material";
import DeleteIcon from "@mui/icons-material/Delete";
import WarningIcon from "@mui/icons-material/Warning";
import CheckCircleIcon from "@mui/icons-material/CheckCircle";
import { useEffect, useState } from "react";
import {
	SecurityMintFund,
	TokenMetadata,
	ForestProjectService,
	ForestProjectTokenContract,
	SecurityMintFundState,
	PagedResponse_ForestProjectFundInvestor_,
	ForestProjectFundInvestor,
} from "../../../apiClient";
import TransactionButton from "../../../components/TransactionButton";
import { TxnStatus, updateContract } from "../../../lib/concordium";
import { User } from "../../../lib/user";
import { useOutletContext } from "react-router";
import securityMintFund from "../../../contractClients/generated/securityMintFund";
import { toTokenId } from "../../../lib/conversions";
import { useForm } from "react-hook-form";

interface FundDetailsProps {
	fund: SecurityMintFund;
	investmentTokenContract?: ForestProjectTokenContract;
	tokenContract?: ForestProjectTokenContract;
	currencyMetadata?: TokenMetadata;
}

export default function FundDetails({ fund }: FundDetailsProps) {
	const { user } = useOutletContext<{ user: User }>();
	const [fundTokenMetadata, setFundTokenMetadata] = useState<ForestProjectTokenContract>();
	const [fundCurrencyMetadata, setFundCurrencyMetadata] = useState<TokenMetadata>();
	const [deleteTxnStatus, setDeleteTxnStatus] = useState<TxnStatus>("none");
	const [markFailedTxnStatus, setMarkFailedTxnStatus] = useState<TxnStatus>("none");
	const [markSuccessTxnStatus, setMarkSuccessTxnStatus] = useState<TxnStatus>("none");
	const [openSuccessPopup, setOpenSuccessPopup] = useState(false);
	const [investors, setInvestors] = useState<PagedResponse_ForestProjectFundInvestor_>();
	const [investorsPage, setInvestorsPage] = useState(0);
	const [investorsRowsPerPage, setInvestorsRowsPerPage] = useState(10);
	const [claimTxnStatus, setClaimTxnStatus] = useState<TxnStatus>("none");

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
			.then((metadata) => {
				setFundTokenMetadata(metadata);
			})
			.catch(() => {
				setFundTokenMetadata(undefined);
			});
		ForestProjectService.getAdminTokenMetadata(fund.currency_token_contract_address, fund.currency_token_id)
			.then((metadata) => {
				setFundCurrencyMetadata(metadata);
			})
			.catch(() => {
				setFundCurrencyMetadata(undefined);
			});
	}, [fund]);
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
	}, [fund, investorsPage, investorsRowsPerPage]);

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
			window.location.reload();
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
			window.location.reload();
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
			window.location.reload();
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
			window.location.reload();
		} catch (e) {
			console.error(e);
			setClaimTxnStatus("error");
			alert(`Failed to claim investment: ${JSON.stringify(e)}`);
		}
	};
	return (
		<>
			<Paper variant="outlined" sx={{ padding: 2, marginBottom: 2 }}>
				<div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
					<Typography variant="h6">Fund Details</Typography>
					<ButtonGroup>
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
					</ButtonGroup>
				</div>
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
			<Dialog open={openSuccessPopup} onClose={() => setOpenSuccessPopup(false)} fullWidth>
				<DialogTitle>Mark Fund as Success</DialogTitle>
				<DialogContent>
					<Box
						component="form"
						onSubmit={handleSubmit(markSuccess)}
						sx={{ display: "flex", flexDirection: "column", gap: 2 }}
					>
						<TextField
							label="Receiver Account Address"
							{...register("receiverAccountAddress", { required: true })}
							defaultValue={user.concordiumAccountAddress}
							error={!!errors.receiverAccountAddress}
							helperText={errors.receiverAccountAddress ? "This field is required" : ""}
						/>
						<DialogActions>
							<Button onClick={() => setOpenSuccessPopup(false)} color="primary">
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
