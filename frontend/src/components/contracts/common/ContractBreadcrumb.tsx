import { Breadcrumbs } from "@mui/material";
import { capitalCase } from "change-case";
import { Link } from "react-router-dom";
import { ContractType } from "../ContractTypes";

export default function ContractBreadcrumb(props: {
	contractType: ContractType;
	index: string;
	subIndex: string;
	entrypointDisplayNames: Record<string, string>;
	path?: string;
}) {
	const { index, subIndex, path = "" } = props;
	return (
		<Breadcrumbs>
			<Link to="/">Contracts</Link>
			<Link to={`/contracts/${props.contractType}/${index}/${subIndex}`}>
				{`${capitalCase(props.contractType)}<${index},${subIndex}>`}
			</Link>
			<Link
				color="text.primary"
				to={`/contracts/${props.contractType}/${index}/${subIndex}/${path}`}
			>
				{props.entrypointDisplayNames[path] || path}
			</Link>
		</Breadcrumbs>
	);
}
