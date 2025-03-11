import React from "react";
import { useForm, Controller } from "react-hook-form";
import { v4 as uuidv4 } from "uuid";
import { Dialog, DialogTitle, DialogContent, DialogActions, Button, TextField } from "@mui/material";
import { ForestProjectMedia, ForestProjectService } from "../../../apiClient";
import ImageUploader from "../../components/ImageUploader";
import { adminUploadImage } from "../../libs/utils";

interface AddMediaPopupProps {
	open: boolean;
	onClose: () => void;
	projectId: string;
	fileBaseUrl: string;
}

const AddMediaPopup: React.FC<AddMediaPopupProps> = ({ open, onClose, projectId, fileBaseUrl }) => {
	const { control, handleSubmit, reset, setValue, watch } = useForm<ForestProjectMedia>();

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
							<TextField
								{...field}
								autoFocus
								margin="dense"
								label="Image URL"
								type="url"
								fullWidth
								variant="standard"
								disabled
								InputLabelProps={{ shrink: !!field.value }}
							/>
						)}
					/>
					<ImageUploader
						aspectRatio={2.5}
						onChange={(v) => adminUploadImage(fileBaseUrl, "project_media", v).then((url) => setValue("image_url", url))}
						url={watch("image_url")}
					/>
				</DialogContent>
				<DialogActions>
					<Button onClick={onClose}>Cancel</Button>
					<Button type="submit" color="primary" disabled={!watch("image_url")}>
						Add
					</Button>
				</DialogActions>
			</form>
		</Dialog>
	);
};

export default AddMediaPopup;
