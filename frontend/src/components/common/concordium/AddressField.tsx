import { AccountAddress, Address, ContractAddress } from "@concordium/web-sdk";
import { MenuItem, Select, SelectChangeEvent, Stack } from "@mui/material";
import { useState } from "react";
import AccountAddressField from "./AccountAddressField";
import ContractAddressField from "./ContractAddressField";

type AddressType = "AddressAccount" | "AddressAccount";
export interface ConcordiumAddressFieldProps {
	onChange: (address?: Address) => void;
	value?: Address;
	name?: string;
	helperText?: string;
}

export default function AddressField(props: ConcordiumAddressFieldProps) {
	const [type, setType] = useState<AddressType>(
		(props.value?.type as AddressType) || "AddressAccount",
	);
	const [value, setValue] = useState(props.value);

	const handleTypeChange = (e: SelectChangeEvent) => {
		setType(e.target.value as AddressType);
		setValue(undefined);
	};

	const handleChange = (address?: Address) => {
		setValue(address);
		props.onChange(address);
	};

	return (
		<Stack direction={"row"}>
			<Select
				onChange={handleTypeChange}
				value={type}
				sx={{ p: "0px!" }}
				name={(props.name || "").concat("address-type")}
			>
				<MenuItem value="AddressAccount">Account</MenuItem>
				<MenuItem value="AddressContract">Contract</MenuItem>
			</Select>
			{
				{
					AddressAccount: (
						<AccountAddressField
							onChange={(address) =>
								handleChange(
									address ? { type: "AddressAccount", address } : undefined,
								)
							}
							name={props.name}
							helperText={props.helperText}
							value={value?.address as AccountAddress.Type}
						/>
					),
					AddressContract: (
						<ContractAddressField
							onChange={(address) =>
								handleChange(
									address ? { type: "AddressContract", address } : undefined,
								)
							}
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
							value={value?.address as ContractAddress.Type}
						/>
					),
				}[type]
			}
		</Stack>
	);
}
