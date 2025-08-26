import { TextField, TextFieldProps } from "@mui/material";
import { Control, Controller, FieldValues, Path } from "react-hook-form";

interface IntegerInputProps<T extends FieldValues> {
	name: Path<T>;
	control: Control<T>;
	label: string;
	min?: number;
	step?: number;
	textFieldProps?: Omit<TextFieldProps, "name" | "label" | "type" | "onChange" | "value" | "error" | "helperText">;
}

function IntegerInput<T extends FieldValues>({
	name,
	control,
	label,
	min = 1,
	step = 1,
	textFieldProps,
}: IntegerInputProps<T>) {
	// Helper function to parse input as integer
	const parseIntegerInput = (value: string): number => {
		const parsed = parseInt(value, 10);
		return isNaN(parsed) ? 0 : parsed;
	};

	return (
		<Controller
			name={name}
			control={control}
			rules={{
				required: "Value is required",
				validate: {
					isNumber: (value) => Number.isInteger(Number(value)) || "Please enter a valid integer",
					positive: (value) => Number(value) > 0 || "Value must be greater than 0",
				},
			}}
			render={({ field, fieldState }) => (
				<TextField
					{...field}
					{...textFieldProps}
					label={label}
					type="number"
					inputProps={{ min, step }}
					onChange={(e) => {
						const intValue = parseIntegerInput(e.target.value);
						field.onChange(intValue);
					}}
					error={!!fieldState.error}
					helperText={fieldState.error?.message}
					fullWidth
					variant="outlined"
					size="small"
				/>
			)}
		/>
	);
}

export default IntegerInput;
