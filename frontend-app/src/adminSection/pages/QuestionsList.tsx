import { useEffect, useState } from "react";
import {
	Box,
	Button,
	ButtonGroup,
	Dialog,
	DialogActions,
	DialogContent,
	DialogTitle,
	Table,
	TableBody,
	TableCell,
	TableHead,
	TableRow,
	Typography,
} from "@mui/material";
import { SupportQuestion } from "../../apiClient/models/SupportQuestion";
import { PagedResponse_SupportQuestion } from "../../apiClient/models/PagedResponse_SupportQuestion";
import { UserCommunicationService } from "../../apiClient";

export default function QuestionsList() {
	const [questions, setQuestions] = useState<PagedResponse_SupportQuestion>();
	const [page, setPage] = useState(0);
	const [rowsPerPage, setRowsPerPage] = useState(10);
	const [selectedQuestion, setSelectedQuestion] = useState<SupportQuestion>();
	const [open, setOpen] = useState(false);
	const [refreshCounter, setRefreshCounter] = useState(0);

	useEffect(() => {
		UserCommunicationService.getAdminSupportQuestionsList(page, rowsPerPage).then(setQuestions);
	}, [page, rowsPerPage, refreshCounter]);

	const handleDelete = (id: string) => {
		UserCommunicationService.deleteAdminSupportQuestions(id).then(() => {
			setRefreshCounter((c) => c + 1);
		});
	};

	const handleViewDetails = (question: SupportQuestion) => {
		setSelectedQuestion(question);
		setOpen(true);
	};

	const handleClose = () => {
		setOpen(false);
		setSelectedQuestion(undefined);
	};

	const truncateMessage = (message: string) => {
		const maxLength = 50;
		return message.length > maxLength ? `${message.substring(0, maxLength)}...` : message;
	};

	return (
		<Box>
			<Typography variant="h4" gutterBottom>
				Support Questions
			</Typography>
			<Table>
				<TableHead>
					<TableRow>
						<TableCell>User Email</TableCell>
						<TableCell>Message</TableCell>
						<TableCell>Actions</TableCell>
					</TableRow>
				</TableHead>
				<TableBody>
					{questions?.data.map((question) => (
						<TableRow key={question.id}>
							<TableCell>
								<a href={`mailto:${question.user_email}`}>{question.user_email}</a>
							</TableCell>
							<TableCell>{truncateMessage(question.message)}</TableCell>
							<TableCell>
								<ButtonGroup size="small" variant="outlined">
									<Button onClick={() => handleViewDetails(question)}>View Details</Button>
									<Button color="error" onClick={() => handleDelete(question.id)}>
										Delete
									</Button>
								</ButtonGroup>
							</TableCell>
						</TableRow>
					))}
				</TableBody>
			</Table>
			<Dialog open={open} onClose={handleClose}>
				<DialogTitle>Question Details</DialogTitle>
				<DialogContent>
					<Typography>{selectedQuestion?.message}</Typography>
				</DialogContent>
				<DialogActions>
					<Button onClick={handleClose}>Close</Button>
				</DialogActions>
			</Dialog>
		</Box>
	);
}
