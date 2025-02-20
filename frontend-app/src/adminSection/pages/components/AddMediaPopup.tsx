import React from "react";
import { useForm, Controller } from "react-hook-form";
import { v4 as uuidv4 } from "uuid";
import { Dialog, DialogTitle, DialogContent, DialogActions, Button, TextField } from "@mui/material";
import { ForestProjectMedia, ForestProjectService } from "../../../apiClient";

interface AddMediaPopupProps {
	open: boolean;
	onClose: () => void;
	projectId: string;
}

const AddMediaPopup: React.FC<AddMediaPopupProps> = ({ open, onClose, projectId }) => {
	const { control, handleSubmit, reset } = useForm<ForestProjectMedia>();

	const onSubmit = (data: ForestProjectMedia) => {
		data.id = uuidv4();
		data.project_id = projectId;
		ForestProjectService.postAdminForestProjectsMedia(projectId, data).then(() => {
			onClose();
			reset();
		});
	};

	return (
		<Dialog open={open} onClose={onClose}>
			<DialogTitle>Add Media</DialogTitle>
			<form onSubmit={handleSubmit(onSubmit)}>
				<DialogContent>
					<Controller
						name="image_url"
						control={control}
						defaultValue=""
						render={({ field }) => (
							<TextField {...field} autoFocus margin="dense" label="Image URL" type="url" fullWidth variant="standard" />
						)}
					/>
				</DialogContent>
				<DialogActions>
					<Button onClick={onClose}>Cancel</Button>
					<Button type="submit" color="primary">
						Add
					</Button>
				</DialogActions>
			</form>
		</Dialog>
	);
};

export default AddMediaPopup;
