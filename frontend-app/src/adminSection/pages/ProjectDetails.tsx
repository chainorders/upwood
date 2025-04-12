import { useCallback, useEffect, useState } from "react";
import { useLocation, useParams } from "react-router";
import {
	ForestProject,
	ForestProjectPrice,
	ForestProjectService,
	ForestProjectTokenContract,
	LegalContract,
	PagedResponse_ForestProjectMedia,
	PagedResponse_ForestProjectPrice,
	SystemContractsConfigApiModel,
	UserService,
} from "../../apiClient";
import {
	Box,
	Typography,
	Grid,
	Paper,
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
	Divider,
	Card,
	CardContent,
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
import HomeIcon from "@mui/icons-material/Home";
import ForestIcon from "@mui/icons-material/Folder";
import DescriptionIcon from "@mui/icons-material/Description";
import useTheme from "@mui/material/styles/useTheme";

export default function ProjectDetails({ fileBaseUrl }: { fileBaseUrl: string }) {
	const { id } = useParams<{ id: string }>();
	const location = useLocation();
	const theme = useTheme();
	const [project, setProject] = useState<ForestProject>(location.state?.project);
	const [tokenContracts, setTokenContracts] = useState<ForestProjectTokenContract[]>([]);
	const [prices, setPrices] = useState<PagedResponse_ForestProjectPrice>();
	const [medias, setMedias] = useState<PagedResponse_ForestProjectMedia>();
	const [pricesPage, setPricesPage] = useState(0);
	const [pricesPageSize, setPricesPageSize] = useState(10);
	const [openAddPricePopup, setOpenAddPricePopup] = useState(false);
	const [contracts, setContracts] = useState<SystemContractsConfigApiModel>();
	const [legalContract, setLegalContract] = useState<LegalContract>();
	const [openAddLegalContractPopup, setOpenAddLegalContractPopup] = useState(false);
	const [openUpdateLegalContractPopup, setOpenUpdateLegalContractPopup] = useState(false);
	const [openAddMediaPopup, setOpenAddMediaPopup] = useState(false);
	const [refreshCounter, setRefreshCounter] = useState(0);
	const [legalContractHtml, setLegalContractHtml] = useState<string | null>(null);
	const [legalContractAccordionExpanded, setLegalContractAccordionExpanded] = useState(false);

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

	const fetchLegalContractHtml = useCallback(() => {
		if (legalContract?.text_url) {
			fetch(legalContract.text_url)
				.then((response) => response.text())
				.then((data) => setLegalContractHtml(data))
				.catch((error) => console.error("Failed to fetch HTML content", error));
		}
	}, [legalContract?.text_url]);

	const handleLegalContractAccordionChange = (_event: React.SyntheticEvent, isExpanded: boolean) => {
		setLegalContractAccordionExpanded(isExpanded);
		if (isExpanded && !legalContractHtml) {
			fetchLegalContractHtml();
		}
	};

	useEffect(() => {
		if (legalContractAccordionExpanded) {
			fetchLegalContractHtml();
		}
	}, [legalContract, refreshCounter, legalContractAccordionExpanded, fetchLegalContractHtml]);

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
			<Breadcrumbs aria-label="breadcrumb" sx={{ mb: 2 }}>
				<Link
					to="/admin"
					style={{ display: "flex", alignItems: "center", textDecoration: "none", color: theme.palette.text.secondary }}
				>
					<HomeIcon sx={{ mr: 0.5 }} fontSize="small" />
					Admin
				</Link>
				<Link
					to="/admin/projects"
					style={{ display: "flex", alignItems: "center", textDecoration: "none", color: theme.palette.text.secondary }}
				>
					<ForestIcon sx={{ mr: 0.5 }} fontSize="small" />
					Projects
				</Link>
				<Link
					to={`/admin/projects/${id}/details`}
					style={{ display: "flex", alignItems: "center", textDecoration: "none", color: theme.palette.text.secondary }}
				>
					{project.name}
				</Link>
				<Typography color="text.primary" sx={{ display: "flex", alignItems: "center" }}>
					<DescriptionIcon sx={{ mr: 0.5 }} fontSize="small" />
					Details
				</Typography>
			</Breadcrumbs>
			<Box sx={{ flexGrow: 1, padding: 2 }}>
				<Grid container spacing={2}>
					{/* Change md={8} to md={12} to take full width */}
					<Grid item xs={12} md={12}>
						<Paper sx={{ padding: 2 }} id="project-db-details">
							<Box sx={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: 2 }}>
								<Typography variant="h4" gutterBottom>
									{project.name}
								</Typography>
								<Button variant="outlined" color="primary" component={Link} to={`../update`}>
									Update Project
								</Button>
							</Box>

							{/* Display project images at the top */}
							<Grid container spacing={2} sx={{ mb: 3 }}>
								<Grid item xs={12} md={6}>
									<img src={project.image_large_url} alt={project.name} style={{ width: "100%", borderRadius: "4px" }} />
								</Grid>
								<Grid item xs={12} md={6}>
									<img src={project.image_small_url} alt={project.name} style={{ width: "100%", borderRadius: "4px" }} />
								</Grid>
							</Grid>

							<Typography variant="h6" gutterBottom>
								Basic Information
							</Typography>
							<Typography variant="body1" gutterBottom>
								{project.desc_short}
							</Typography>
							<Typography variant="body1" gutterBottom>
								{project.desc_long}
							</Typography>
							<Typography variant="body2" color="textSecondary">
								Label: {project.label}
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

							<Divider sx={{ my: 2 }} />

							{/* Offering Document section with image preview */}
							<Grid container spacing={2}>
								<Grid item xs={12} md={project.offering_doc_img_url ? 8 : 12}>
									<Typography variant="h6" gutterBottom>
										Offering Document
									</Typography>
									<Typography variant="body2" color="textSecondary">
										Title: {project.offering_doc_title || "N/A"}
									</Typography>
									<Typography variant="body2" color="textSecondary">
										Header: {project.offering_doc_header || "N/A"}
									</Typography>
									<Typography variant="body2" color="textSecondary">
										Footer: {project.offering_doc_footer || "N/A"}
									</Typography>
								</Grid>
								{project.offering_doc_img_url && (
									<Grid item xs={12} md={4} sx={{ display: "flex", alignItems: "center", justifyContent: "center" }}>
										<Box sx={{ p: 1, border: "1px solid #e0e0e0", borderRadius: 1 }}>
											<img
												src={project.offering_doc_img_url}
												alt="Offering Document"
												style={{ maxWidth: "100%", maxHeight: "200px", objectFit: "contain" }}
											/>
											<Typography variant="caption" display="block" align="center" sx={{ mt: 1 }}>
												<a href={project.offering_doc_img_url} target="_blank" rel="noopener noreferrer">
													View Full Size
												</a>
											</Typography>
										</Box>
									</Grid>
								)}
							</Grid>

							<Divider sx={{ my: 2 }} />

							{/* Financial Projection section with image preview */}
							<Grid container spacing={2}>
								<Grid item xs={12} md={project.financial_projection_img_url ? 8 : 12}>
									<Typography variant="h6" gutterBottom>
										Financial Projection
									</Typography>
									<Typography variant="body2" color="textSecondary">
										Title: {project.financial_projection_title || "N/A"}
									</Typography>
									<Typography variant="body2" color="textSecondary">
										Header: {project.financial_projection_header || "N/A"}
									</Typography>
									<Typography variant="body2" color="textSecondary">
										Footer: {project.financial_projection_footer || "N/A"}
									</Typography>
								</Grid>
								{project.financial_projection_img_url && (
									<Grid item xs={12} md={4} sx={{ display: "flex", alignItems: "center", justifyContent: "center" }}>
										<Box sx={{ p: 1, border: "1px solid #e0e0e0", borderRadius: 1 }}>
											<img
												src={project.financial_projection_img_url}
												alt="Financial Projection"
												style={{ maxWidth: "100%", maxHeight: "200px", objectFit: "contain" }}
											/>
											<Typography variant="caption" display="block" align="center" sx={{ mt: 1 }}>
												<a href={project.financial_projection_img_url} target="_blank" rel="noopener noreferrer">
													View Full Size
												</a>
											</Typography>
										</Box>
									</Grid>
								)}
							</Grid>

							<Divider sx={{ my: 2 }} />

							{/* Geospatial Information section with image preview */}
							<Grid container spacing={2}>
								<Grid item xs={12} md={project.geo_img_url ? 8 : 12}>
									<Typography variant="h6" gutterBottom>
										Geospatial Information
									</Typography>
									<Typography variant="body2" color="textSecondary">
										Title: {project.geo_title || "N/A"}
									</Typography>
									<Typography variant="body2" color="textSecondary">
										Header: {project.geo_header || "N/A"}
									</Typography>
									<Typography variant="body2" color="textSecondary">
										Footer: {project.geo_footer || "N/A"}
									</Typography>
								</Grid>
								{project.geo_img_url && (
									<Grid item xs={12} md={4} sx={{ display: "flex", alignItems: "center", justifyContent: "center" }}>
										<Box sx={{ p: 1, border: "1px solid #e0e0e0", borderRadius: 1 }}>
											<img
												src={project.geo_img_url}
												alt="Geospatial Information"
												style={{ maxWidth: "100%", maxHeight: "200px", objectFit: "contain" }}
											/>
											<Typography variant="caption" display="block" align="center" sx={{ mt: 1 }}>
												<a href={project.geo_img_url} target="_blank" rel="noopener noreferrer">
													View Full Size
												</a>
											</Typography>
										</Box>
									</Grid>
								)}
							</Grid>
						</Paper>

						{/* Accordion sections remain the same */}
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
								{/* Add Property Media information at the top */}
								<Paper elevation={0} sx={{ p: 2, mb: 3 }}>
									<Typography variant="subtitle1" gutterBottom>
										Property Media Information
									</Typography>
									<Typography variant="body2" color="textSecondary">
										Header: {project.property_media_header}
									</Typography>
									<Typography variant="body2" color="textSecondary" sx={{ mb: 2 }}>
										Footer: {project.property_media_footer}
									</Typography>
								</Paper>

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
												<TableCell>Action</TableCell>
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
						<Accordion
							sx={{ marginTop: 2 }}
							expanded={legalContractAccordionExpanded}
							onChange={handleLegalContractAccordionChange}
						>
							<AccordionSummary expandIcon={<ExpandMoreIcon />}>
								<Typography variant="h6">Legal Contract</Typography>
							</AccordionSummary>
							<AccordionDetails>
								<Box sx={{ display: "flex", justifyContent: "flex-end", width: "100%", marginBottom: 2 }}>
									<Button
										variant="outlined"
										color="primary"
										onClick={handleOpenLegalContractPopup}
										startIcon={legalContract ? <EditIcon /> : <AddIcon />}
									>
										{legalContract ? "Update Legal Contract" : "Add Legal Contract"}
									</Button>
								</Box>
								{legalContract ? (
									<>
										<Paper elevation={0} sx={{ p: 2, mb: 2 }}>
											<Grid container spacing={2}>
												<Grid item xs={12}>
													<Typography variant="h6">{legalContract.name || "Contract"}</Typography>
													{legalContract.tag && (
														<Typography variant="body2" color="textSecondary" gutterBottom>
															Tag: {legalContract.tag}
														</Typography>
													)}
												</Grid>
												<Grid item xs={12}>
													<Typography variant="subtitle1" gutterBottom>
														Available Formats:
													</Typography>
													<Box sx={{ display: "flex", gap: 2, flexWrap: "wrap" }}>
														{legalContract.text_url && (
															<Button variant="outlined" color="primary" size="small" href={legalContract.text_url} target="_blank">
																Text Version
															</Button>
														)}
														{legalContract.edoc_url && (
															<Button variant="outlined" color="primary" size="small" href={legalContract.edoc_url} target="_blank">
																E-Document
															</Button>
														)}
														{legalContract.pdf_url && (
															<Button variant="outlined" color="primary" size="small" href={legalContract.pdf_url} target="_blank">
																PDF Version
															</Button>
														)}
													</Box>
												</Grid>
												<Grid item xs={12}>
													<Typography variant="body2" color="textSecondary">
														Created: {new Date(legalContract.created_at).toLocaleDateString()} at{" "}
														{new Date(legalContract.created_at).toLocaleTimeString()}
													</Typography>
													<Typography variant="body2" color="textSecondary">
														Last updated: {new Date(legalContract.updated_at).toLocaleDateString()} at{" "}
														{new Date(legalContract.updated_at).toLocaleTimeString()}
													</Typography>
												</Grid>
											</Grid>
										</Paper>
										{legalContractHtml && (
											<Card variant="outlined" sx={{ marginTop: 2, maxHeight: "500px", overflow: "auto" }}>
												<CardContent>
													<Typography variant="h6" gutterBottom>
														Contract Content Preview:
													</Typography>
													<Box sx={{ mt: 2 }} dangerouslySetInnerHTML={{ __html: legalContractHtml }} />
												</CardContent>
											</Card>
										)}
									</>
								) : (
									<Typography variant="body1" align="center" sx={{ py: 2 }}>
										No legal contract has been added yet.
									</Typography>
								)}
							</AccordionDetails>
						</Accordion>
					</Grid>
					{/* Remove the entire sidebar Grid item */}
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
				<AddMediaPopup
					open={openAddMediaPopup}
					onClose={handleCloseAddMediaPopup}
					projectId={id!}
					fileBaseUrl={fileBaseUrl}
				/>
			</Box>
		</>
	);
}
