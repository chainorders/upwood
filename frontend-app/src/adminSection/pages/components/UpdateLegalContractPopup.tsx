import { Dialog, DialogTitle, DialogContent, DialogActions, TextField, Button, Card, CardContent } from "@mui/material";
import { ForestProjectService, LegalContract } from "../../../apiClient";
import { useEffect, useState } from "react";
import { useForm, Controller } from "react-hook-form";
import { formatDate } from "../../../lib/conversions"; // Import formatDate

interface UpdateLegalContractPopupProps {
	projectId: string;
	open: boolean;
	onClose: () => void;
}

export default function UpdateLegalContractPopup({ open, onClose, projectId }: UpdateLegalContractPopupProps) {
	const [legalContract, setLegalContract] = useState<LegalContract>();
	const { control, handleSubmit, reset, watch } = useForm<LegalContract>({
		defaultValues: legalContract,
	});

	const textUrl = watch("text_url");
	const [htmlContent, setHtmlContent] = useState<string | null>(null);

	useEffect(() => {
		if (textUrl) {
			fetch(textUrl)
				.then((response) => response.text())
				.then((data) => setHtmlContent(data))
				.catch((error) => console.error("Failed to fetch HTML content", error));
		} else {
			setHtmlContent(null);
		}
	}, [textUrl]);

	useEffect(() => {
		ForestProjectService.getAdminLegalContract(projectId).then((data) => {
			setLegalContract(data);
			reset(data);
		});
	}, [projectId, reset]);

	const onSubmit = async (data: LegalContract) => {
		try {
			const now = new Date();
			data.updated_at = formatDate(now); // Format updated_at
			await ForestProjectService.putAdminLegalContract(data);
			onClose();
		} catch (error) {
			console.error("Failed to update legal contract", error);
		}
	};

	return (
		<Dialog open={open} onClose={onClose} fullWidth>
			<DialogTitle>Update Legal Contract</DialogTitle>
			<DialogContent>
				<form onSubmit={handleSubmit(onSubmit)}>
					<Controller
						name="project_id"
						control={control}
						defaultValue={projectId}
						render={({ field }) => <TextField {...field} type="hidden" />}
					/>
					<Controller
						name="name"
						control={control}
						defaultValue=""
						render={({ field }) => <TextField {...field} label="Name" fullWidth margin="normal" />}
					/>
					<Controller
						name="tag"
						control={control}
						defaultValue=""
						render={({ field }) => <TextField {...field} label="Tag" fullWidth margin="normal" />}
					/>
					<Controller
						name="text_url"
						control={control}
						defaultValue=""
						render={({ field }) => <TextField {...field} label="Text URL" fullWidth margin="normal" />}
					/>
					{htmlContent && (
						<Card variant="outlined" sx={{ marginTop: 2 }}>
							<CardContent dangerouslySetInnerHTML={{ __html: htmlContent }} />
						</Card>
					)}
					<Controller
						name="edoc_url"
						control={control}
						defaultValue=""
						render={({ field }) => <TextField {...field} label="eDoc URL" fullWidth margin="normal" />}
					/>
					<Controller
						name="pdf_url"
						control={control}
						defaultValue=""
						render={({ field }) => <TextField {...field} label="PDF URL" fullWidth margin="normal" />}
					/>
					<DialogActions>
						<Button onClick={onClose} color="primary">
							Cancel
						</Button>
						<Button type="submit" variant="contained" color="primary">
							Update Legal Contract
						</Button>
					</DialogActions>
				</form>
			</DialogContent>
		</Dialog>
	);
}
