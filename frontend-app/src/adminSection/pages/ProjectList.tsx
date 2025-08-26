import { useEffect, useState } from "react";
import { ForestProject, ForestProjectService, ForestProjectState } from "../../apiClient";
import { Link } from "react-router";
import {
	Button,
	Select,
	MenuItem,
	InputLabel,
	FormControl,
	Table,
	TableBody,
	TableCell,
	TableContainer,
	TableHead,
	TableRow,
	Paper,
	Pagination,
	SelectChangeEvent,
	Typography,
	Breadcrumbs,
	Box,
	Chip,
	Stack,
	useTheme,
	alpha,
	Tooltip,
	IconButton,
	CircularProgress,
} from "@mui/material";
import HomeIcon from "@mui/icons-material/Home";
import ForestIcon from "@mui/icons-material/Folder";
import FilterListIcon from "@mui/icons-material/FilterList";
import AddIcon from "@mui/icons-material/Add";
import VisibilityIcon from "@mui/icons-material/Visibility";
import EditIcon from "@mui/icons-material/Edit";

export default function ProjectList() {
	const theme = useTheme();
	const [loading, setLoading] = useState<boolean>(false);
	const [projects, setProjects] = useState<ForestProject[]>([]);
	const [filters, setFilters] = useState<{ page: number; state?: ForestProjectState }>({
		page: 0,
	});
	const [pageCount, setPageCount] = useState(0);

	useEffect(() => {
		setLoading(true);
		ForestProjectService.getAdminForestProjectsList(filters.page, filters.state)
			.then(({ data: projects, page_count: pageCount }) => {
				setProjects(projects);
				setPageCount(pageCount);
				setLoading(false);
			})
			.catch(() => {
				alert("Failed to fetch projects");
				setLoading(false);
			});
	}, [filters]);

	const handleStateChange = (event: SelectChangeEvent<ForestProjectState>) => {
		const value = event.target.value as ForestProjectState;
		setFilters({ ...filters, state: value ? value : undefined });
	};

	const handlePageChange = (_event: React.ChangeEvent<unknown>, newPage: number) => {
		setFilters({ ...filters, page: newPage - 1 });
	};

	// Helper function to render project state with appropriate color
	const getStateChip = (state: ForestProjectState) => {
		let color: "default" | "primary" | "secondary" | "error" | "info" | "success" | "warning" = "default";

		switch (state) {
			case ForestProjectState.DRAFT:
				color = "default";
				break;
			case ForestProjectState.ACTIVE:
				color = "success";
				break;
			case ForestProjectState.FUNDED:
				color = "primary";
				break;
			case ForestProjectState.BOND:
				color = "info";
				break;
			case ForestProjectState.ARCHIVED:
				color = "error";
				break;
		}

		return <Chip label={state} color={color} size="small" />;
	};

	// Function to truncate text
	const truncateText = (text: string | undefined, maxLength: number) => {
		if (!text) return "";
		return text.length > maxLength ? text.slice(0, maxLength) + "..." : text;
	};

	return (
		<>
			<Box sx={{ mb: 4 }}>
				<Breadcrumbs aria-label="breadcrumb" sx={{ mb: 2 }}>
					<Link
						to="/admin"
						style={{ display: "flex", alignItems: "center", textDecoration: "none", color: theme.palette.text.secondary }}
					>
						<HomeIcon sx={{ mr: 0.5 }} fontSize="small" />
						Admin
					</Link>
					<Typography color="text.primary" sx={{ display: "flex", alignItems: "center" }}>
						<ForestIcon sx={{ mr: 0.5 }} fontSize="small" />
						Projects
					</Typography>
				</Breadcrumbs>
				<Typography variant="h4" gutterBottom fontWeight="500" sx={{ display: "flex", alignItems: "center" }}>
					<ForestIcon sx={{ mr: 1, color: theme.palette.primary.main }} />
					Forest Projects
				</Typography>
			</Box>

			<Paper elevation={2} sx={{ p: 2, mb: 4, borderRadius: 2 }}>
				<Box
					sx={{
						display: "flex",
						justifyContent: "space-between",
						alignItems: "center",
						flexWrap: "wrap",
						gap: 2,
					}}
				>
					<Stack direction="row" spacing={2} alignItems="center">
						<FilterListIcon color="action" />
						<FormControl variant="outlined" size="small" sx={{ minWidth: 200 }}>
							<InputLabel id="state-filter-label">Project State</InputLabel>
							<Select
								labelId="state-filter-label"
								id="state-filter"
								value={filters.state || ""}
								onChange={handleStateChange}
								label="Project State"
							>
								<MenuItem value="">
									<em>All States</em>
								</MenuItem>
								<MenuItem value={ForestProjectState.DRAFT}>Draft</MenuItem>
								<MenuItem value={ForestProjectState.ACTIVE}>Active</MenuItem>
								<MenuItem value={ForestProjectState.FUNDED}>Funded</MenuItem>
								<MenuItem value={ForestProjectState.BOND}>Bond</MenuItem>
								<MenuItem value={ForestProjectState.ARCHIVED}>Archived</MenuItem>
							</Select>
						</FormControl>
					</Stack>

					<Button
						variant="contained"
						color="primary"
						component={Link}
						to="create"
						startIcon={<AddIcon />}
						sx={{
							borderRadius: 2,
							boxShadow: 2,
							px: 3,
						}}
					>
						Create Project
					</Button>
				</Box>
			</Paper>

			<TableContainer
				component={Paper}
				elevation={3}
				sx={{
					borderRadius: 2,
					overflow: "hidden",
					mb: 3,
					"& .MuiTableRow-root:hover": {
						backgroundColor: alpha(theme.palette.primary.main, 0.04),
					},
				}}
			>
				<Table className="project-table" size="medium">
					<TableHead>
						<TableRow sx={{ backgroundColor: alpha(theme.palette.primary.main, 0.08) }}>
							<TableCell sx={{ fontWeight: "bold" }}>ID</TableCell>
							<TableCell sx={{ fontWeight: "bold" }}>Name</TableCell>
							<TableCell sx={{ fontWeight: "bold" }}>Label</TableCell>
							<TableCell sx={{ fontWeight: "bold" }}>Description</TableCell>
							<TableCell sx={{ fontWeight: "bold" }} align="center">
								Area
							</TableCell>
							<TableCell sx={{ fontWeight: "bold" }} align="center">
								Carbon Credits
							</TableCell>
							<TableCell sx={{ fontWeight: "bold" }} align="center">
								ROI %
							</TableCell>
							<TableCell sx={{ fontWeight: "bold" }} align="center">
								State
							</TableCell>
							<TableCell sx={{ fontWeight: "bold" }} align="center">
								Shares
							</TableCell>
							<TableCell sx={{ fontWeight: "bold" }}>Created</TableCell>
							<TableCell sx={{ fontWeight: "bold" }}>Updated</TableCell>
							<TableCell sx={{ fontWeight: "bold" }} align="center">
								Actions
							</TableCell>
						</TableRow>
					</TableHead>
					<TableBody>
						{loading ? (
							<TableRow>
								<TableCell colSpan={12} align="center" sx={{ py: 5 }}>
									<CircularProgress size={40} />
									<Typography variant="body2" sx={{ mt: 2, color: "text.secondary" }}>
										Loading projects...
									</Typography>
								</TableCell>
							</TableRow>
						) : projects.length === 0 ? (
							<TableRow>
								<TableCell colSpan={12} align="center" sx={{ py: 5 }}>
									<Typography variant="body2" sx={{ color: "text.secondary" }}>
										No projects found
									</Typography>
								</TableCell>
							</TableRow>
						) : (
							projects.map((project) => (
								<TableRow key={project.id}>
									<TableCell>{project.id}</TableCell>
									<TableCell sx={{ fontWeight: 500 }}>{project.name}</TableCell>
									<TableCell>{project.label}</TableCell>
									<TableCell>
										<Tooltip title={project.desc_short || ""} arrow placement="top">
											<span>{truncateText(project.desc_short, 40)}</span>
										</Tooltip>
									</TableCell>
									<TableCell align="center">{project.area}</TableCell>
									<TableCell align="center">{project.carbon_credits}</TableCell>
									<TableCell align="center">
										{project.roi_percent ? (
											<Chip
												label={`${project.roi_percent}%`}
												size="small"
												variant="outlined"
												color={project.roi_percent > 10 ? "success" : "default"}
											/>
										) : (
											"-"
										)}
									</TableCell>
									<TableCell align="center">{getStateChip(project.state)}</TableCell>
									<TableCell align="center">{project.shares_available}</TableCell>
									<TableCell>{new Date(project.created_at).toLocaleDateString()}</TableCell>
									<TableCell>{new Date(project.updated_at).toLocaleDateString()}</TableCell>
									<TableCell>
										<Stack direction="row" spacing={1} justifyContent="center">
											<Tooltip title="View Details">
												<IconButton
													size="small"
													component={Link}
													to={`${project.id}/details`}
													sx={{
														color: theme.palette.primary.main,
														bgcolor: alpha(theme.palette.primary.main, 0.1),
														"&:hover": {
															bgcolor: alpha(theme.palette.primary.main, 0.2),
														},
													}}
												>
													<VisibilityIcon fontSize="small" />
												</IconButton>
											</Tooltip>
											<Tooltip title="Edit Project">
												<IconButton
													size="small"
													component={Link}
													to={`${project.id}/update`}
													sx={{
														color: theme.palette.warning.main,
														bgcolor: alpha(theme.palette.warning.main, 0.1),
														"&:hover": {
															bgcolor: alpha(theme.palette.warning.main, 0.2),
														},
													}}
												>
													<EditIcon fontSize="small" />
												</IconButton>
											</Tooltip>
										</Stack>
									</TableCell>
								</TableRow>
							))
						)}
					</TableBody>
				</Table>
			</TableContainer>

			<Box sx={{ display: "flex", justifyContent: "center", mt: 4, mb: 2 }}>
				<Pagination
					count={pageCount}
					page={filters.page + 1}
					onChange={handlePageChange}
					color="primary"
					size="large"
					showFirstButton
					showLastButton
					sx={{
						"& .MuiPaginationItem-root": {
							borderRadius: 1,
						},
					}}
				/>
			</Box>
		</>
	);
}
