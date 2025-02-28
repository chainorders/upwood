import React from 'react';
import { User } from "../../lib/user";
import { useEffect, useState } from "react";
import { UserRegistrationRequest, UserService } from "../../apiClient";
import {
	Table,
	TableBody,
	TableCell,
	TableContainer,
	TableHead,
	TableRow,
	Paper,
	TablePagination,
	Button,
	ButtonGroup,
} from "@mui/material";

const UserInvitations = ({ user }: { user: User }) => {
	const [loading, setLoading] = useState(true);
	const [page, setPage] = useState(0);
	const [rowsPerPage, setRowsPerPage] = useState(10);
	const [requests, setRequests] = useState<UserRegistrationRequest[]>([]);

	useEffect(() => {
		setLoading(true);
		UserService.getAdminRegistrationRequestList(page, rowsPerPage).then((data) => {
			setRequests(data.data);
			setLoading(false);
		});
	}, [user, page, rowsPerPage]);

	const handleChangePage = (_event: unknown, newPage: number) => {
		setPage(newPage);
	};

	const handleChangeRowsPerPage = (event: React.ChangeEvent<HTMLInputElement>) => {
		setRowsPerPage(parseInt(event.target.value, 10));
		setPage(0);
	};

	const handleAccept = (id: string) => {
		UserService.putAdminRegistrationRequestAccept(id, true).then(() => {
			setRequests((prevRequests) =>
				prevRequests.map((request) => (request.id === id ? { ...request, is_accepted: true } : request)),
			);
		});
	};

	const handleDeny = (id: string) => {
		UserService.putAdminRegistrationRequestAccept(id, false).then(() => {
			setRequests((prevRequests) =>
				prevRequests.map((request) => (request.id === id ? { ...request, is_accepted: false } : request)),
			);
		});
	};

	if (loading) {
		return <div>Loading...</div>;
	}

	return (
		<Paper style={{ flex: 1 }}>
			<TableContainer>
				<Table>
					<TableHead>
						<TableRow>
							<TableCell>Email</TableCell>
							<TableCell>Affiliate Account Address</TableCell>
							<TableCell>Accepted</TableCell>
							<TableCell>Created At</TableCell>
							<TableCell>Updated At</TableCell>
							<TableCell>Actions</TableCell>
						</TableRow>
					</TableHead>
					<TableBody>
						{requests.slice(page * rowsPerPage, page * rowsPerPage + rowsPerPage).map((request) => (
							<TableRow key={request.id}>
								<TableCell>{request.email}</TableCell>
								<TableCell>{request.affiliate_account_address}</TableCell>
								<TableCell>{request.is_accepted ? "Yes" : "No"}</TableCell>
								<TableCell>{new Date(request.created_at).toLocaleDateString()}</TableCell>
								<TableCell>{new Date(request.updated_at).toLocaleDateString()}</TableCell>
								<TableCell>
									<ButtonGroup size="small" variant="outlined">
										<Button color="primary" onClick={() => handleAccept(request.id)} disabled={request.is_accepted}>
											Accept
										</Button>
										<Button color="secondary" onClick={() => handleDeny(request.id)} style={{ marginLeft: 8 }}>
											Deny
										</Button>
									</ButtonGroup>
								</TableCell>
							</TableRow>
						))}
					</TableBody>
				</Table>
			</TableContainer>
			<TablePagination
				rowsPerPageOptions={[5, 10, 25]}
				component="div"
				count={requests.length}
				rowsPerPage={rowsPerPage}
				page={page}
				onPageChange={handleChangePage}
				onRowsPerPageChange={handleChangeRowsPerPage}
			/>
		</Paper>
	);
};

export default UserInvitations;
