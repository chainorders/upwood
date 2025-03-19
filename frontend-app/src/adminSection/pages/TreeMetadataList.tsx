import React, { useCallback, useEffect, useState } from "react";
import {
	Paper,
	Typography,
	Button,
	Table,
	TableBody,
	TableCell,
	TableContainer,
	TableHead,
	TableRow,
	Box,
	IconButton,
	Tooltip,
	Dialog,
	CircularProgress,
	Snackbar,
	Alert,
	TablePagination,
} from "@mui/material";
import DeleteIcon from "@mui/icons-material/Delete";
import AddIcon from "@mui/icons-material/Add";
import { TreeNftMetadataService } from "../../apiClient/services/TreeNftMetadataService";
import { TreeNftMetadata } from "../../apiClient/models/TreeNftMetadata";
import { CreateTreeNftMetadataPopup } from "./components/CreateTreeNftMetadataPopup";
import { User } from "../../lib/user";

interface TreeMetadataListProps {
	user: User;
	fileBaseUrl: string;
}

const TreeMetadataList: React.FC<TreeMetadataListProps> = ({ user, fileBaseUrl }) => {
	const [metadatas, setMetadatas] = useState<TreeNftMetadata[]>([]);
	const [loading, setLoading] = useState(true);
	const [error, setError] = useState<string>();
	const [openCreatePopup, setOpenCreatePopup] = useState(false);
	const [currentPage, setCurrentPage] = useState(0);
	const [totalCount, setTotalCount] = useState(0);
	const [rowsPerPage, setRowsPerPage] = useState(10);
	const [notification, setNotification] = useState<{ message: string; type: "success" | "error" }>();

	const fetchMetadatas = useCallback(async () => {
		setLoading(true);
		try {
			const response = await TreeNftMetadataService.getAdminTreeNftMetadataList(currentPage);
			setMetadatas(response);

			// If this is the first page and we have results, estimate total count
			// In a real app, the API should return the total count
			if (currentPage === 0) {
				// If we have fewer records than rows per page, that's our total
				// Otherwise we estimate there might be more pages
				setTotalCount(response.length < rowsPerPage ? response.length : response.length * 2);
			} else if (response.length === 0 && currentPage > 0) {
				// If we get an empty page, we've reached the end
				setTotalCount(currentPage * rowsPerPage);
			} else {
				// Otherwise, update with what we know
				setTotalCount(Math.max(totalCount, (currentPage + 1) * response.length));
			}

			setError(undefined);
		} catch (err) {
			console.error("Error fetching tree NFT metadata:", err);
			setError("Failed to load metadata records");
		} finally {
			setLoading(false);
		}
	}, [currentPage, rowsPerPage, totalCount]);

	useEffect(() => {
		fetchMetadatas();
	}, [currentPage, fetchMetadatas]);

	const handleDeleteMetadata = async (id: string) => {
		if (window.confirm("Are you sure you want to delete this metadata?")) {
			try {
				await TreeNftMetadataService.deleteAdminTreeNftMetadata(id);
				setNotification({ message: "Metadata deleted successfully", type: "success" });
				fetchMetadatas(); // Refresh the list
			} catch (err) {
				console.error("Error deleting metadata:", err);
				setNotification({ message: "Failed to delete metadata", type: "error" });
			}
		}
	};

	const handleCreateSuccess = () => {
		setOpenCreatePopup(false);
		setNotification({ message: "Metadata created successfully", type: "success" });
		// Go to first page after creation
		setCurrentPage(0);
		fetchMetadatas(); // Refresh the list
	};

	const handleChangePage = (_event: React.MouseEvent<HTMLButtonElement> | null, newPage: number) => {
		setCurrentPage(newPage);
	};

	const handleChangeRowsPerPage = (event: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
		setRowsPerPage(parseInt(event.target.value, 10));
		setCurrentPage(0); // Reset to first page
	};

	const closeNotification = () => {
		setNotification(undefined);
	};

	return (
		<Paper sx={{ padding: 3 }}>
			<Box sx={{ display: "flex", justifyContent: "space-between", alignItems: "center", mb: 3 }}>
				<Typography variant="h5" component="h1">
					Tree NFT Metadatas
				</Typography>
				<Button variant="contained" color="primary" startIcon={<AddIcon />} onClick={() => setOpenCreatePopup(true)}>
					Create New Metadata
				</Button>
			</Box>

			{loading && metadatas.length === 0 ? (
				<Box sx={{ display: "flex", justifyContent: "center", my: 4 }}>
					<CircularProgress />
				</Box>
			) : error ? (
				<Box sx={{ my: 2 }}>
					<Alert severity="error">{error}</Alert>
				</Box>
			) : (
				<>
					<TableContainer>
						<Table>
							<TableHead>
								<TableRow>
									<TableCell>ID</TableCell>
									<TableCell>Metadata URL</TableCell>
									<TableCell>Metadata Hash</TableCell>
									<TableCell>Probability Percentage</TableCell>
									<TableCell>Created At</TableCell>
									<TableCell align="right">Actions</TableCell>
								</TableRow>
							</TableHead>
							<TableBody>
								{loading ? (
									<TableRow>
										<TableCell colSpan={6} align="center">
											<CircularProgress size={24} />
										</TableCell>
									</TableRow>
								) : metadatas.length === 0 ? (
									<TableRow>
										<TableCell colSpan={6} align="center">
											No records found
										</TableCell>
									</TableRow>
								) : (
									metadatas.map((metadata) => (
										<TableRow key={metadata.id}>
											<TableCell>{metadata.id}</TableCell>
											<TableCell>
												<Tooltip title={metadata.metadata_url}>
													<span>
														{metadata.metadata_url.length > 30
															? `${metadata.metadata_url.substring(0, 30)}...`
															: metadata.metadata_url}
													</span>
												</Tooltip>
											</TableCell>
											<TableCell>{metadata.metadata_hash || "N/A"}</TableCell>
											<TableCell>{metadata.probablity_percentage}%</TableCell>
											<TableCell>{new Date(metadata.created_at).toLocaleString()}</TableCell>
											<TableCell align="right">
												<Tooltip title="Delete">
													<IconButton onClick={() => handleDeleteMetadata(metadata.id)} color="error" size="small">
														<DeleteIcon />
													</IconButton>
												</Tooltip>
											</TableCell>
										</TableRow>
									))
								)}
							</TableBody>
						</Table>
					</TableContainer>

					<TablePagination
						component="div"
						count={totalCount}
						page={currentPage}
						onPageChange={handleChangePage}
						rowsPerPage={rowsPerPage}
						onRowsPerPageChange={handleChangeRowsPerPage}
						rowsPerPageOptions={[5, 10, 25, 50]}
						labelDisplayedRows={({ from, to, count }) => `${from}-${to} of ${count !== -1 ? count : `more than ${to}`}`}
						SelectProps={{
							inputProps: { "aria-label": "rows per page" },
							native: true,
						}}
						sx={{ mt: 2 }}
					/>
				</>
			)}

			<CreateTreeNftMetadataPopup
				open={openCreatePopup}
				onClose={() => setOpenCreatePopup(false)}
				onSuccess={handleCreateSuccess}
				fileBaseUrl={fileBaseUrl}
			/>

			<Snackbar
				open={!!notification}
				autoHideDuration={6000}
				onClose={closeNotification}
				anchorOrigin={{ vertical: "bottom", horizontal: "right" }}
			>
				<Alert onClose={closeNotification} severity={notification?.type} sx={{ width: "100%" }}>
					{notification?.message}
				</Alert>
			</Snackbar>
		</Paper>
	);
};

export default TreeMetadataList;
