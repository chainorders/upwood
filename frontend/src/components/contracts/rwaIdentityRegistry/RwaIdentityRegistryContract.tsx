import { Stack, Grid, Paper, Select, MenuItem } from "@mui/material";
import { Route, Routes, useLocation, useParams } from "react-router-dom";
import { AttributesKeys, ContractAddress } from "@concordium/web-sdk";
import EntrypointsList from "../../common/EntrypointsList";
import {
	ENTRYPOINTS,
	ENTRYPOINT_DISPLAY_NAMES,
} from "../../../lib/rwaIdentityRegistry";
import { ENTRYPOINTS_UI } from "../../../lib/rwaIdentityRegistryUi";
import { RegistryWidgetsType, UiSchema, WidgetProps } from "@rjsf/utils";

const AttributeTagSelect = (props: WidgetProps) => {
	return (
		<Select
			value={props.value || 0}
			onChange={(e) => props.onChange(e.target.value)}
		>
			<MenuItem value={AttributesKeys.firstName}>First Name</MenuItem>
			<MenuItem value={AttributesKeys.lastName}>Last Name</MenuItem>
			<MenuItem value={AttributesKeys.countryOfResidence}>
				Country Of Residence
			</MenuItem>
			<MenuItem value={AttributesKeys.dob}>Date of Birth</MenuItem>
			<MenuItem value={AttributesKeys.idDocExpiresAt}>
				Id Doc Expires At
			</MenuItem>
			<MenuItem value={AttributesKeys.idDocIssuedAt}>Id Doc Issued At</MenuItem>
			<MenuItem value={AttributesKeys.idDocIssuer}>Id Doc Issuer</MenuItem>
			<MenuItem value={AttributesKeys.idDocNo}>Id Doc Number</MenuItem>
			<MenuItem value={AttributesKeys.idDocType}>Id Doc Type</MenuItem>
			<MenuItem value={AttributesKeys.nationalIdNo}>National Id No</MenuItem>
			<MenuItem value={AttributesKeys.nationality}>Nationality</MenuItem>
			<MenuItem value={AttributesKeys.sex}>Sex</MenuItem>
			<MenuItem value={AttributesKeys.taxIdNo}>Tax Id No</MenuItem>
		</Select>
	);
};

const entrypoints_ui_customizations: Record<
	keyof typeof ENTRYPOINTS_UI,
	{ uiSchema?: UiSchema; uiWidgets?: RegistryWidgetsType }
> = {
	registerIdentity: {
		uiSchema: {
			identity: {
				attributes: {
					items: {
						tag: {
							"ui:widget": "attributeTagSelect",
						},
					},
				},
			},
		},
		uiWidgets: {
			attributeTagSelect: AttributeTagSelect,
		},
	},
};

export default function RwaIdentityRegistryContract() {
	const { index, subIndex } = useParams();
	const contract = ContractAddress.create(BigInt(index!), BigInt(subIndex!));
	const { pathname } = useLocation();
	const paths = pathname.split("/");
	const path = paths[paths.length - 1];

	AttributesKeys;

	return (
		<Stack>
			<Grid container spacing={1}>
				<Grid item xs={12} md={9}>
					<Paper variant="outlined" sx={{ border: 0 }}>
						<Routes>
							{Object.keys(ENTRYPOINTS).map((entrypoint) => (
								<Route
									key={entrypoint}
									path={entrypoint}
									element={ENTRYPOINTS_UI[entrypoint]({
										contract,
										uiSchema:
											entrypoints_ui_customizations[entrypoint]?.uiSchema,
										uiWidgets:
											entrypoints_ui_customizations[entrypoint]?.uiWidgets,
									})}
								/>
							))}
						</Routes>
					</Paper>
				</Grid>
				<Grid item xs={12} md={3}>
					<Paper>
						<EntrypointsList
							entrypoints={ENTRYPOINTS}
							entrypointDisplayNames={ENTRYPOINT_DISPLAY_NAMES}
							selectedPath={path}
						/>
					</Paper>
				</Grid>
			</Grid>
		</Stack>
	);
}
