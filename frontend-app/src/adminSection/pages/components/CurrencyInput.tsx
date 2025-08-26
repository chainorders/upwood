import React from "react";
import TextField from "@mui/material/TextField";
import { Control, Controller, FieldValues, Path } from "react-hook-form";

interface CurrencyInputProps<T extends FieldValues> {
	name: Path<T>;
	control: Control<T>;
	label: string;
	textFieldProps?: Omit<
		React.ComponentProps<typeof TextField>,
		"name" | "label" | "onChange" | "value" | "error" | "helperText"
	>;
}

function CurrencyInput<T extends FieldValues>({ name, control, label, textFieldProps }: CurrencyInputProps<T>) {
	const parseInputValue = (inputValue: string): number | string => {
		try {
			const numericValue = parseFloat(inputValue);
			if (isNaN(numericValue)) {
				return inputValue;
			}
			if (numericValue === 0) {
				return inputValue;
			}
			return Math.round(numericValue * 1000000);
		} catch (error) {
			return 0;
		}
	};

	const parseFieldValue = (value: string | number | undefined): string | undefined => {
		if (value === undefined || value === "") return value;
		if (value === null) return value;

		try {
			const numValue = typeof value === "string" ? parseInt(value, 10) : value;
			if (isNaN(numValue)) return value.toString();
			if (numValue === 0) return value.toString();
			if (numValue > Number.MAX_SAFE_INTEGER) {
				return value.toString();
			}
			const floatString = (numValue / 1000000).toString();
			return parseFloat(floatString).toString();
		} catch (error) {
			return value.toString();
		}
	};

	return (
		<Controller
			name={name}
			control={control}
			rules={{
				required: "Currency value is required",
				validate: {
					isValid: (value) => !isNaN(Number(value)) || "Please enter a valid number",
					isPositive: (value) => Number(value) >= 0 || "Currency must be a positive value",
					isBigInt: (value) => {
						const parsedValue = parseFloat(value);
						return parsedValue <= Number.MAX_SAFE_INTEGER || "Value is too large";
					},
				},
			}}
			render={({ field, fieldState }) => (
				<TextField
					{...field}
					{...textFieldProps}
					value={parseFieldValue(field.value)}
					label={label}
					// value={formatDisplayValue(field.value || "")}
					onChange={(event: React.ChangeEvent<HTMLInputElement>) => {
						const inputValue = event.target.value;
						field.onChange(parseInputValue(inputValue));
					}}
					onBlur={field.onBlur}
					error={!!fieldState.error}
					helperText={fieldState.error?.message}
					variant="outlined"
					inputProps={{
						inputMode: "decimal",
						pattern: "[0-9]*.?[0-9]{0,6}",
						...textFieldProps?.inputProps,
					}}
					fullWidth
					size="small"
					type="number"
				/>
			)}
		/>
	);
}

export default CurrencyInput;
