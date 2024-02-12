import {
	Avatar,
	Button,
	Card,
	CardActions,
	CardContent,
	CardHeader,
	CardMedia,
	Chip,
	Collapse,
	IconButton,
	IconButtonProps,
	Skeleton,
	Stack,
	Typography,
	styled,
} from "@mui/material";
import { MarketToken } from "../../lib/contracts-api-client";
import { red } from "@mui/material/colors";
import {
	ArrowBack,
	CurrencyExchange,
	ExpandCircleDown,
	SellSharp,
} from "@mui/icons-material";
import {
	CIS2,
	CIS2Contract,
	ConcordiumGRPCClient,
	ContractAddress,
} from "@concordium/web-sdk";
import { useEffect, useState } from "react";
import {
	TokenMetadata,
	getTokenMetadata,
	toDataUrl,
} from "../../lib/cis2Utils";

type Props = {
	token: MarketToken;
	grpcClient: ConcordiumGRPCClient;
	onList?: (token: MarketToken) => void;
	onWithdraw?: (token: MarketToken) => void;
	onDeList?: (token: MarketToken) => void;
	onExchange?: (token: MarketToken) => void;
	disabled?: boolean;
};
interface ExpandMoreProps extends IconButtonProps {
	expand: boolean;
}
const ExpandMore = styled((props: ExpandMoreProps) => {
	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	const { expand, ...other } = props;
	return <IconButton {...other} />;
})(({ theme, expand }) => ({
	transform: !expand ? "rotate(0deg)" : "rotate(180deg)",
	marginLeft: "auto",
	transition: theme.transitions.create("transform", {
		duration: theme.transitions.duration.shortest,
	}),
}));
export default function TokenDisplay(props: Props) {
	const {
		grpcClient,
		token,
		disabled,
		onList,
		onWithdraw,
		onDeList,
		onExchange,
	} = props;
	const [metadataUrl, setMetadataUrl] = useState<CIS2.MetadataUrl | undefined>(
		undefined,
	);
	const [loadingMetadataUrl, setLoadingMetadataUrl] = useState(false);
	const [metadata, setMetadata] = useState<TokenMetadata | undefined>(
		undefined,
	);
	const [loadingMetadata, setLoadingMetadata] = useState(false);
	const [imageUrl, setImageUrl] = useState<string | undefined>(undefined);
	const [expanded, setExpanded] = useState(false);
	const handleExpandClick = () => {
		setExpanded(!expanded);
	};

	const setError = (error: string) => {
		console.error(error);
	};

	useEffect(() => {
		setMetadataUrl(undefined);
		setMetadata(undefined);
		setImageUrl(undefined);
		setLoadingMetadataUrl(true);
		CIS2Contract.create(
			grpcClient,
			ContractAddress.create(
				token.token_contract.index,
				token.token_contract.subindex,
			),
		)
			.then((client) => client.tokenMetadata(token.token_id))
			.then((metadata) => setMetadataUrl(metadata))
			.catch((error) => setError(error.message))
			.finally(() => setLoadingMetadataUrl(false));
	}, [token, grpcClient]);
	useEffect(() => {
		setMetadata(undefined);
		setImageUrl(undefined);
		if (!metadataUrl) {
			return;
		}
		setLoadingMetadata(true);
		getTokenMetadata(metadataUrl)
			.then((metadata) => setMetadata(metadata))
			.catch((error) => setError(error.message))
			.finally(() => setLoadingMetadata(false));
	}, [metadataUrl]);
	useEffect(() => {
		setImageUrl(undefined);
		if (metadata && metadata.display?.url) {
			toDataUrl(metadata.display?.url)
				.then((url) => setImageUrl(url))
				.catch((error) => setError(error.message));
		} else if (metadata && metadata.thumbnail?.url) {
			toDataUrl(metadata.thumbnail?.url)
				.then((url) => setImageUrl(url))
				.catch((error) => setError(error.message));
		} else {
			setImageUrl(undefined);
		}
	}, [metadata]);

	return (
		<Card>
			<CardHeader
				title={`Token Id: ${token.token_id}`}
				subheader={`${token.token_contract.index}/${token.token_contract.subindex}`}
				avatar={
					<Avatar sx={{ bgcolor: red[500] }} aria-label="recipe">
						T
					</Avatar>
				}
			/>
			{imageUrl && (
				<CardMedia
					component="img"
					alt={`token: ${token.token_id} Image`}
					image={imageUrl}
				/>
			)}
			{loadingMetadata || loadingMetadataUrl ? (
				<Skeleton variant="rectangular" width="100%" height={118} />
			) : (
				<></>
			)}

			<Collapse in={expanded} timeout="auto" unmountOnExit>
				<CardContent>
					<Stack spacing={0}>
						{metadata?.description && (
							<Typography paragraph>{metadata.description}</Typography>
						)}
						{metadata?.attributes && metadata?.attributes.length > 0 && (
							<>
								{metadata.attributes.map((attribute, index) => (
									<Chip
										key={index}
										label={`${attribute.name}:${attribute.value}`}
										variant="outlined"
									/>
								))}
							</>
						)}
					</Stack>
				</CardContent>
			</Collapse>
			<CardActions disableSpacing>
				{onWithdraw && (
					<Button
						sx={{ mr: 1 }}
						aria-label="Return to Owner"
						variant="text"
						title="Withdraw"
						disabled={disabled}
						onClick={() => onWithdraw!(token)}
					>
						<ArrowBack sx={{ pr: 1 }} /> Return
					</Button>
				)}
				{onList && (
					<Button
						aria-label="Sell on Market"
						variant="contained"
						title="List"
						color="primary"
						disabled={disabled}
						onClick={() => onList!(token)}
					>
						<SellSharp sx={{ pr: 1 }} /> List
					</Button>
				)}
				{onDeList && (
					<Button
						sx={{ mr: 1 }}
						aria-label="De List"
						variant="text"
						title="De List"
						disabled={disabled}
						onClick={() => onDeList!(token)}
					>
						<ArrowBack sx={{ pr: 1 }} /> De List
					</Button>
				)}
				{onExchange && (
					<Button
						sx={{ mr: 1 }}
						aria-label="exchange"
						color="primary"
						variant="contained"
						title="Exchange"
						hidden={!onExchange}
						disabled={disabled}
						onClick={() => onExchange!(token)}
					>
						<CurrencyExchange sx={{ pr: 1 }} /> Buy
					</Button>
				)}
				<ExpandMore
					expand={expanded}
					onClick={handleExpandClick}
					aria-expanded={expanded}
					aria-label="show more"
				>
					<ExpandCircleDown />
				</ExpandMore>
			</CardActions>
		</Card>
	);
}
