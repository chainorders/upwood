import React from "react";
import { useForm, Controller } from "react-hook-form";
import { Button, Dialog, DialogActions, DialogContent, DialogTitle, TextField } from "@mui/material";
import { UserCommunicationService } from "../../apiClient";
import { v4 as uuid } from "uuid";
import { MaintenanceMessage } from "../../apiClient/models/MaintenanceMessage";
import { daysSince, formatDate } from "../../lib/conversions";

interface AddMaintenanceMsgPopupProps {
	open: boolean;
	onClose: () => void;
	onAdd: () => void;
}

const AddMaintenanceMsgPopup: React.FC<AddMaintenanceMsgPopupProps> = ({ open, onClose, onAdd }) => {
	const { control, handleSubmit, reset } = useForm<MaintenanceMessage>({
		defaultValues: {
			message: "",
			created_at: new Date().toISOString(),
			order_index: Math.ceil(new Date().getTime() / 1000),
		},
	});

	const onSubmit = async (data: MaintenanceMessage) => {
		data.id = uuid();
		data.created_at = formatDate(new Date());
		data.order_index = Number(data.order_index);
		await UserCommunicationService.postAdminMaintenanceMessages(data);
		reset();
		onClose();
		onAdd();
	};

	return (
		<Dialog open={open} onClose={onClose}>
			<DialogTitle>Add Maintenance Message</DialogTitle>
			<DialogContent>
				<form onSubmit={handleSubmit(onSubmit)}>
					<Controller
						name="message"
						control={control}
						rules={{ required: true }}
						render={({ field }) => <TextField {...field} autoFocus margin="dense" label="Message" type="text" fullWidth />}
					/>
					<Controller
						name="order_index"
						control={control}
						render={({ field }) => <TextField {...field} margin="dense" label="Order Index" type="number" fullWidth />}
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

export default AddMaintenanceMsgPopup;
