import {
	Table,
	TableBody,
	TableCell,
	TableContainer,
	TableHead,
	TableRow,
	Paper,
	Button,
	ButtonGroup,
	Box,
	Dialog,
	DialogTitle,
	DialogContent,
	TablePagination,
} from "@mui/material";
import { ForestProjectTokenContract, Market, PagedResponse_Token } from "../../../apiClient";
import { useNavigate } from "react-router";
import { AddProjectTokenPopup } from "./AddProjectTokenPopup";
import { daysSince } from "../../../lib/conversions";
import { useState } from "react";
import { User } from "../../../lib/user";

interface TokenListProps {
	tokens: PagedResponse_Token;
	tokenContract: ForestProjectTokenContract;
	tokenContractMarket?: Market;
	onTokenAdded: () => void;
	onPageChange: (page: number) => void;
	onPageSizeChange: (pageSize: number) => void;
	user: User;
	pageSize: number;
}

export default function ProjectTokenList({
	tokens,
	tokenContract,
	onTokenAdded,
	user,
	tokenContractMarket,
	onPageChange,
	onPageSizeChange,
	pageSize,
}: TokenListProps) {
	const navigate = useNavigate();
	const [openPopup, setOpenPopup] = useState(false);

	const handleOpenPopup = () => {
		setOpenPopup(true);
	};

	const handleClosePopup = () => {
		setOpenPopup(false);
		onTokenAdded();
	};

	return (
		<Box>
			<Box sx={{ display: "flex", justifyContent: "flex-end", marginBottom: 2 }}>
				<Button variant="outlined" color="primary" onClick={() => handleOpenPopup()}>
					Add Token
				</Button>
			</Box>
			<TableContainer component={Paper}>
				<Table>
					<TableHead>
						<TableRow>
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
						{tokens.data.map((token) => (
							<TableRow key={token.cis2_address + token.token_id}>
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
								<TableCell>{tokenContractMarket?.token_id === token.token_id ? "Yes" : "No"}</TableCell>
								<TableCell>{tokenContract?.fund_token_id == token.token_id ? "Yes" : "No"}</TableCell>
								<TableCell>{token.supply}</TableCell>
								<TableCell>
									<ButtonGroup size="small">
										<Button variant="outlined" color="primary" onClick={() => navigate(`../token/${token.token_id}/details`)}>
											Details
										</Button>
									</ButtonGroup>
								</TableCell>
							</TableRow>
						))}
						{tokens.data.length === 0 && (
							<TableRow>
								<TableCell colSpan={10} align="center">
									No tokens found.
								</TableCell>
							</TableRow>
						)}
					</TableBody>
				</Table>
				<TablePagination
					component="div"
					count={tokens.page_count * pageSize}
					page={tokens.page}
					onPageChange={(_, newPage) => {
						onPageChange(newPage);
					}}
					rowsPerPage={pageSize}
					onRowsPerPageChange={(e) => {
						onPageSizeChange(parseInt(e.target.value, 10));
						onPageChange(0);
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
