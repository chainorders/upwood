import { useEffect, useState } from "react";
import { SystemContractsConfigApiModel, UserKYCModel, UserService } from "../../apiClient";
import { useOutletContext, useNavigate } from "react-router";
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
	Box,
	ButtonGroup,
} from "@mui/material";
import { TxnStatus, updateContract } from "../../lib/concordium";
import rwaIdentityRegistry from "../../contractClients/generated/rwaIdentityRegistry";
import TransactionButton from "../../components/TransactionButton";
import { User } from "../../lib/user";

export default function UserList() {
	const { user } = useOutletContext<{ user: User }>();
	const [users, setUsers] = useState<UserKYCModel[]>([]);
	const [contracts, setContracts] = useState<SystemContractsConfigApiModel>();
	const [loading, setLoading] = useState(true);
	const [page, setPage] = useState(0);
	const [rowsPerPage, setRowsPerPage] = useState(10);
	const navigate = useNavigate();
	const [registerIdentityTxnStatus, setRegisterIdentityTxnStatus] = useState<Record<string, TxnStatus>>({});
	const [deleteIdentityTxnStatus, setRemoveIdentityTxnStatus] = useState<Record<string, TxnStatus>>({});

	useEffect(() => {
		UserService.getSystemConfig().then(setContracts);
	}, []);
	useEffect(() => {
		setLoading(true);
		UserService.getAdminUserList(page, rowsPerPage).then((data) => {
			setUsers(data.data);
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

	const handleUserInvitations = () => {
		navigate("/admin/users/invitations");
	};

	const handleRegisterIdentity = async (userToRegister: UserKYCModel) => {
		if (!contracts) {
			alert("System contracts not loaded");
			return;
		}

		try {
			await updateContract(
				user.concordiumAccountAddress,
				contracts.identity_registry_contract_index,
				rwaIdentityRegistry.registerIdentity,
				{
					address: { Account: [userToRegister.account_address] },
					identity: {
						credentials: [],
						attributes: [
							{
								tag: 5,
								value: userToRegister.nationality,
							},
						],
					},
				},
				(status) => {
					setRegisterIdentityTxnStatus((prev) => ({ ...prev, [userToRegister.account_address]: status }));
				},
			);
			alert("Identity registered successfully");
			setUsers((prevUsers) =>
				prevUsers.map((u) => (u.cognito_user_id === userToRegister.cognito_user_id ? { ...u, kyc_verified: true } : u)),
			);
		} catch (e) {
			console.error(e);
			alert("Failed to register identity");
			setRegisterIdentityTxnStatus((prev) => ({ ...prev, [userToRegister.account_address]: "error" }));
		}
	};

	const handleDeleteIdentity = async (userToRegister: UserKYCModel) => {
		if (!contracts) {
			alert("System contracts not loaded");
			return;
		}

		try {
			await updateContract(
				user.concordiumAccountAddress,
				contracts.identity_registry_contract_index,
				rwaIdentityRegistry.deleteIdentity,
				{
					Account: [userToRegister.account_address],
				},
				(status) => {
					setRemoveIdentityTxnStatus((prev) => ({ ...prev, [userToRegister.account_address]: status }));
				},
			);
			alert("Identity removed successfully");
			setUsers((prevUsers) =>
				prevUsers.map((u) => (u.cognito_user_id === userToRegister.cognito_user_id ? { ...u, kyc_verified: false } : u)),
			);
		} catch (e) {
			console.error(e);
			alert("Failed to remove identity");
			setRemoveIdentityTxnStatus((prev) => ({ ...prev, [userToRegister.account_address]: "error" }));
		}
	};

	if (loading) {
		return <div>Loading...</div>;
	}

	return (
		<Box display="flex">
			<Paper style={{ flex: 1 }}>
				<TableContainer>
					<Table>
						<TableHead>
							<TableRow>
								<TableCell>Email</TableCell>
								<TableCell>First Name</TableCell>
								<TableCell>Last Name</TableCell>
								<TableCell>Nationality</TableCell>
								<TableCell>Investment Amount</TableCell>
								<TableCell>Account</TableCell>
								<TableCell>KYC Verified</TableCell>
								<TableCell>Affiliate Commission</TableCell>
								<TableCell>Actions</TableCell>
							</TableRow>
						</TableHead>
						<TableBody>
							{users.slice(page * rowsPerPage, page * rowsPerPage + rowsPerPage).map((user) => (
								<TableRow key={user.cognito_user_id}>
									<TableCell>{user.email}</TableCell>
									<TableCell>{user.first_name}</TableCell>
									<TableCell>{user.last_name}</TableCell>
									<TableCell>{user.nationality}</TableCell>
									<TableCell>{user.desired_investment_amount}</TableCell>
									<TableCell>{user.account_address}</TableCell>
									<TableCell>{user.kyc_verified ? "Yes" : "No"}</TableCell>
									<TableCell>{user.affiliate_commission}</TableCell>
									<TableCell>
										<ButtonGroup variant="outlined" size="small">
											{user.kyc_verified ? (
												<TransactionButton
													defaultText="Remove Identity"
													loadingText="Removing"
													txnStatus={deleteIdentityTxnStatus[user.account_address] || "none"}
													onClick={() => handleDeleteIdentity(user)}
												/>
											) : (
												<TransactionButton
													defaultText="Register Identity"
													loadingText="Registering"
													txnStatus={registerIdentityTxnStatus[user.account_address] || "none"}
													onClick={() => handleRegisterIdentity(user)}
												/>
											)}
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
					count={users.length}
					rowsPerPage={rowsPerPage}
					page={page}
					onPageChange={handleChangePage}
					onRowsPerPageChange={handleChangeRowsPerPage}
				/>
			</Paper>
			<Box width={200} ml={2}>
				<Button variant="text" color="primary" onClick={handleUserInvitations}>
					User Invitations
				</Button>
			</Box>
		</Box>
	);
}
