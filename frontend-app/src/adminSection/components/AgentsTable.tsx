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
import { PagedResponse_Agent, IndexerService } from "../../apiClient";
import useCommonStyles from "../../theme/useCommonStyles";

interface AgentsTableProps {
	contract_index: string;
}

const AgentsTable: React.FC<AgentsTableProps> = ({ contract_index }) => {
	const classes = useCommonStyles();
	const [agents, setAgents] = useState<PagedResponse_Agent>();
	const [page, setPage] = useState(0);
	const [pageSize, setPageSize] = useState(10);
	const [loading, setLoading] = useState(false);

	useEffect(() => {
		if (!contract_index) return;
		setLoading(true);
		IndexerService.getAdminIndexerAgents(contract_index, page, pageSize)
			.then(setAgents)
			.finally(() => setLoading(false));
	}, [contract_index, page, pageSize]);

	if (loading) return <CircularProgress />;

	return (
		<Box>
			<TableContainer component={Paper} sx={classes.tableContainer}>
				<Table>
					<TableHead>
						<TableRow>
							<TableCell sx={classes.tableHeaderCell}>Agent Address</TableCell>
							<TableCell sx={classes.tableHeaderCell}>Roles</TableCell>
						</TableRow>
					</TableHead>
					<TableBody>
						{agents?.data.length ? (
							agents.data.map((agent) => (
								<TableRow key={agent.agent_address}>
									<TableCell>{agent.agent_address}</TableCell>
									<TableCell>{agent.roles?.join(", ")}</TableCell>
								</TableRow>
							))
						) : (
							<TableRow>
								<TableCell colSpan={2} align="center">
									No agents found.
								</TableCell>
							</TableRow>
						)}
					</TableBody>
				</Table>
			</TableContainer>
			<TablePagination
				component="div"
				count={agents?.page_count || 0 * pageSize}
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

export default AgentsTable;
