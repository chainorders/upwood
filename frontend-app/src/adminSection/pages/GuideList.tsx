import React, { useEffect, useState } from "react";
import {
	Box,
	Button,
	Paper,
	Table,
	TableBody,
	TableCell,
	TableContainer,
	TableHead,
	TablePagination,
	TableRow,
	Typography,
} from "@mui/material";
import { PagedResponse_Guide, UserCommunicationService } from "../../apiClient";
import AddGuidePopup from "../components/AddGuidePopup";

const GuideList = () => {
	const [guides, setGuides] = useState<PagedResponse_Guide>();
	const [page, setPage] = useState(0);
	const [rowsPerPage, setRowsPerPage] = useState(10);
	const [open, setOpen] = useState(false);
	const [refreshCounter, setRefreshCounter] = useState(0);

	useEffect(() => {
		UserCommunicationService.getGuidesList(page, rowsPerPage).then(setGuides);
	}, [page, rowsPerPage, refreshCounter]);

	const handleDeleteGuide = async (id: string) => {
		UserCommunicationService.deleteAdminGuides(id).then(() => {
			setRefreshCounter((c) => c + 1);
		});
	};

	return (
		<Box>
			<Box display="flex" justifyContent="space-between" alignItems="center" mb={2}>
				<Typography variant="h4">How to Guides</Typography>
				<Button variant="contained" color="primary" onClick={() => setOpen(true)}>
					Add Guide
				</Button>
			</Box>
			<TableContainer component={Paper}>
				<Table>
					<TableHead>
						<TableRow>
							<TableCell>ID</TableCell>
							<TableCell>Title</TableCell>
							<TableCell>Label</TableCell>
                            <TableCell>Created At</TableCell>
                            <TableCell>Url</TableCell>
							<TableCell>Actions</TableCell>
						</TableRow>
					</TableHead>
					<TableBody>
						{guides?.data.map((guide) => (
							<TableRow key={guide.id}>
								<TableCell>{guide.id}</TableCell>
								<TableCell>{guide.title}</TableCell>
								<TableCell>{guide.label}</TableCell>
                                <TableCell>{guide.created_at}</TableCell>
                                <TableCell>{guide.guide_url}</TableCell>
								<TableCell>
									<Button color="secondary" onClick={() => handleDeleteGuide(guide.id)}>
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
				page={guides?.page || 0}
				rowsPerPageOptions={[10, 25, 50]}
				count={guides?.page_count || 0 * rowsPerPage || 0}
				onPageChange={(event, newPage) => setPage(newPage)}
				rowsPerPage={rowsPerPage}
				onRowsPerPageChange={(event) => setRowsPerPage(parseInt(event.target.value, 10))}
			/>
			<AddGuidePopup open={open} onClose={() => setOpen(false)} onAdd={() => setRefreshCounter((c) => c + 1)} />
		</Box>
	);
};

export default GuideList;
