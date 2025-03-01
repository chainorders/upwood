import React from "react";
import { useForm, Controller } from "react-hook-form";
import { Dialog, DialogActions, DialogContent, DialogTitle, TextField, Button } from "@mui/material";
import { PlatformUpdate, UserCommunicationService } from "../../apiClient";
import { v4 as uuid } from "uuid";
import { formatDate } from "../../lib/conversions";

const PlatformUpdateMessageCreatePopup = ({
	open,
	onClose,
	onAdd,
}: {
	open: boolean;
	onClose: () => void;
	onAdd: () => void;
}) => {
	const { control, handleSubmit, reset } = useForm<PlatformUpdate>({
		defaultValues: {
			order_index: Math.ceil(new Date().getTime() / 1000),
		},
	});

	const onSubmit = async (data: PlatformUpdate) => {
		data.id = uuid();
		data.created_at = formatDate(new Date());
		data.order_index = Number(data.order_index);
		await UserCommunicationService.postAdminPlatformUpdates(data);
		onAdd();
		reset();
		onClose();
	};

	return (
		<Dialog open={open} onClose={onClose}>
			<DialogTitle>Add Platform Update Message</DialogTitle>
			<DialogContent>
				<form onSubmit={handleSubmit(onSubmit)}>
					<Controller
						name="title"
						control={control}
						defaultValue=""
						render={({ field }) => <TextField {...field} label="Title" fullWidth margin="normal" />}
					/>
					<Controller
						name="label"
						control={control}
						defaultValue=""
						render={({ field }) => <TextField {...field} label="Label" fullWidth margin="normal" />}
					/>
					<Controller
						name="order_index"
						control={control}
						defaultValue={0}
						render={({ field }) => <TextField {...field} label="Order Index" type="number" fullWidth margin="normal" />}
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

export default PlatformUpdateMessageCreatePopup;
