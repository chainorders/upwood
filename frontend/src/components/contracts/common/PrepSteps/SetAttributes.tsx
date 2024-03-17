import {
	Stack,
	TextField,
	Button,
	FormControlLabel,
	Checkbox,
} from "@mui/material";
import { useState } from "react";

export type Attributes = {
	constructionDate?: string;
	longitude?: string;
	latitude?: string;
	unique?: boolean;
	name?: string;
	description?: string;
	symbol?: string;
};
const SetAttributes = (props: {
	onDone: (attributes: Attributes) => void;
	value?: Attributes;
	doneButtonText: string;
}) => {
	const [attrs, setAttrs] = useState<Attributes>(props.value || {});
	const isValid = attrs.name && attrs.description;

	return (
		<Stack spacing={1}>
			<TextField
				label="Name"
				value={attrs.name || ""}
				onChange={(e) => setAttrs({ ...attrs, name: e.target.value })}
			/>
			<TextField
				label="Description"
				value={attrs.description || ""}
				onChange={(e) => setAttrs({ ...attrs, description: e.target.value })}
			/>
			<TextField
				label="Symbol"
				value={attrs.symbol || ""}
				onChange={(e) => setAttrs({ ...attrs, symbol: e.target.value })}
			/>
			<FormControlLabel
				control={
					<Checkbox
						defaultChecked
						value={attrs.unique || false}
						onChange={(e) => setAttrs({ ...attrs, unique: e.target.checked })}
					/>
				}
				label="Is Unique"
			/>
			<TextField
				label="Latitude"
				value={attrs.latitude || ""}
				onChange={(e) => setAttrs({ ...attrs, latitude: e.target.value })}
			/>
			<TextField
				label="Longitude"
				value={attrs.longitude || ""}
				onChange={(e) => setAttrs({ ...attrs, longitude: e.target.value })}
			/>
			<TextField
				type="date"
				label="Construction Date"
				value={attrs.constructionDate || ""}
				onChange={(e) =>
					setAttrs({ ...attrs, constructionDate: e.target.value })
				}
			/>
			<Button onClick={() => props.onDone(attrs)} disabled={!isValid}>
				{props.doneButtonText}
			</Button>
		</Stack>
	);
};

export default SetAttributes;
