import { useForm, Controller } from "react-hook-form";
import { Dialog, DialogActions, DialogContent, DialogTitle, Button, TextField } from "@mui/material";
import { Guide, UserCommunicationService } from "../../apiClient";
import { v4 as uuid } from "uuid";
import { formatDate } from "../../lib/conversions";

const AddGuidePopup = ({ open, onClose, onAdd }: { open: boolean; onClose: () => void; onAdd: () => void }) => {
	const { control, handleSubmit, reset } = useForm<Guide>({
		defaultValues: {
			title: "",
			label: "",
			guide_url: "",
			order_index: Math.ceil(new Date().getTime() / 1000),
		},
	});

	const onSubmit = async (data: Guide) => {
		data.id = uuid();
		data.created_at = formatDate(new Date());
		data.order_index = Number(data.order_index);

		await UserCommunicationService.postAdminGuides(data);
		onAdd();
		reset();
		onClose();
	};

	return (
		<Dialog open={open} onClose={onClose}>
			<DialogTitle>Add Guide</DialogTitle>
			<DialogContent>
				<form onSubmit={handleSubmit(onSubmit)}>
					<Controller
						name="title"
						control={control}
						render={({ field }) => <TextField {...field} label="Title" fullWidth margin="normal" />}
					/>
					<Controller
						name="label"
						control={control}
						render={({ field }) => <TextField {...field} label="Label" fullWidth margin="normal" />}
					/>
					<Controller
						name="guide_url"
						control={control}
						render={({ field }) => <TextField {...field} label="Guide URL" fullWidth margin="normal" />}
					/>
					<Controller
						name="order_index"
						control={control}
						render={({ field }) => <TextField {...field} label="Order Index" fullWidth margin="normal" />}
					/>
					<DialogActions>
						<Button onClick={onClose} color="primary">
							Cancel
						</Button>
						<Button type="submit" color="primary">
							Add
						</Button>
					</DialogActions>
				</form>
			</DialogContent>
		</Dialog>
	);
};

export default AddGuidePopup;
