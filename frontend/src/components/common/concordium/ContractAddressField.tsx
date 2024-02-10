import { ContractAddress } from "@concordium/web-sdk";
import { Stack, TextField, TextFieldProps } from "@mui/material";
import { useEffect, useState } from "react";

export interface ContractAddressFieldProps {
	onChange: (address?: ContractAddress.Type) => void;
	value?: ContractAddress.Type;
	indexName?: string;
	subIndexName?: string;
	indexHelperText?: string;
	subIndexHelperText?: string;
	textFieldCommonProps?: Omit<
		TextFieldProps,
		"onChange" | "value" | "name" | "helperText"
	>;
	disabled?: boolean;
	readonly?: boolean;
}

export default function ContractAddressField(props: ContractAddressFieldProps) {
	const [error, setError] = useState("");
	const [values, setValues] = useState({
		index: props.value?.index.toString() || "",
		subIndex: props.value?.subindex.toString() || "0",
	});
	const handleChange = (key: keyof typeof values, value: string) => {
		setError("");
		const newValues = { ...values, [key]: value };
		setValues(newValues);

		if (newValues.index && newValues.subIndex) {
			try {
				const contract = ContractAddress.create(
					BigInt(newValues.index),
					BigInt(newValues.subIndex),
				);
				props.onChange(contract);
				setError("");
			} catch (e: unknown) {
				setError(e instanceof Error ? e.message : "Unknown error");
			}
		} else {
			props.onChange(undefined);
		}
	};

	useEffect(() => {
		setValues({
			index: props.value?.index.toString() || "",
			subIndex: props.value?.subindex.toString() || "0",
		});
	}, [props?.value]);

	return (
		<Stack direction={"row"} width={"100%"}>
			<TextField
				{...props.textFieldCommonProps}
				disabled={props?.disabled}
				inputProps={{ readOnly: props?.readonly }}
				error={!!error}
				name={props.indexName || "contract-index"}
				helperText={props.indexHelperText}
				value={values.index?.toString()}
				onChange={(e) => handleChange("index", e.target.value)}
				type="number"
				fullWidth
			/>
			<TextField
				{...props.textFieldCommonProps}
				disabled={props?.disabled}
				inputProps={{ readOnly: props?.readonly }}
				error={!!error}
				name={props.subIndexName || "contract-subindex"}
				helperText={props.subIndexHelperText}
				value={values.subIndex?.toString()}
				onChange={(e) => handleChange("subIndex", e.target.value)}
				type="number"
				fullWidth
			/>
		</Stack>
	);
}
