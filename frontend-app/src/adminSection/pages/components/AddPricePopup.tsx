import { useForm, Controller } from "react-hook-form";
import { Dialog, DialogTitle, DialogContent, TextField, Button } from "@mui/material";
import { ForestProjectPrice, ForestProjectService, TokenMetadata } from "../../../apiClient";
import { formatDate, toDisplayAmount } from "../../../lib/conversions";
interface AddPricePopupProps {
	open: boolean;
	onClose: () => void;
	euroEMetadata: TokenMetadata;
	projectId: string;
}

export default function AddPricePopup({ open, onClose, projectId, euroEMetadata }: AddPricePopupProps) {
	const { control, handleSubmit, reset } = useForm<ForestProjectPrice>({
		defaultValues: {
			currency_token_id: euroEMetadata.token_id,
			currency_token_contract_address: euroEMetadata.contract_address,
			project_id: projectId,
			price_at: formatDate(new Date()),
		},
	});

	const onSubmit = async (data: ForestProjectPrice) => {
		try {
			data.price_at = formatDate(new Date(data.price_at));
			data.currency_token_id = euroEMetadata.token_id;
			data.currency_token_contract_address = euroEMetadata.contract_address;
			data.project_id = projectId;
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
						render={({ field }) => (
							<TextField
								{...field}
								label="Price"
								type="number"
								fullWidth
								margin="normal"
								helperText={`${toDisplayAmount(field.value || "0", euroEMetadata.decimals || 6, euroEMetadata.decimals || 6)}${euroEMetadata.symbol || ""}`}
							/>
						)}
					/>
					<Controller
						name="price_at"
						control={control}
						render={({ field }) => <TextField {...field} label="Price At" type="datetime-local" fullWidth margin="normal" />}
					/>
					<Controller
						name="currency_token_id"
						control={control}
						disabled
						render={({ field }) => <TextField {...field} label="Currency Token ID" fullWidth margin="normal" />}
					/>
					<Controller
						name="currency_token_contract_address"
						control={control}
						disabled
						render={({ field }) => <TextField {...field} label="Currency Token Contract Address" fullWidth margin="normal" />}
					/>
					<Controller
						name="project_id"
						control={control}
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
