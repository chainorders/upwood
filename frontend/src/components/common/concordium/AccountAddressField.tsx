import { AccountAddress } from "@concordium/web-sdk";
import { TextField, TextFieldProps } from "@mui/material";
import { useState } from "react";

export interface AccountAddressFieldProps
	extends Omit<TextFieldProps, "onChange" | "value"> {
	onChange?: (address: AccountAddress.Type) => void;
	value?: AccountAddress.Type;
	disabled?: boolean;
}

export default function AccountAddressField(props: AccountAddressFieldProps) {
	const [error, setError] = useState("");

	const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
		setError("");

		try {
			const address = AccountAddress.fromBase58(e.target.value);
			props.onChange && props.onChange(address);
		} catch (e: unknown) {
			setError(e instanceof Error ? e.message : "Unknown error");
		}
	};

	return (
		<TextField
			{...props}
			name="account-address"
			fullWidth
			error={!!error}
			value={props.value?.address || ""}
			onChange={handleChange}
			disabled={props.disabled}
			helperText={
				(props.helperText ? props.helperText + " " : "") + "Account Address"
			}
		/>
	);
}
