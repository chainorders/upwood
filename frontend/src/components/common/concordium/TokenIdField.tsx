import { TextField, TextFieldProps } from "@mui/material";

type TokenId = string;
export default function TokenIdField(
	props: { value?: TokenId; onChange: (value: TokenId) => void; sizeByte: number } & Omit<
		TextFieldProps,
		"value" | "onChange" | "type" | "helperText"
	>
) {
	const { value, onChange, sizeByte, ...rest } = props;
	const maxValue = BigInt(2) ** BigInt(sizeByte * 8) - BigInt(1);
	const minValue = BigInt(0);
	const isValid =
		(sizeByte === 0 && value === "") || (value && BigInt("0x" + value) <= maxValue && BigInt("0x" + value) >= minValue);

	return (
		<TextField
			{...rest}
			error={!isValid}
			type="number"
			value={value ? parseInt(value, 16) : ""}
			onChange={(e) =>
				onChange(
					Number(e.target.value)
						.toString(16)
						.toUpperCase()
						.padStart(sizeByte * 2, "0")
				)
			}
			helperText={`Token Id HEX: ${value || ""}`}
		/>
	);
}
