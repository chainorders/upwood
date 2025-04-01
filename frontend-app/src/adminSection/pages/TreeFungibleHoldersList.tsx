import React, { useEffect, useState } from "react";
import {
	Box,
	Card,
	CardContent,
	CircularProgress,
	Paper,
	Table,
	TableBody,
	TableCell,
	TableContainer,
	TableHead,
	TablePagination,
	TableRow,
	Typography,
} from "@mui/material";
import { UserService } from "../../apiClient/services/UserService";
import { TreeNftService } from "../../apiClient/services/TreeNftService";
import { UserTokenHolder } from "../../apiClient/models/UserTokenHolder";
import { NftMultiRewardedDetails } from "../../apiClient/models/NftMultiRewardedDetails";
import { SystemContractsConfigApiModel, TreeNftMetadataService } from "../../apiClient";
import TransactionButton from "../../components/TransactionButton";
import { TxnStatus, updateContract } from "../../lib/concordium";
import { User } from "../../lib/user";
import nftMultiRewarded from "../../contractClients/generated/nftMultiRewarded";

export default function TreeFungibleHoldersList({ user }: { user: User }) {
	const [page, setPage] = useState(0);
	const [rowsPerPage, setRowsPerPage] = useState(10);
	const [loading, setLoading] = useState(true);
	const [holders, setHolders] = useState<UserTokenHolder[]>([]);
	const [totalCount, setTotalCount] = useState(0);
	const [nftContractDetails, setNftContractDetails] = useState<NftMultiRewardedDetails | null>(null);
	const [nftContractLoading, setNftContractLoading] = useState(true);
	const [contracts, setContracts] = useState<SystemContractsConfigApiModel>();
	const [tokenId, setTokenId] = useState<string>();
	const [contract, setContract] = useState<string>();
	const [txnStatus, setTxnStatus] = useState<TxnStatus>("none");
	const [refreshCounter, setRefreshCounter] = useState(0);

	useEffect(() => {
		UserService.getSystemConfig()
			.then((response) => {
				setContracts(response);
				setTokenId("0");
				setContract(response.tree_ft_contract_index);
			})
			.catch((error) => {
				console.error("Error fetching system contracts config:", error);
			});
	}, []);

	// Fetch token holders
	useEffect(() => {
		if (!tokenId || !contract) return;

		setLoading(true);
		UserService.getAdminHolderList(tokenId, contract, page, rowsPerPage)
			.then((response) => {
				setHolders(response.data || []);
				setTotalCount(response.page_count * rowsPerPage);
			})
			.catch((error) => {
				console.error("Error fetching token holders:", error);
			})
			.finally(() => {
				setLoading(false);
			});
	}, [tokenId, contract, page, rowsPerPage, refreshCounter]);

	// Fetch NFT contract details
	useEffect(() => {
		setNftContractLoading(true);
		TreeNftService.getAdminTreeNftContract()
			.then((details) => {
				setNftContractDetails(details);
			})
			.catch((error) => {
				console.error("Error fetching NFT contract details:", error);
			})
			.finally(() => {
				setNftContractLoading(false);
			});
	}, [refreshCounter]);

	const handleChangePage = (_event: unknown, newPage: number) => {
		setPage(newPage - 1);
	};

	const handleChangeRowsPerPage = (event: React.ChangeEvent<HTMLInputElement>) => {
		setRowsPerPage(parseInt(event.target.value, 10));
		setPage(0);
	};

	const handleConvertToNft = async (holder: UserTokenHolder) => {
		if (!contracts) return;

		// Placeholder for NFT conversion logic
		console.log(`Converting to NFT for user: ${holder.cognito_user_id}, token: ${holder.token_id}`);
		try {
			setTxnStatus("sending");
			const metadata = await TreeNftMetadataService.getTreeNftMetadataRandomUnsigned();
			await updateContract(
				user.concordiumAccountAddress,
				contracts?.tree_nft_contract_index,
				nftMultiRewarded.mintAgent,
				{
					account: holder.account_address,
					metadata_url: {
						url: metadata.url,
						hash: metadata.hash ? { Some: [metadata.hash] } : { None: {} },
					},
				},
				setTxnStatus,
			);
			setTxnStatus("success");
			setRefreshCounter(refreshCounter + 1);
		} catch (error) {
			console.error("Error converting to NFT:", error);
			setTxnStatus("error");
		}
	};

	return (
		<Box>
			<Typography variant="h4" gutterBottom>
				Token Holders for {tokenId} ({contract})
			</Typography>

			{/* NFT Contract Details Card */}
			<Card sx={{ mb: 3 }}>
				<CardContent>
					<Typography variant="h6" gutterBottom>
						NFT Contract Details
					</Typography>
					{nftContractLoading ? (
						<CircularProgress size={20} />
					) : nftContractDetails ? (
						<Box>
							<Typography variant="body1">Contract Address: {nftContractDetails.contract.contract_address}</Typography>
							<Typography variant="body1">Total Tokens: {nftContractDetails.tokens_count}</Typography>
							<Typography variant="body1">Unique Metadata Count: {nftContractDetails.unique_metdata_count}</Typography>
						</Box>
					) : (
						<Typography color="error">Error loading NFT contract details</Typography>
					)}
				</CardContent>
			</Card>

			{/* Token Holders Table */}
			<Paper>
				<TableContainer>
					<Table>
						<TableHead>
							<TableRow>
								<TableCell>User</TableCell>
								<TableCell>Email</TableCell>
								<TableCell>Account Address</TableCell>
								<TableCell>Frozen Balance</TableCell>
								<TableCell>Unfrozen Balance</TableCell>
								<TableCell>Total Balance</TableCell>
								<TableCell>Actions</TableCell>
							</TableRow>
						</TableHead>
						<TableBody>
							{loading ? (
								<TableRow>
									<TableCell colSpan={7} align="center">
										<CircularProgress />
									</TableCell>
								</TableRow>
							) : holders.length > 0 ? (
								holders.map((holder) => (
									<TableRow key={holder.cognito_user_id}>
										<TableCell>
											{holder.first_name} {holder.last_name}
										</TableCell>
										<TableCell>{holder.email}</TableCell>
										<TableCell>{holder.account_address}</TableCell>
										<TableCell>{holder.balance_frozen}</TableCell>
										<TableCell>{holder.balance_unfrozen}</TableCell>
										<TableCell>{holder.balance_total}</TableCell>
										<TableCell>
											<TransactionButton
												variant="contained"
												color="primary"
												onClick={() => handleConvertToNft(holder)}
												defaultText="Convert to NFT"
												loadingText="Converting"
												txnStatus={txnStatus}
											/>
										</TableCell>
									</TableRow>
								))
							) : (
								<TableRow>
									<TableCell colSpan={7} align="center">
										No token holders found
									</TableCell>
								</TableRow>
							)}
						</TableBody>
					</Table>
				</TableContainer>
				<TablePagination
					rowsPerPageOptions={[5, 10, 25]}
					component="div"
					count={totalCount}
					rowsPerPage={rowsPerPage}
					page={page}
					onPageChange={handleChangePage}
					onRowsPerPageChange={handleChangeRowsPerPage}
				/>
			</Paper>
		</Box>
	);
}
