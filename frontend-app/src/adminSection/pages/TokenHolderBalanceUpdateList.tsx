import { useEffect, useState } from "react";
import { useParams } from "react-router";
import {
	Box,
	Paper,
	Table,
	TableBody,
	TableCell,
	TableContainer,
	TableHead,
	TableRow,
	Typography,
	TablePagination,
	Chip,
} from "@mui/material";
import { IndexerService, PagedResponse_TokenHolderBalanceUpdate, TokenHolderBalanceUpdate } from "../../apiClient";
import { format } from "date-fns";

export default function TokenHolderBalanceUpdateList() {
	const { contract, token_id, holder } = useParams<{
		contract: string;
		token_id: string;
		holder: string;
	}>();

	const [page, setPage] = useState<number>(0);
	const [pageSize, setPageSize] = useState<number>(20);
	const [balanceUpdates, setBalanceUpdates] = useState<PagedResponse_TokenHolderBalanceUpdate>({
		data: [],
		page: 0,
		page_count: 0,
	});
	const [loading, setLoading] = useState<boolean>(true);

	useEffect(() => {
		if (!contract || !token_id || !holder) return;

		setLoading(true);
		IndexerService.getAdminIndexerCis2TokenHolderBalanceUpdates(contract, token_id, holder, page, pageSize)
			.then((response) => {
				setBalanceUpdates(response);
			})
			.catch((error) => {
				console.error("Error fetching balance updates:", error);
			})
			.finally(() => {
				setLoading(false);
			});
	}, [contract, token_id, holder, page, pageSize]);

	const handlePageChange = (event: unknown, newPage: number) => {
		setPage(newPage);
	};

	const handleRowsPerPageChange = (event: React.ChangeEvent<HTMLInputElement>) => {
		setPageSize(parseInt(event.target.value, 10));
		setPage(0);
	};

	const getUpdateTypeColor = (type: string) => {
		switch (type.toUpperCase()) {
			case "MINT":
				return "success";
			case "BURN":
				return "error";
			case "TRANSFER":
				return "primary";
			case "FREEZE":
				return "warning";
			case "UNFREEZE":
				return "info";
			default:
				return "default";
		}
	};

	const formatDate = (dateString: string) => {
		try {
			return format(new Date(dateString), "yyyy-MM-dd HH:mm:ss");
		} catch (e) {
			return dateString;
		}
	};

	return (
		<Box sx={{ p: 3 }}>
			<Typography variant="h4" component="h1" gutterBottom>
				Balance Updates for Holder
			</Typography>
			<Typography variant="subtitle1" gutterBottom>
				Contract: {contract} | Token ID: {token_id} | Holder: {holder}
			</Typography>

			<TableContainer component={Paper} sx={{ mt: 3 }}>
				<Table aria-label="balance updates table">
					<TableHead>
						<TableRow>
							<TableCell>ID</TableCell>
							<TableCell>Block Height</TableCell>
							<TableCell>Transaction Index</TableCell>
							<TableCell>Amount</TableCell>
							<TableCell>Frozen Balance</TableCell>
							<TableCell>Unfrozen Balance</TableCell>
							<TableCell>Update Type</TableCell>
							<TableCell>Transaction Sender</TableCell>
							<TableCell>Created At</TableCell>
						</TableRow>
					</TableHead>
					<TableBody>
						{loading ? (
							<TableRow>
								<TableCell colSpan={9} align="center">
									Loading...
								</TableCell>
							</TableRow>
						) : balanceUpdates.data.length > 0 ? (
							balanceUpdates.data.map((update: TokenHolderBalanceUpdate) => (
								<TableRow key={update.id}>
									<TableCell>{update.id_serial || update.id}</TableCell>
									<TableCell>{update.block_height}</TableCell>
									<TableCell>{update.txn_index}</TableCell>
									<TableCell>{update.amount}</TableCell>
									<TableCell>{update.frozen_balance}</TableCell>
									<TableCell>{update.un_frozen_balance}</TableCell>
									<TableCell>
										<Chip label={update.update_type} color={getUpdateTypeColor(update.update_type)} size="small" />
									</TableCell>
									<TableCell title={update.txn_instigator}>{update.txn_sender}</TableCell>
									<TableCell>{formatDate(update.create_time)}</TableCell>
								</TableRow>
							))
						) : (
							<TableRow>
								<TableCell colSpan={9} align="center">
									No balance updates found
								</TableCell>
							</TableRow>
						)}
					</TableBody>
				</Table>
				<TablePagination
					component="div"
					count={balanceUpdates.page_count * pageSize} // Approximate total count
					page={page}
					onPageChange={handlePageChange}
					rowsPerPage={pageSize}
					onRowsPerPageChange={handleRowsPerPageChange}
					rowsPerPageOptions={[10, 20, 50, 100]}
				/>
			</TableContainer>
		</Box>
	);
}
