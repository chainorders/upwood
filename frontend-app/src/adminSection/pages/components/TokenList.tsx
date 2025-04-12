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
} from "@mui/material";
import { ForestProjectTokenContract, Market, Token } from "../../../apiClient";
import { useNavigate } from "react-router";
import { AddProjectTokenPopup } from "./AddProjectTokenPopup";
import { daysSince } from "../../../lib/conversions";
import { useState } from "react";
import { User } from "../../../lib/user";

interface TokenListProps {
	tokens: Token[];
	tokenContract?: ForestProjectTokenContract;
	tokenContractMarket?: Market;
	onTokenAdded: () => void;
	user: User;
}

export default function TokenList({ tokens, tokenContract, onTokenAdded, user, tokenContractMarket }: TokenListProps) {
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
						{tokens.map((token) => (
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
					</TableBody>
				</Table>
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
