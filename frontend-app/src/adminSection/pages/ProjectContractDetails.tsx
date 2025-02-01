import { useParams } from "react-router";
import { useEffect, useState } from "react";
import {
	Box,
	Typography,
	CircularProgress,
	Grid,
	Paper,
	List,
	ListItem,
	ListItemText,
	ListItemIcon,
} from "@mui/material";
import { ForestProjectService, ForestProjectTokenContract } from "../../apiClient";
import { Link } from "react-router";
import EditIcon from "@mui/icons-material/Edit";
import DeleteIcon from "@mui/icons-material/Delete";
import TokenIcon from "@mui/icons-material/Token";
import MarketDetails from "./components/MarketDetails";
import FundDetails from "./components/FundDetails";

export default function ProjectContractDetails() {
	const { contract_address } = useParams<{ contract_address?: string }>();
	const [contract, setContract] = useState<ForestProjectTokenContract | null>(null);
	const [loading, setLoading] = useState(true);

	useEffect(() => {
		ForestProjectService.getAdminForestProjectsContract(contract_address!)
			.then((data) => {
				setContract(data);
				setLoading(false);
			})
			.catch(() => {
				alert("Failed to fetch contract details");
				setLoading(false);
			});
	}, [contract_address]);

	if (loading) {
		return <CircularProgress />;
	}

	if (!contract) {
		return <Typography>No contract found</Typography>;
	}

	return (
		<Box sx={{ flexGrow: 1, padding: 2 }}>
			<Grid container spacing={2}>
				<Grid item xs={12} md={8}>
					<Paper sx={{ padding: 2 }}>
						<Typography variant="h6">Contract Details</Typography>
						<Typography>
							<strong>Contract Address:</strong> {contract.contract_address}
						</Typography>
						<Typography>
							<strong>Forest Project ID:</strong> {contract.forest_project_id}
						</Typography>
						<Typography>
							<strong>Contract Type:</strong> {contract.contract_type}
						</Typography>
						<Typography>
							<strong>Fund Token ID:</strong>
							{contract.fund_token_id ? (
								<Link to={`../token/${contract.fund_token_id}/details`}>{contract.fund_token_id}</Link>
							) : (
								"N/A"
							)}
						</Typography>
						<Typography>
							<strong>Market Token ID:</strong>
							{contract.market_token_id ? (
								<Link to={`../token/${contract.market_token_id}/details`}>{contract.market_token_id}</Link>
							) : (
								"N/A"
							)}
						</Typography>
						<Typography>
							<strong>Symbol:</strong> {contract.symbol || "N/A"}
						</Typography>
						<Typography>
							<strong>Decimals:</strong> {contract.decimals !== undefined ? contract.decimals : "N/A"}
						</Typography>
						<Typography>
							<strong>Created At:</strong> {contract.created_at}
						</Typography>
						<Typography>
							<strong>Updated At:</strong> {contract.updated_at}
						</Typography>
					</Paper>
					{contract.market_token_id && (
						<MarketDetails contract_address={contract.contract_address} token_id={contract.market_token_id} />
					)}
					{contract.fund_token_id && (
						<FundDetails contract_address={contract.contract_address} token_id={contract.fund_token_id || ""} />
					)}
				</Grid>
				<Grid item xs={12} md={4}>
					<Paper sx={{ padding: 2 }}>
						<Typography variant="h6" gutterBottom>
							Actions
						</Typography>
						<List>
							<ListItem button component={Link} to={`../update`}>
								<ListItemIcon>
									<EditIcon />
								</ListItemIcon>
								<ListItemText primary="Update Contract" />
							</ListItem>
							<ListItem button onClick={() => alert("Not implemented")}>
								<ListItemIcon>
									<DeleteIcon />
								</ListItemIcon>
								<ListItemText primary="Delete Contract" />
							</ListItem>
							<ListItem button component={Link} to={`../token/list`}>
								<ListItemIcon>
									<TokenIcon />
								</ListItemIcon>
								<ListItemText primary="Tokens List" />
							</ListItem>
						</List>
					</Paper>
				</Grid>
			</Grid>
		</Box>
	);
}
