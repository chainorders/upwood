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
} from "@mui/material";
import { MaintenanceMessage, PagedResponse_MaintenanceMessage, UserCommunicationService } from "../../apiClient";
import AddMaintenanceMsgPopup from "../components/AddMaintenanceMsgPopup";

const MaintenanceMessageList = () => {
	const [messages, setMessages] = useState<PagedResponse_MaintenanceMessage>();
	const [page, setPage] = useState(0);
	const [rowsPerPage, setRowsPerPage] = useState(10);
	const [totalMessages, setTotalMessages] = useState(0);
	const [open, setOpen] = useState(false);
	const [refreshCounter, setRefreshCounter] = useState(0);

	useEffect(() => {
		UserCommunicationService.getMaintenanceMessagesList(page, rowsPerPage).then(setMessages);
	}, [page, rowsPerPage, refreshCounter]);

	const handleDeleteMessage = async (id: string) => {
		UserCommunicationService.deleteAdminMaintenanceMessages(id).then(() => {
			setRefreshCounter((c) => c + 1);
		});
	};

	return (
		<Box>
			<Button variant="contained" color="primary" onClick={() => setOpen(true)}>
				Add Maintenance Message
			</Button>
			<TableContainer component={Paper}>
				<Table>
					<TableHead>
						<TableRow>
							<TableCell>ID</TableCell>
							<TableCell>Message</TableCell>
							<TableCell>Created At</TableCell>
							<TableCell>Actions</TableCell>
						</TableRow>
					</TableHead>
					<TableBody>
						{messages?.data.map((message) => (
							<TableRow key={message.id}>
								<TableCell>{message.id}</TableCell>
								<TableCell>{message.message}</TableCell>
								<TableCell>{message.created_at}</TableCell>
								<TableCell>
									<Button color="secondary" onClick={() => handleDeleteMessage(message.id)}>
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
				page={messages?.page || 0}
				rowsPerPageOptions={[10, 25, 50]}
				count={messages?.page_count || 0 * rowsPerPage || 0}
				onPageChange={(event, newPage) => setPage(newPage)}
				rowsPerPage={rowsPerPage}
				onRowsPerPageChange={(event) => setRowsPerPage(parseInt(event.target.value, 10))}
				/>
			<AddMaintenanceMsgPopup
				open={open}
				onClose={() => setOpen(false)}
				onAdd={() => setRefreshCounter((c) => c + 1)}
			/>
		</Box>
	);
};

export default MaintenanceMessageList;
