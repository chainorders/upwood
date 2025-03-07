import { useEffect, useState } from "react";
import { CircularProgress, Box, Typography, Dialog, DialogTitle, DialogContent, Button } from "@mui/material";
import { FaExclamationTriangle } from "react-icons/fa";
import { FaCheckCircle } from "react-icons/fa";
import { FaArrowCircleLeft } from "react-icons/fa";

import { Link, Navigate, useLocation, useParams } from "react-router";
import { UserService } from "../apiClient";

export default function CompanyInvitation({ accepted }: { accepted: boolean }) {
	const [state, setState] = useState<"none" | "accepting" | "rejecting" | "accepted" | "rejected" | "error">("rejected");
	const location = useLocation();
	const { invitationId } = useParams<{ invitationId: string }>();
	useEffect(() => {
		if (!invitationId) {
			return;
		}

		if (accepted) {
			setState("accepting");
			UserService.putCompanyInvitation(invitationId, accepted)
				.then(() => {
					setState("accepted");
				})
				.catch(() => {
					setState("error");
				});
		} else {
			setState("rejecting");
			UserService.putCompanyInvitation(invitationId, accepted)
				.then(() => {
					setState("rejected");
				})
				.catch(() => {
					setState("error");
				});
		}
	}, [invitationId, accepted]);

	if (!invitationId) {
		return <Navigate to="../" />;
	}

	return (
		<Dialog open>
			<DialogTitle textAlign={"center"}>Invitation</DialogTitle>
			<DialogContent>
				<Box display="flex" justifyContent="center" alignItems="center" height="100%" flexDirection="column">
					{state === "accepting" && <CircularProgress size="3em" sx={{ color: "green" }} />}
					{state === "rejecting" && <CircularProgress size="3em" sx={{ color: "red" }} />}
					{state === "accepted" && <FaCheckCircle fontSize="3em" color="green" />}
					{state === "rejected" && <FaCheckCircle fontSize="3em" color="red" />}
					{state === "error" && <FaExclamationTriangle fontSize="3em" />}
					<Typography variant="h6" textAlign="center" mt="10px">
						{state === "accepting" && "Accepting invitation"}
						{state === "rejecting" && "Rejecting invitation"}
						{state === "accepted" && "Invitation accepted"}
						{state === "rejected" && "Invitation rejected"}
						{state === "error" && "An error occurred. Please try again later."}
					</Typography>
					<Link to="/settings">
						<Button variant="text" sx={{color: "green"}}>
							<FaArrowCircleLeft />
							<Typography ml="5px">Back to Home</Typography>
						</Button>
					</Link>
				</Box>
			</DialogContent>
		</Dialog>
	);
}
