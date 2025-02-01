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
} from "@mui/material";
import { ForestProjectService, Token, ForestProjectTokenContract, IndexerService } from "../../apiClient";
import { Link, useParams } from "react-router";
import { AddProjectTokenPopup } from "./components/AddProjectTokenPopup";
import Dialog from "@mui/material/Dialog";
import { daysSince } from "../../lib/conversions";

export default function ProjectTokenList() {
	const { id, contract_address } = useParams<{ id: string; contract_address: string }>();
	const [tokens, setTokens] = useState<Token[]>([]);
	const [tokenContract, setTokenContract] = useState<ForestProjectTokenContract | null>(null);
	const [openPopup, setOpenPopup] = useState(false);

	const handleOpenPopup = () => {
		setOpenPopup(true);
	};

	const handleClosePopup = () => {
		setOpenPopup(false);
		window.location.reload();
	};

	useEffect(() => {
		ForestProjectService.getAdminForestProjectsContract(contract_address!)
			.then((contract) => {
				setTokenContract(contract);
			})
			.catch(() => {
				alert("Failed to fetch contract details");
			});
		IndexerService.getAdminIndexerCis2TokenList(contract_address!)
			.then((tokens) => {
				setTokens(tokens);
			})
			.catch(() => {
				alert("Failed to fetch tokens");
			});
	}, [id, contract_address]);

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
						{tokens.map((token) => (
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
								<TableCell>{tokenContract?.market_token_id == token.token_id ? "Yes" : "No"}</TableCell>
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
					</TableBody>
				</Table>
			</TableContainer>
			<Dialog open={openPopup} onClose={handleClosePopup} fullWidth>
				<DialogTitle>Add Token</DialogTitle>
				<DialogContent>
					<AddProjectTokenPopup
						token_contract={tokenContract!}
						token_id={tokenContract ? daysSince(tokenContract?.created_at).toString() : ""}
						onDone={handleClosePopup}
					/>
				</DialogContent>
			</Dialog>
		</Box>
	);
}
