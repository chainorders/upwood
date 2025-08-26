import { useEffect, useState } from "react";
import { PagedResponse_UserKYCModel, SystemContractsConfigApiModel, UserKYCModel, UserService } from "../../apiClient";
import { useNavigate } from "react-router";
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
	Tooltip,
} from "@mui/material";
import { TxnStatus, updateContract } from "../../lib/concordium";
import rwaIdentityRegistry from "../../contractClients/generated/rwaIdentityRegistry";
import TransactionButton from "../../components/TransactionButton";
import { User } from "../../lib/user";

const UserList = ({ user }: { user: User }) => {
	const [users, setUsers] = useState<PagedResponse_UserKYCModel>();
	const [contracts, setContracts] = useState<SystemContractsConfigApiModel>();
	const [page, setPage] = useState(0);
	const [rowsPerPage, setRowsPerPage] = useState(10);
	const navigate = useNavigate();
	const [registerIdentityTxnStatus, setRegisterIdentityTxnStatus] = useState<Record<string, TxnStatus>>({});
	const [deleteIdentityTxnStatus, setRemoveIdentityTxnStatus] = useState<Record<string, TxnStatus>>({});
	const [refreshCounter] = useState(0);

	useEffect(() => {
		UserService.getSystemConfig().then(setContracts);
	}, [user]);
	useEffect(() => {
		UserService.getAdminUserList(page, rowsPerPage).then(setUsers);
	}, [page, rowsPerPage, refreshCounter]);

	const updateUserKycStatus = (accountAddress: string, kycVerified: boolean) => {
		setUsers((prev) => ({
			...prev!,
			data: prev!.data.map((user) => {
				if (user.account_address === accountAddress) {
					return { ...user, kyc_verified: kycVerified };
				}
				return user;
			}),
		}));
	};
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
			updateUserKycStatus(userToRegister.account_address, true);
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
			updateUserKycStatus(userToRegister.account_address, false);
		} catch (e) {
			console.error(e);
			alert("Failed to remove identity");
			setRemoveIdentityTxnStatus((prev) => ({ ...prev, [userToRegister.account_address]: "error" }));
		}
	};

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
							{users?.data.map((user) => (
								<TableRow key={user.cognito_user_id}>
									<TableCell>
										<Tooltip title={user.cognito_user_id}>
											<span>{user.email}</span>
										</Tooltip>
									</TableCell>
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
					count={users?.page_count || 0 * rowsPerPage || 0}
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
};

export default UserList;
