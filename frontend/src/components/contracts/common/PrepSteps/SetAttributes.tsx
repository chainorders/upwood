import { Stack, TextField, Button } from "@mui/material";
import { useState } from "react";

export type Attributes = {
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
		<Stack>
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
			<Button onClick={() => props.onDone(attrs)} disabled={!isValid}>
				{props.doneButtonText}
			</Button>
		</Stack>
	);
};

export default SetAttributes;
