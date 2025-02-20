import { useEffect, useState } from "react";
import { useParams } from "react-router";
import {
	ForestProject,
	ForestProjectPrice,
	ForestProjectService,
	ForestProjectTokenContract,
	LegalContract,
	PagedResponse_ForestProjectMedia_,
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
	Breadcrumbs,
} from "@mui/material";
import { Link } from "react-router";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import AddPricePopup from "./components/AddPricePopup";
import AddLegalContractPopup from "./components/AddLegalContractPopup";
import UpdateLegalContractPopup from "./components/UpdateLegalContractPopup";
import AddIcon from "@mui/icons-material/Add";
import EditIcon from "@mui/icons-material/Edit";
import AddMediaPopup from "./components/AddMediaPopup";
import DeleteIcon from "@mui/icons-material/Delete"; // Add this import
import { toDisplayAmount } from "../../lib/conversions";

export default function ProjectDetails() {
	const { id } = useParams<{ id: string }>();
	const [project, setProject] = useState<ForestProject | null>(null);
	const [tokenContracts, setTokenContracts] = useState<ForestProjectTokenContract[]>([]);
	const [prices, setPrices] = useState<PagedResponse_ForestProjectPrice_>();
	const [medias, setMedias] = useState<PagedResponse_ForestProjectMedia_>();
	const [pricesPage, setPricesPage] = useState(0);
	const [pricesPageSize, setPricesPageSize] = useState(10);
	const [openAddPricePopup, setOpenAddPricePopup] = useState(false);
	const [contracts, setContracts] = useState<SystemContractsConfigApiModel>();
	const [legalContract, setLegalContract] = useState<LegalContract>();
	const [openAddLegalContractPopup, setOpenAddLegalContractPopup] = useState(false);
	const [openUpdateLegalContractPopup, setOpenUpdateLegalContractPopup] = useState(false);
	const [openAddMediaPopup, setOpenAddMediaPopup] = useState(false);
	const [refreshCounter, setRefreshCounter] = useState(0);

	const handleOpenAddPricePopup = () => {
		setOpenAddPricePopup(true);
	};

	const handleCloseAddPricePopup = () => {
		setOpenAddPricePopup(false);
		setRefreshCounter((c) => c + 1);
	};

	const handleOpenLegalContractPopup = () => {
		if (legalContract) {
			setOpenUpdateLegalContractPopup(true);
		} else {
			setOpenAddLegalContractPopup(true);
		}
	};

	const handleCloseLegalContractPopup = () => {
		setOpenAddLegalContractPopup(false);
		setOpenUpdateLegalContractPopup(false);
		setRefreshCounter((c) => c + 1);
	};

	const handleOpenAddMediaPopup = () => {
		setOpenAddMediaPopup(true);
	};

	const handleCloseAddMediaPopup = () => {
		setOpenAddMediaPopup(false);
		setRefreshCounter((c) => c + 1);
	};

	const handleDeleteMedia = (mediaId: string) => {
		ForestProjectService.deleteAdminForestProjectsMedia(id!, mediaId).then(() => {
			setRefreshCounter((c) => c + 1);
		});
	};

	const handleDeletePrice = (price: ForestProjectPrice) => {
		ForestProjectService.deleteAdminForestProjectsPrice(id!, price.price_at).then(() => {
			setRefreshCounter((c) => c + 1);
		});
	};

	useEffect(() => {
		ForestProjectService.getAdminForestProjects(id!).then(setProject);
		ForestProjectService.getForestProjectsContractList(id!).then(setTokenContracts);
		UserService.getSystemConfig().then(setContracts);
		ForestProjectService.getAdminLegalContract(id!).then(setLegalContract);
		ForestProjectService.getForestProjectsMediaList(id!).then(setMedias);
	}, [id, refreshCounter]);

	useEffect(() => {
		ForestProjectService.getAdminForestProjectsPriceList(id!, pricesPage, pricesPageSize).then(setPrices);
	}, [id, pricesPage, pricesPageSize, refreshCounter]);

	const handlePriceChangePage = (_event: unknown, newPage: number) => {
		setPricesPage(newPage);
	};

	const handlePriceChangeRowsPerPage = (event: React.ChangeEvent<HTMLInputElement>) => {
		setPricesPage(1);
		setPricesPageSize(parseInt(event.target.value, 10));
	};

	if (!project) {
		return <div>Loading...</div>;
	}

	return (
		<>
			<Breadcrumbs aria-label="breadcrumb">
				<Link to="/admin">Admin</Link>
				<Link to="/admin/projects">Projects</Link>
				<Link to={`/admin/projects/${id}/details`}>{project.name}</Link>
				<Typography color="textPrimary">Details</Typography>
			</Breadcrumbs>
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
									<Button
										variant="outlined"
										color="primary"
										onClick={handleOpenAddPricePopup}
										disabled={!contracts?.euro_e_metadata}
									>
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
												<TableCell></TableCell>
											</TableRow>
										</TableHead>
										<TableBody>
											{prices?.data.map((price) => (
												<TableRow key={price.price_at}>
													<TableCell>{toDisplayAmount(price.price, contracts?.euro_e_metadata.decimals || 6, 6)}</TableCell>
													<TableCell>{new Date(price.price_at).toLocaleDateString()}</TableCell>
													<TableCell>{price.currency_token_id}</TableCell>
													<TableCell>{price.currency_token_contract_address}</TableCell>
													<TableCell>
														<ButtonGroup size="small" variant="outlined">
															<Button color="secondary" startIcon={<DeleteIcon />} onClick={() => handleDeletePrice(price)}>
																Delete
															</Button>
														</ButtonGroup>
													</TableCell>
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
									rowsPerPage={10}
									rowsPerPageOptions={[10]}
									onRowsPerPageChange={handlePriceChangeRowsPerPage}
								/>
							</AccordionDetails>
						</Accordion>
						<Accordion sx={{ marginTop: 2 }}>
							<AccordionSummary expandIcon={<ExpandMoreIcon />}>
								<Typography variant="h6">Forest Project Media</Typography>
							</AccordionSummary>
							<AccordionDetails>
								<Box sx={{ display: "flex", justifyContent: "flex-end", width: "100%", marginBottom: 2 }}>
									<Button variant="outlined" color="primary" onClick={handleOpenAddMediaPopup}>
										Add Media
									</Button>
								</Box>
								<TableContainer>
									<Table>
										<TableHead>
											<TableRow>
												<TableCell>Preview</TableCell>
												<TableCell>ID</TableCell>
												<TableCell>Image URL</TableCell>
												<TableCell>Action</TableCell> {/* Add this TableCell */}
											</TableRow>
										</TableHead>
										<TableBody>
											{medias?.data.map((media) => (
												<TableRow key={media.id}>
													<TableCell>
														<img src={media.image_url} alt="preview" style={{ width: 50, height: 50 }} />
													</TableCell>
													<TableCell>{media.id}</TableCell>
													<TableCell>
														<a href={media.image_url} target="_blank" rel="noopener noreferrer">
															{media.image_url}
														</a>
													</TableCell>
													<TableCell>
														<Button
															variant="outlined"
															color="secondary"
															startIcon={<DeleteIcon />}
															onClick={() => handleDeleteMedia(media.id)}
														>
															Delete
														</Button>
													</TableCell>
												</TableRow>
											))}
										</TableBody>
									</Table>
								</TableContainer>
								<TablePagination
									component="div"
									count={medias?.page_count || 0}
									page={medias?.page || 0}
									onPageChange={(_event, newPage) => setPricesPage(newPage)}
									rowsPerPage={10}
									rowsPerPageOptions={[10]}
									onRowsPerPageChange={(event) => setPricesPageSize(parseInt(event.target.value, 10))}
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
								<ListItem button onClick={handleOpenLegalContractPopup}>
									<ListItemIcon>{legalContract ? <EditIcon /> : <AddIcon />}</ListItemIcon>
									<ListItemText primary={legalContract ? "Update Legal Contract" : "Add Legal Contract"} />
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
				{contracts?.euro_e_metadata && (
					<AddPricePopup
						open={openAddPricePopup}
						onClose={handleCloseAddPricePopup}
						projectId={id!}
						euroEMetadata={contracts.euro_e_metadata}
					/>
				)}
				<AddLegalContractPopup
					open={openAddLegalContractPopup}
					onClose={handleCloseLegalContractPopup}
					projectId={project.id}
				/>
				<UpdateLegalContractPopup
					open={openUpdateLegalContractPopup}
					onClose={handleCloseLegalContractPopup}
					projectId={project.id}
				/>
				<AddMediaPopup open={openAddMediaPopup} onClose={handleCloseAddMediaPopup} projectId={id!} />
			</Box>
		</>
	);
}
