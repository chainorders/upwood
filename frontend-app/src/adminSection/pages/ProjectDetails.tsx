import { useEffect, useState } from "react";
import { useParams } from "react-router";
import {
	ForestProject,
	ForestProjectService,
	ForestProjectTokenContract,
	PagedResponse_ForestProjectPrice_,
	SystemContractsConfigApiModel,
	UserService,
} from "../../apiClient";
import {
	Box,
	Typography,
	Grid,
	Paper,
	List,
	ListItem,
	ListItemText,
	ListItemIcon,
	Table,
	TableBody,
	TableCell,
	TableContainer,
	TableHead,
	TableRow,
	Button,
	ButtonGroup,
	Accordion,
	AccordionSummary,
	AccordionDetails,
	TablePagination,
} from "@mui/material";
import { Link } from "react-router";
import PhotoLibraryIcon from "@mui/icons-material/PhotoLibrary";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import AddPricePopup from "./components/AddPricePopup";

export default function ProjectDetails() {
	const { id } = useParams<{ id: string }>();
	const [project, setProject] = useState<ForestProject | null>(null);
	const [tokenContracts, setTokenContracts] = useState<ForestProjectTokenContract[]>([]);
	const [prices, setPrices] = useState<PagedResponse_ForestProjectPrice_>();
	const [pricesPage, setPricesPage] = useState(0);
	const [pricesPageSize, setPricesPageSize] = useState(10);
	const [openAddPricePopup, setOpenAddPricePopup] = useState(false);
	const [contracts, setContracts] = useState<SystemContractsConfigApiModel>();

	const handleOpenAddPricePopup = () => {
		setOpenAddPricePopup(true);
	};

	const handleCloseAddPricePopup = () => {
		setOpenAddPricePopup(false);
		setPricesPage(0);
	};

	useEffect(() => {
		ForestProjectService.getAdminForestProjects(id!).then((response) => {
			setProject(response);
		});
		ForestProjectService.getForestProjectsContractList(id!).then((contracts) => {
			setTokenContracts(contracts);
		});
		UserService.getSystemConfig().then(setContracts);
	}, [id]);

	useEffect(() => {
		ForestProjectService.getAdminForestProjectsPriceList(id!, pricesPage, pricesPageSize).then(setPrices);
	}, [id, pricesPage, pricesPageSize]);

	const handlePriceChangePage = (_event: unknown, newPage: number) => {
		setPricesPage(newPage);
	};

	const handlePriceChangeRowsPerPage = (event: React.ChangeEvent<HTMLInputElement>) => {
		setPricesPage(1);
		setPricesPageSize(parseInt(event.target.value, 10));
		// Assuming the API supports changing the number of items per page
		// setRowsPerPage(parseInt(event.target.value, 10));
	};

	if (!project) {
		return <div>Loading...</div>;
	}

	return (
		<Box sx={{ flexGrow: 1, padding: 2 }}>
			<Grid container spacing={2}>
				<Grid item xs={12} md={8}>
					<Paper sx={{ padding: 2 }}>
						<Box sx={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: 2 }}>
							<Typography variant="h4" gutterBottom>
								{project.name}
							</Typography>
							<Button variant="outlined" color="primary" component={Link} to={`../update`}>
								Update Project
							</Button>
						</Box>
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
					<Accordion sx={{ marginTop: 2 }}>
						<AccordionSummary expandIcon={<ExpandMoreIcon />}>
							<Typography variant="h6">Smart Contracts</Typography>
						</AccordionSummary>
						<AccordionDetails>
							<Box sx={{ display: "flex", justifyContent: "flex-end", width: "100%", marginBottom: 2 }}>
								<Button
									variant="outlined"
									color="primary"
									component={Link}
									to={`../contract/create`}
									onClick={(e) => e.stopPropagation()}
								>
									Add Contract
								</Button>
							</Box>
							<TableContainer>
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
										{tokenContracts.map((contract) => (
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
													<ButtonGroup variant="outlined" size="small">
														<Button color="primary" component={Link} to={`../contract/${contract.contract_address}/details`}>
															Details
														</Button>
													</ButtonGroup>
												</TableCell>
											</TableRow>
										))}
									</TableBody>
								</Table>
							</TableContainer>
						</AccordionDetails>
					</Accordion>
					<Accordion sx={{ marginTop: 2 }}>
						<AccordionSummary expandIcon={<ExpandMoreIcon />}>
							<Typography variant="h6">Prices</Typography>
						</AccordionSummary>
						<AccordionDetails>
							<Box sx={{ display: "flex", justifyContent: "flex-end", width: "100%", marginBottom: 2 }}>
								<Button variant="outlined" color="primary" onClick={handleOpenAddPricePopup} disabled={!contracts}>
									Add Price
								</Button>
							</Box>
							<TableContainer>
								<Table>
									<TableHead>
										<TableRow>
											<TableCell>Price</TableCell>
											<TableCell>Price At</TableCell>
											<TableCell>Currency Token ID</TableCell>
											<TableCell>Currency Token Contract Address</TableCell>
										</TableRow>
									</TableHead>
									<TableBody>
										{prices?.data.map((price) => (
											<TableRow key={price.price_at}>
												<TableCell>{price.price}</TableCell>
												<TableCell>{new Date(price.price_at).toLocaleDateString()}</TableCell>
												<TableCell>{price.currency_token_id}</TableCell>
												<TableCell>{price.currency_token_contract_address}</TableCell>
											</TableRow>
										))}
									</TableBody>
								</Table>
							</TableContainer>
							<TablePagination
								component="div"
								count={prices?.page_count || 0}
								page={prices?.page || 0}
								onPageChange={handlePriceChangePage}
								rowsPerPage={-1}
								onRowsPerPageChange={handlePriceChangeRowsPerPage}
							/>
						</AccordionDetails>
					</Accordion>
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
			<AddPricePopup
				open={openAddPricePopup}
				onClose={handleCloseAddPricePopup}
				projectId={id!}
				euroEMetadata={contracts?.euro_e_metadata}
			/>
		</Box>
	);
}
