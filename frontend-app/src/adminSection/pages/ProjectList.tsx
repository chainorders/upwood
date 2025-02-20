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
	ButtonGroup,
	Breadcrumbs,
} from "@mui/material";

export default function ProjectList() {
	const [projects, setProjects] = useState<ForestProject[]>([]);
	const [filters, setFilters] = useState<{ page: number; state?: ForestProjectState }>({
		page: 0,
	});
	const [pageCount, setPageCount] = useState(0);

	useEffect(() => {
		ForestProjectService.getAdminForestProjectsList(filters.page, filters.state)
			.then(({ data: projects, page_count: pageCount }) => {
				setProjects(projects);
				// setFilters({ ...filters, page: page });
				setPageCount(pageCount);
			})
			.catch(() => {
				alert("Failed to fetch projects");
			});
	}, [filters]);

	const handleStateChange = (event: SelectChangeEvent<ForestProjectState>) => {
		const value = event.target.value as ForestProjectState;
		setFilters({ ...filters, state: value ? value : undefined });
	};

	const handlePageChange = (_event: React.ChangeEvent<unknown>, newPage: number) => {
		setFilters({ ...filters, page: newPage - 1 });
	};

	return (
		<>
			<Breadcrumbs aria-label="breadcrumb">
				<Link to="/admin">Admin</Link>
				<Link to="/admin/projects">Projects</Link>
			</Breadcrumbs>
			<Typography variant="h4" gutterBottom>
				Forest Projects
			</Typography>
			<div className="filter-bar" style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
				<FormControl variant="outlined" style={{ minWidth: 200 }}>
					<InputLabel id="state-filter-label">Filter by State</InputLabel>
					<Select
						labelId="state-filter-label"
						id="state-filter"
						value={filters.state || ""}
						onChange={handleStateChange}
						label="Filter by State"
					>
						<MenuItem value="">
							<em>All</em>
						</MenuItem>
						<MenuItem value={ForestProjectState.DRAFT}>Draft</MenuItem>
						<MenuItem value={ForestProjectState.ACTIVE}>Active</MenuItem>
						<MenuItem value={ForestProjectState.FUNDED}>Funded</MenuItem>
						<MenuItem value={ForestProjectState.BOND}>Bond</MenuItem>
						<MenuItem value={ForestProjectState.ARCHIVED}>Archived</MenuItem>
					</Select>
				</FormControl>
				<Button variant="contained" color="primary" component={Link} to="create">
					Create Project
				</Button>
			</div>
			<TableContainer component={Paper}>
				<Table className="project-table">
					<TableHead>
						<TableRow>
							<TableCell>ID</TableCell>
							<TableCell>Name</TableCell>
							<TableCell>Label</TableCell>
							<TableCell>Short Description</TableCell>
							<TableCell>Area</TableCell>
							<TableCell>Carbon Credits</TableCell>
							<TableCell>ROI Percent</TableCell>
							<TableCell>State</TableCell>
							<TableCell>Shares Available</TableCell>
							<TableCell>Created At</TableCell>
							<TableCell>Updated At</TableCell>
							<TableCell></TableCell>
						</TableRow>
					</TableHead>
					<TableBody>
						{projects.map((project) => (
							<TableRow key={project.id}>
								<TableCell>{project.id}</TableCell>
								<TableCell>{project.name}</TableCell>
								<TableCell>{project.label}</TableCell>
								<TableCell>{project.desc_short}</TableCell>
								<TableCell>{project.area}</TableCell>
								<TableCell>{project.carbon_credits}</TableCell>
								<TableCell>{project.roi_percent}</TableCell>
								<TableCell>{project.state}</TableCell>
								<TableCell>{project.shares_available}</TableCell>
								<TableCell>{new Date(project.created_at).toLocaleDateString()}</TableCell>
								<TableCell>{new Date(project.updated_at).toLocaleDateString()}</TableCell>
								<TableCell>
									<ButtonGroup variant="contained" size="small">
										<Button color="primary" component={Link} to={`${project.id}/details`}>
											Details
										</Button>
									</ButtonGroup>
								</TableCell>
							</TableRow>
						))}
					</TableBody>
				</Table>
			</TableContainer>
			<Pagination
				count={pageCount}
				page={filters.page + 1}
				onChange={handlePageChange}
				style={{ marginTop: 20, display: "flex", justifyContent: "center" }}
			/>
		</>
	);
}
