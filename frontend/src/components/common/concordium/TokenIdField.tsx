import { TextField, TextFieldProps } from "@mui/material";
import { toTokenIdInt } from "../../../lib/cis2Utils";

type TokenId = string;
export default function TokenIdField(
	props: {
		value?: TokenId;
		onChange: (value: TokenId) => void;
		sizeByte: number;
	} & Omit<TextFieldProps, "value" | "onChange" | "type" | "helperText">,
) {
	const { value, onChange, sizeByte, ...rest } = props;
	const maxValue = BigInt(2) ** BigInt(sizeByte * 8) - BigInt(1);
	const minValue = BigInt(0);
	const isValid =
		(sizeByte === 0 && value === "") ||
		(value &&
			BigInt("0x" + value) <= maxValue &&
			BigInt("0x" + value) >= minValue);

	return (
		<TextField
			{...rest}
			error={!isValid}
			type="number"
			value={value ? toTokenIdInt(value) : ""}
			onChange={(e) =>
				onChange(
					Number(e.target.value)
						.toString(16)
						.toUpperCase()
						.padStart(sizeByte * 2, "0"),
				)
			}
			helperText={`Token Id HEX: ${value || ""}`}
		/>
	);
}
