import { ContractAddress } from "@concordium/web-sdk";
import { Typography } from "@mui/material";
import { getContractName } from "./types";

export default function ContractName(props: {
	contract: ContractAddress.Type;
	variant?:
		| "inherit"
		| "h6"
		| "h5"
		| "h4"
		| "h3"
		| "h2"
		| "h1"
		| "subtitle1"
		| "subtitle2"
		| "body1"
		| "body2"
		| "caption"
		| "button"
		| "overline"
		| undefined;
	fontSize?: string;
}) {
	const contractName = getContractName(props.contract);

	return (
		<Typography
			variant={props.variant || "inherit"}
			noWrap
			fontSize={props.fontSize}
		>
			{contractName}
		</Typography>
	);
}
