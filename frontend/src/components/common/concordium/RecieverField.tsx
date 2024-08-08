import { MenuItem, Select, Stack, TextField } from "@mui/material";
import AccountAddressField from "./AccountAddressField";
import {
	AccountAddress,
	ContractAddress,
	EntrypointName,
} from "@concordium/web-sdk";
import ContractAddressField from "./ContractAddressField";
import { useState } from "react";

export type Receiver =
	| {
			type: "AddressAccount";
			address: AccountAddress.Type;
	  }
	| {
			type: "AddressContract";
			address: ContractAddress.Type;
			entrypoint: EntrypointName.Type;
	  };

export default function ReceiverField(props: {
	value?: Receiver;
	onChange: (value?: Receiver) => void;
	helperText?: string;
	name?: string;
}) {
	const defaultType = "AddressAccount";
	const { value } = props;
	const [form, setForm] = useState<{
		type: string;
		address?: AccountAddress.Type | ContractAddress.Type;
		entrypoint: string;
	}>({
		type: value?.type || defaultType,
		address: value?.address,
		entrypoint:
			value?.type === "AddressContract" ? value?.entrypoint.value : "",
	});

	const setType = (type: string) => {
		setForm({ type, entrypoint: "", address: undefined });
	};

	const setFormValue = (key: keyof typeof form, value: unknown) => {
		const newForm = { ...form, [key]: value };
		setForm(newForm);

		if (
			newForm.type === "AddressContract" &&
			newForm.address &&
			newForm.entrypoint
		) {
			props.onChange({
				type: "AddressContract",
				address: newForm.address as ContractAddress.Type,
				entrypoint: EntrypointName.fromString(newForm.entrypoint),
			});
		} else if (newForm.type === "AddressAccount" && newForm.address) {
			props.onChange({
				type: "AddressAccount",
				address: newForm.address as AccountAddress.Type,
			});
		} else {
			props.onChange(undefined);
		}
	};

	return (
		<Stack spacing={0} direction={"row"}>
			<Select value={form.type} onChange={(e) => setType(e.target.value)}>
				<MenuItem value="AddressAccount">Account</MenuItem>
				<MenuItem value="AddressContract">Contract</MenuItem>
			</Select>
			{
				{
					AddressAccount: (
						<AccountAddressField
							onChange={(address) => setFormValue("address", address)}
							name={props.name}
							helperText={props.helperText}
							value={form.address as AccountAddress.Type}
						/>
					),
					AddressContract: (
						<>
							<ContractAddressField
								onChange={(address) => setFormValue("address", address)}
								indexHelperText={
									(props.helperText ? props.helperText + " " : "") +
									"Contract Index"
								}
								subIndexHelperText={
									(props.helperText ? props.helperText + " " : "") +
									"Contract SubIndex"
								}
								indexName={props.name?.concat("index")}
								subIndexName={props.name?.concat("subIndex")}
								value={form.address as ContractAddress.Type}
							/>
							<TextField
								label="Entrypoint"
								value={form.entrypoint}
								disabled={form.address === undefined}
								onChange={(e) => setFormValue("entrypoint", e.target.value)}
							/>
						</>
					),
				}[form.type]
			}
		</Stack>
	);
}
