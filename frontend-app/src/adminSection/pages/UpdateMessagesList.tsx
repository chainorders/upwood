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
import { PagedResponse_PlatformUpdate, UserCommunicationService } from "../../apiClient";
import PlatformUpdateMessageCreatePopup from "../components/PlatformUpdateMessageCreatePopup";

const UpdateMessagesList = () => {
	const [updates, setUpdates] = useState<PagedResponse_PlatformUpdate>();
	const [page, setPage] = useState(0);
	const [rowsPerPage, setRowsPerPage] = useState(10);
	const [open, setOpen] = useState(false);
	const [refreshCounter, setRefreshCounter] = useState(0);

	useEffect(() => {
		UserCommunicationService.getPlatformUpdatesList(page, rowsPerPage).then(setUpdates);
	}, [page, rowsPerPage, refreshCounter]);

	const handleDeleteUpdate = async (id: string) => {
		UserCommunicationService.deleteAdminPlatformUpdates(id).then(() => {
			setRefreshCounter((c) => c + 1);
		});
	};

	return (
		<Box>
			<Box display="flex" justifyContent="space-between" alignItems="center" mb={2}>
				<Typography variant="h4">Platform Update Messages</Typography>
				<Button variant="contained" color="primary" onClick={() => setOpen(true)}>
					Add Platform Update Message
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
							<TableCell>Actions</TableCell>
						</TableRow>
					</TableHead>
					<TableBody>
						{updates?.data.map((update) => (
							<TableRow key={update.id}>
								<TableCell>{update.id}</TableCell>
								<TableCell>{update.title}</TableCell>
								<TableCell>{update.label}</TableCell>
								<TableCell>{update.created_at}</TableCell>
								<TableCell>
									<Button color="secondary" onClick={() => handleDeleteUpdate(update.id)}>
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
				page={updates?.page || 0}
				rowsPerPageOptions={[10, 25, 50]}
				count={updates?.page_count || 0 * rowsPerPage || 0}
				onPageChange={(event, newPage) => setPage(newPage)}
				rowsPerPage={rowsPerPage}
				onRowsPerPageChange={(event) => setRowsPerPage(parseInt(event.target.value, 10))}
			/>
			<PlatformUpdateMessageCreatePopup
				open={open}
				onClose={() => setOpen(false)}
				onAdd={() => setRefreshCounter((c) => c + 1)}
			/>
		</Box>
	);
};

export default UpdateMessagesList;
