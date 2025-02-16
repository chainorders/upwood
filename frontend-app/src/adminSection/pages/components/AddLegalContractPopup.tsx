import { useForm, Controller } from "react-hook-form";
import { Dialog, DialogTitle, DialogContent, DialogActions, TextField, Button, Card, CardContent } from "@mui/material";
import { LegalContract, ForestProjectService } from "../../../apiClient";
import { formatDate } from "../../../lib/conversions"; // Import formatDate
import { useState, useEffect } from "react";

interface AddLegalContractPopupProps {
	open: boolean;
	onClose: () => void;
	projectId: string;
}

export default function AddLegalContractPopup({ open, onClose, projectId }: AddLegalContractPopupProps) {
	const { control, handleSubmit, reset, watch } = useForm<LegalContract>({
		defaultValues: {
			project_id: projectId,
			text_url: "",
			edoc_url: "",
			pdf_url: "",
			created_at: formatDate(new Date()), // Format created_at
			updated_at: formatDate(new Date()), // Format updated_at
		},
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

	const onSubmit = async (data: LegalContract) => {
		try {
			const now = new Date();
			data.project_id = projectId;
			data.created_at = formatDate(now); // Format created_at
			data.updated_at = formatDate(now); // Format updated_at
			await ForestProjectService.postAdminLegalContract(data);
			onClose();
			reset();
		} catch (error) {
			console.error("Failed to add legal contract", error);
		}
	};

	return (
		<Dialog open={open} onClose={onClose} fullWidth>
			<DialogTitle>Add Legal Contract</DialogTitle>
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
							Add Legal Contract
						</Button>
					</DialogActions>
				</form>
			</DialogContent>
		</Dialog>
	);
}
