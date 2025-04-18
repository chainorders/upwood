import { useEffect, useState } from "react";
import {
	TableContainer,
	Table,
	TableHead,
	TableRow,
	TableCell,
	TableBody,
	Paper,
	TablePagination,
	Box,
	CircularProgress,
} from "@mui/material";
import { PagedResponse_Token, IndexerService } from "../../apiClient";
import useCommonStyles from "../../theme/useCommonStyles";
import { toDisplayAmount } from "../../lib/conversions";
import { format } from "date-fns";

function formatDateField(dateStr?: string) {
	if (!dateStr) return "-";
	const date = new Date(dateStr);
	return isNaN(date.getTime()) ? "-" : format(date, "yyyy-MM-dd HH:mm:ss");
}

interface TokensTableProps {
	contract_index: string;
}

const TokensTable: React.FC<TokensTableProps> = ({ contract_index }) => {
	const classes = useCommonStyles();
	const [tokens, setTokens] = useState<PagedResponse_Token>();
	const [page, setPage] = useState(0);
	const [pageSize, setPageSize] = useState(10);
	const [loading, setLoading] = useState(false);

	useEffect(() => {
		if (!contract_index) return;
		setLoading(true);
		IndexerService.getAdminIndexerTokens(page, pageSize, contract_index)
			.then(setTokens)
			.finally(() => setLoading(false));
	}, [contract_index, page, pageSize]);

	if (loading) return <CircularProgress />;

	return (
		<Box>
			<TableContainer component={Paper} sx={classes.tableContainer}>
				<Table>
					<TableHead>
						<TableRow>
							<TableCell sx={classes.tableHeaderCell}>Token ID</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Paused</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Metadata URL</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Metadata Hash</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Supply</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Created</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Updated</TableCell>
						</TableRow>
					</TableHead>
					<TableBody>
						{tokens?.data?.length ? (
							tokens.data.map((token) => (
								<TableRow key={token.token_id}>
									<TableCell>{token.token_id}</TableCell>
									<TableCell>{token.is_paused ? "Yes" : "No"}</TableCell>
									<TableCell>{token.metadata_url}</TableCell>
									<TableCell>{token.metadata_hash || "-"}</TableCell>
									<TableCell>{toDisplayAmount(token.supply, 0)}</TableCell>
									<TableCell>{formatDateField(token.create_time)}</TableCell>
									<TableCell>{formatDateField(token.update_time)}</TableCell>
								</TableRow>
							))
						) : (
							<TableRow>
								<TableCell colSpan={7} align="center">
									No tokens found.
								</TableCell>
							</TableRow>
						)}
					</TableBody>
				</Table>
			</TableContainer>
			<TablePagination
				component="div"
				count={tokens?.page_count || 0 * pageSize}
				page={page}
				onPageChange={(_, newPage) => setPage(newPage)}
				rowsPerPage={pageSize}
				onRowsPerPageChange={(e) => {
					setPageSize(parseInt(e.target.value, 10));
					setPage(0);
				}}
				rowsPerPageOptions={[10, 20, 50]}
			/>
		</Box>
	);
};

export default TokensTable;
