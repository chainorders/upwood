import { useEffect, useState } from "react";
import {
	Table,
	TableBody,
	TableCell,
	TableContainer,
	TableHead,
	TableRow,
	Paper,
	Typography,
	Box,
	Button,
	ButtonGroup,
	DialogTitle,
	DialogContent,
	TablePagination,
} from "@mui/material";
import {
	ForestProjectService,
	ForestProjectTokenContract,
	IndexerService,
	Market,
	PagedResponse_Token,
} from "../../apiClient";
import { Link, useParams } from "react-router";
import { AddProjectTokenPopup } from "./components/AddProjectTokenPopup";
import Dialog from "@mui/material/Dialog";
import { daysSince } from "../../lib/conversions";
import { User } from "../../lib/user";

export default function ProjectTokenList({ user, market }: { user: User; market?: Market }) {
	const { id, contract_address } = useParams<{ id: string; contract_address: string }>();
	const [tokens, setTokens] = useState<PagedResponse_Token>();
	const [page, setPage] = useState(0);
	const [pageSize, setPageSize] = useState(10);
	const [tokenContract, setTokenContract] = useState<ForestProjectTokenContract>();
	const [openPopup, setOpenPopup] = useState(false);
	const [refreshCounter, setRefreshCounter] = useState(0);

	const handleOpenPopup = () => {
		setOpenPopup(true);
	};

	const handleClosePopup = () => {
		setOpenPopup(false);
		setRefreshCounter((c) => c + 1);
	};

	useEffect(() => {
		ForestProjectService.getAdminForestProjectsContract(contract_address!).then(setTokenContract).catch(console.error);
	}, [id, contract_address, refreshCounter]);
	useEffect(() => {
		if (!tokenContract) return;
		IndexerService.getAdminIndexerCis2TokenList(tokenContract.contract_address, page, pageSize)
			.then(setTokens)
			.catch(console.error);
	}, [tokenContract, page, pageSize]);

	return (
		<Box sx={{ flexGrow: 1, padding: 2 }}>
			<Typography variant="h4" gutterBottom>
				Forest Project Tokens
			</Typography>
			<Box sx={{ display: "flex", justifyContent: "flex-end", marginBottom: 2 }}>
				<Button variant="contained" color="primary" onClick={handleOpenPopup}>
					Add Token
				</Button>
			</Box>
			<TableContainer component={Paper}>
				<Table>
					<TableHead>
						<TableRow>
							<TableCell>Forest Project ID</TableCell>
							<TableCell>Token Contract Address</TableCell>
							<TableCell>Token Contract Type</TableCell>
							<TableCell>Symbol</TableCell>
							<TableCell>Decimals</TableCell>
							<TableCell>Token ID</TableCell>
							<TableCell>Metadata URL</TableCell>
							<TableCell>Is Paused</TableCell>
							<TableCell>Market Token</TableCell>
							<TableCell>Fund Token</TableCell>
							<TableCell>Supply</TableCell>
							<TableCell>Actions</TableCell>
						</TableRow>
					</TableHead>
					<TableBody>
						{tokens?.data.map((token) => (
							<TableRow key={token.cis2_address + token.token_id}>
								<TableCell>{id}</TableCell>
								<TableCell>{tokenContract?.contract_address}</TableCell>
								<TableCell>{tokenContract?.contract_type}</TableCell>
								<TableCell>{tokenContract?.symbol}</TableCell>
								<TableCell>{tokenContract?.decimals}</TableCell>
								<TableCell>{token.token_id}</TableCell>
								<TableCell>
									<a href={token.metadata_url} target="_blank" rel="noopener noreferrer">
										{token.metadata_url}
									</a>
								</TableCell>
								<TableCell>{token.is_paused ? "Yes" : "No"}</TableCell>
								<TableCell>{market?.token_id == token.token_id ? "Yes" : "No"}</TableCell>
								<TableCell>{tokenContract?.fund_token_id == token.token_id ? "Yes" : "No"}</TableCell>
								<TableCell>{token.supply}</TableCell>
								<TableCell>
									<ButtonGroup size="small">
										<Button variant="contained" color="primary" component={Link} to={`../${token.token_id}/details`}>
											Details
										</Button>
									</ButtonGroup>
								</TableCell>
							</TableRow>
						))}
						{tokens?.data.length === 0 && (
							<TableRow>
								<TableCell colSpan={12} align="center">
									No tokens found.
								</TableCell>
							</TableRow>
						)}
					</TableBody>
				</Table>
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
			</TableContainer>
			<Dialog open={openPopup} onClose={handleClosePopup} fullWidth>
				<DialogTitle>Add Token</DialogTitle>
				<DialogContent>
					<AddProjectTokenPopup
						user={user}
						token_contract={tokenContract!}
						token_id={tokenContract ? daysSince(tokenContract?.created_at).toString() : ""}
						onDone={handleClosePopup}
					/>
				</DialogContent>
			</Dialog>
		</Box>
	);
}
