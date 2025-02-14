import { useForm, Controller } from "react-hook-form";
import { Dialog, DialogTitle, DialogContent, TextField, Button } from "@mui/material";
import { ForestProjectPrice, ForestProjectService, TokenMetadata } from "../../../apiClient";
import { formatDate } from "../../../lib/conversions";

interface AddPricePopupProps {
	open: boolean;
	onClose: () => void;
	euroEMetadata?: TokenMetadata;
	projectId: string;
}

export default function AddPricePopup({ open, onClose, projectId, euroEMetadata }: AddPricePopupProps) {
	const { control, handleSubmit, reset } = useForm<ForestProjectPrice>({
		defaultValues: {
			price_at: new Date().toISOString(),
			currency_token_id: euroEMetadata?.token_id || "",
			currency_token_contract_address: euroEMetadata?.contract_address || "",
			project_id: projectId,
		},
	});

	const onSubmit = async (data: ForestProjectPrice) => {
        try {
            data.price_at = formatDate(new Date(data.price_at));
			await ForestProjectService.postAdminForestProjectsPrice(projectId, data);
			onClose();
			reset();
		} catch (error) {
			console.error("Failed to add price", error);
		}
	};

	return (
		<Dialog open={open} onClose={onClose} fullWidth>
			<DialogTitle>Add Price</DialogTitle>
			<DialogContent>
				<form onSubmit={handleSubmit(onSubmit)}>
					<Controller
						name="price"
						control={control}
						defaultValue=""
						render={({ field }) => <TextField {...field} label="Price" fullWidth margin="normal" />}
					/>
					<Controller
						name="price_at"
						control={control}
						defaultValue=""
						render={({ field }) => <TextField {...field} label="Price At" type="datetime-local" fullWidth margin="normal" />}
					/>
					<Controller
						name="currency_token_id"
						control={control}
						defaultValue=""
						render={({ field }) => <TextField {...field} label="Currency Token ID" fullWidth margin="normal" />}
					/>
					<Controller
						name="currency_token_contract_address"
						control={control}
						defaultValue=""
						render={({ field }) => <TextField {...field} label="Currency Token Contract Address" fullWidth margin="normal" />}
					/>
					<Controller
						name="project_id"
						control={control}
						defaultValue={projectId}
						render={({ field }) => <TextField {...field} label="Project ID" fullWidth margin="normal" disabled />}
					/>
					<Button type="submit" variant="contained" color="primary" fullWidth>
						Add Price
					</Button>
				</form>
			</DialogContent>
		</Dialog>
	);
}
