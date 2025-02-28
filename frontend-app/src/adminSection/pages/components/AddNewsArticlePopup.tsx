import React from "react";
import { Dialog, DialogActions, DialogContent, DialogTitle, Button } from "@mui/material";
import { useForm, Controller } from "react-hook-form";
import { NewsArticle, UserCommunicationService } from "../../../apiClient";
import TextField from "@mui/material/TextField";
import { formatDate } from "../../../lib/conversions";
import { v4 as uuid } from "uuid";

interface AddNewsArticlePopupProps {
	open: boolean;
	onClose: () => void;
	onRefresh: () => void;
}

const AddNewsArticlePopup: React.FC<AddNewsArticlePopupProps> = ({ open, onClose, onRefresh }) => {
	const { control, handleSubmit, reset } = useForm<NewsArticle>();

	const onSubmit = (data: NewsArticle) => {
		data.id = uuid();
		data.created_at = formatDate(new Date());
		data.order_index = Number(data.order_index);
		console.log(data);
		UserCommunicationService.postAdminNewsArticles(data).then(() => {
			onRefresh();
			reset();
		});
	};

	return (
		<Dialog open={open} onClose={onClose}>
			<DialogTitle>Add New Article</DialogTitle>
			<DialogContent>
				<form onSubmit={handleSubmit(onSubmit)}>
					<Controller
						name="title"
						control={control}
						defaultValue=""
						render={({ field }) => <TextField {...field} autoFocus margin="dense" label="Title" fullWidth />}
					/>
					<Controller
						name="label"
						control={control}
						defaultValue=""
						render={({ field }) => <TextField {...field} margin="dense" label="Label" fullWidth />}
					/>
					<Controller
						name="content"
						control={control}
						defaultValue=""
						render={({ field }) => <TextField {...field} margin="dense" label="Content" fullWidth />}
					/>
					<Controller
						name="image_url"
						control={control}
						defaultValue=""
						render={({ field }) => <TextField {...field} margin="dense" label="Image URL" fullWidth />}
					/>
					<Controller
						name="article_url"
						control={control}
						defaultValue=""
						render={({ field }) => <TextField {...field} margin="dense" label="Article URL" fullWidth />}
					/>
					<Controller
						name="order_index"
						control={control}
						defaultValue={0}
						render={({ field }) => <TextField {...field} margin="dense" label="Order Index" fullWidth type="number" />}
					/>
					<DialogActions>
						<Button onClick={onClose} color="primary">
							Cancel
						</Button>
						<Button type="submit" color="primary">
							Save
						</Button>
					</DialogActions>
				</form>
			</DialogContent>
		</Dialog>
	);
};

export default AddNewsArticlePopup;
