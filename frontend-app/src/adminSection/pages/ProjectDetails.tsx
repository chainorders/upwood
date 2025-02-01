import { useEffect, useState } from "react";
import { useParams } from "react-router";
import { ForestProject, ForestProjectService, ForestProjectTokenContract } from "../../apiClient";
import { Box, Typography, Grid, Paper, List, ListItem, ListItemText, ListItemIcon, Table, TableBody, TableCell, TableContainer, TableHead, TableRow, Button, ButtonGroup } from "@mui/material";
import { Link } from "react-router";
import PhotoLibraryIcon from "@mui/icons-material/PhotoLibrary";

export default function ProjectDetails() {
	const { id } = useParams<{ id: string }>();
	const [project, setProject] = useState<ForestProject | null>(null);
	const [contracts, setContracts] = useState<ForestProjectTokenContract[]>([]);

	useEffect(() => {
		if (id) {
			ForestProjectService.getAdminForestProjects(id).then((response) => {
				setProject(response);
			});
			ForestProjectService.getForestProjectsContractList(id).then((contracts) => {
				setContracts(contracts);
			});
		}
	}, [id]);

	if (!project) {
		return <div>Loading...</div>;
	}

	return (
		<Box sx={{ flexGrow: 1, padding: 2 }}>
			<Grid container spacing={2}>
				<Grid item xs={12} md={8}>
					<Paper sx={{ padding: 2 }}>
						<Typography variant="h4" gutterBottom>
							{project.name}
						</Typography>
						<Typography variant="body1" gutterBottom>
							{project.desc_short}
						</Typography>
						<Typography variant="body1" gutterBottom>
							{project.desc_long}
						</Typography>
						<Typography variant="body2" color="textSecondary">
							Area: {project.area}
						</Typography>
						<Typography variant="body2" color="textSecondary">
							Carbon Credits: {project.carbon_credits}
						</Typography>
						<Typography variant="body2" color="textSecondary">
							ROI Percent: {project.roi_percent}
						</Typography>
						<Typography variant="body2" color="textSecondary">
							State: {project.state}
						</Typography>
						<Typography variant="body2" color="textSecondary">
							Shares Available: {project.shares_available}
						</Typography>
						<Typography variant="body2" color="textSecondary">
							Created At: {new Date(project.created_at).toLocaleDateString()}
						</Typography>
						<Typography variant="body2" color="textSecondary">
							Updated At: {new Date(project.updated_at).toLocaleDateString()}
						</Typography>
					</Paper>
					<Paper sx={{ padding: 2, marginTop: 2 }}>
						<Box sx={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
							<Typography variant="h6" gutterBottom>
								Smart Contracts
							</Typography>
							<Button variant="contained" color="primary" component={Link} to={`../contract/create`}>
								Add Contract
							</Button>
						</Box>
						<TableContainer component={Paper}>
							<Table>
								<TableHead>
									<TableRow>
										<TableCell>Contract Address</TableCell>
										<TableCell>Contract Type</TableCell>
										<TableCell>Fund Token ID</TableCell>
										<TableCell>Market Token ID</TableCell>
										<TableCell>Symbol</TableCell>
										<TableCell>Decimals</TableCell>
										<TableCell>Created At</TableCell>
										<TableCell>Updated At</TableCell>
										<TableCell></TableCell>
									</TableRow>
								</TableHead>
								<TableBody>
									{contracts.map((contract) => (
										<TableRow key={contract.contract_address}>
											<TableCell>{contract.contract_address}</TableCell>
											<TableCell>{contract.contract_type}</TableCell>
											<TableCell>{contract.fund_token_id}</TableCell>
											<TableCell>{contract.market_token_id}</TableCell>
											<TableCell>{contract.symbol}</TableCell>
											<TableCell>{contract.decimals}</TableCell>
											<TableCell>{new Date(contract.created_at).toLocaleDateString()}</TableCell>
											<TableCell>{new Date(contract.updated_at).toLocaleDateString()}</TableCell>
											<TableCell>
												<ButtonGroup variant="contained" size="small">
													<Button color="primary" component={Link} to={`../contract/${contract.contract_address}/details`}>
														Details
													</Button>
													<Button color="secondary" component={Link} to={`../contract/${contract.contract_address}/token/list`}>
														Tokens
													</Button>
												</ButtonGroup>
											</TableCell>
										</TableRow>
									))}
								</TableBody>
							</Table>
						</TableContainer>
					</Paper>
				</Grid>
				<Grid item xs={12} md={4}>
					<Paper sx={{ padding: 2 }}>
						<Typography variant="h6" gutterBottom>
							Actions
						</Typography>
						<List>
							<ListItem button component={Link} to="../media/list">
								<ListItemIcon>
									<PhotoLibraryIcon />
								</ListItemIcon>
								<ListItemText primary="Media" />
							</ListItem>
						</List>
					</Paper>
					<Paper sx={{ padding: 2 }}>
						<Typography variant="h6" gutterBottom>
							Display Images
						</Typography>
						<img src={project.image_large_url} alt={project.name} style={{ width: "100%", marginBottom: 10 }} />
						<img src={project.image_small_url} alt={project.name} style={{ width: "100%" }} />
					</Paper>
				</Grid>
			</Grid>
		</Box>
	);
}
