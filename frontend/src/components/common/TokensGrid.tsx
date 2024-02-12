import { Box, Grid, Pagination, Paper, Stack, Typography } from "@mui/material";
import TokenCardDisplay, { ActionButtonProps, Token } from "./TokenCardDisplay";
import ErrorDisplay from "./ErrorDisplay";
import { ConcordiumGRPCClient } from "@concordium/web-sdk";

type Props = {
	grpcClient: ConcordiumGRPCClient;
	tokens: Token[];
	onPageChange: (page: number) => void;
	error: string;
	loading: boolean;
	pageCount: number;
	page: number;
	actions: Omit<ActionButtonProps, "token">[];
};
export default function TokensGrid(props: Props) {
	const {
		grpcClient,
		tokens,
		onPageChange,
		error,
		loading,
		pageCount,
		page,
		actions,
	} = props;

	return (
		<Paper variant="outlined" sx={{ p: 2 }}>
			<Stack spacing={2}>
				<Grid container spacing={1}>
					{tokens.map((token, index) => (
						<Grid item xs={12} md={2} key={index}>
							<TokenCardDisplay
								key={index}
								grpcClient={grpcClient}
								token={token}
								actions={actions.map((props) => ({ token, ...props }))}
							/>
						</Grid>
					))}
				</Grid>
				<Box sx={{ display: "flex", justifyContent: "center" }}>
					<Pagination
						count={pageCount}
						page={page + 1}
						variant="outlined"
						shape="rounded"
						onChange={(_e, v) => onPageChange(v)}
					/>
					{error && <ErrorDisplay text={error} />}
					{loading && <Typography>Loading...</Typography>}
				</Box>
			</Stack>
		</Paper>
	);
}
