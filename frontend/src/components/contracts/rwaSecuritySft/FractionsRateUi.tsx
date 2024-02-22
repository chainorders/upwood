import { FieldProps } from "@rjsf/utils";
import { AddTokensRequestUi } from "../../../lib/rwaSecuritySftUi";
import { Flatten } from "../../market/types";
import { Stack, TextField, Typography } from "@mui/material";

type UiType = Flatten<AddTokensRequestUi["tokens"]>["fractions_rate"];
const toUi = (value: number): UiType => ({
	numerator: value || 0,
	denominator: 1,
});
const fromUi = (value: UiType): number => value.numerator / value.denominator;
const FractionsRateUi = (props: FieldProps) => {
	const { formData: value, onChange } = props;
	return (
		<Stack spacing={1}>
			<Typography variant="h5">Fractions</Typography>
			<TextField
				type="number"
				value={value ? fromUi(value) : 0}
				onChange={(e) => onChange(toUi(Number(e.target.value)))}
			/>
		</Stack>
	);
};
export default FractionsRateUi;
