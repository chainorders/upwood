import { Breadcrumbs } from "@mui/material";
import { capitalCase } from "change-case";
import { Link } from "react-router-dom";

export default function ContractBreadcrumb(props: {
	contractType: string;
	index: string;
	subIndex: string;
	entrypointDisplayNames: Record<string, string>;
	path?: string;
}) {
	const { index, subIndex, path = "" } = props;
	return (
		<Breadcrumbs>
			<Link to="/contracts">Contracts</Link>
			<Link to={`/contracts/${props.contractType}/${index}/${subIndex}`}>
				{`${capitalCase(props.contractType)} <${index},${subIndex}>`}
			</Link>
			{props.entrypointDisplayNames[path] && (
				<Link
					to={`/contracts/${props.contractType}/${index}/${subIndex}/${path}`}
				>
					{props.entrypointDisplayNames[path]}
				</Link>
			)}
		</Breadcrumbs>
	);
}
