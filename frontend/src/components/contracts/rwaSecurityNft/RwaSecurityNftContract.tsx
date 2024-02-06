import { Stack, Grid, Paper, List, Divider, ListItem, ListItemButton, ListItemText, Typography } from "@mui/material";
import { Navigate, Route, Routes, useLocation, useNavigate, useParams } from "react-router-dom";
import { ContractAddress } from "@concordium/web-sdk";
import EntrypointsList from "../../common/EntrypointsList";
import { ENTRYPOINTS, ENTRYPOINT_DISPLAY_NAMES } from "../../../lib/rwaSecurityNft";
import { ENTRYPOINTS_UI } from "../../../lib/rwaSecurityNftUi";
import TokenList from "./TokensList";
import { RegistryWidgetsType, UiSchema } from "@rjsf/utils";

const entrypoints_ui_customizations: Record<
	keyof typeof ENTRYPOINTS_UI,
	{ uiSchema?: UiSchema; uiWidgets?: RegistryWidgetsType }
> = {};

export default function RwaSecurityNftContract() {
	const { index, subIndex } = useParams();
	const contract = ContractAddress.create(BigInt(index!), BigInt(subIndex!));
	const navigate = useNavigate();
	const { pathname } = useLocation();
	const paths = pathname.split("/");
	const path = paths[paths.length - 1];

	return (
		<Stack>
			<Grid container spacing={1}>
				<Grid item xs={12} md={9}>
					<Paper variant="outlined" sx={{ border: 0 }}>
						<Routes>
							<Route path="tokens" element={<TokenList contract={contract} />} />
							{Object.keys(ENTRYPOINTS).map((entrypoint) => (
								<Route
									key={entrypoint}
									path={entrypoint}
									element={ENTRYPOINTS_UI[entrypoint]({
										contract,
										uiSchema: entrypoints_ui_customizations[entrypoint]?.uiSchema,
										uiWidgets: entrypoints_ui_customizations[entrypoint]?.uiWidgets,
									})}
								/>
							))}
							<Route path="*" element={<Navigate to="tokens" />} />
						</Routes>
					</Paper>
				</Grid>
				<Grid item xs={12} md={3}>
					<Paper>
						<List>
							<ListItem disablePadding>
								<ListItemButton onClick={() => navigate("tokens")} selected={path === "tokens"}>
									<ListItemText primary="Tokens" />
								</ListItemButton>
							</ListItem>
						</List>
						<Divider />
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
