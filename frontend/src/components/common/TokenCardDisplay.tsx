import {
	CIS2,
	CIS2Contract,
	ConcordiumGRPCClient,
	ContractAddress,
} from "@concordium/web-sdk";
import { ExpandCircleDown } from "@mui/icons-material";
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
import { red } from "@mui/material/colors";
import { useEffect, useState } from "react";
import {
	TokenMetadata,
	getTokenMetadata,
	toDataUrl,
} from "../../lib/cis2Utils";
import SendTransactionButton from "./SendTransactionButton";

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

export type Token = {
	id: string;
	contract: ContractAddress.Type;
	amount: string;
};

export type ActionButtonProps = {
	token: Token;
	ariaLabel: string;
	variant: "text" | "outlined" | "contained";
	title: string;
	disabled: boolean;
	onClick: (token: Token) => void;
	children: React.ReactNode;
	key?: string;
	sendTransaction?: (token: Token) => Promise<string>;
};
export function ActionButton(props: ActionButtonProps) {
	if (props.sendTransaction) {
		return (
			<SendTransactionButton
				smallSize
				key={props.key}
				onClick={() => props.sendTransaction!(props.token)}
				onDone={() => props.onClick(props.token)}
			>
				{props.children}
			</SendTransactionButton>
		);
	}

	return (
		<Button
			key={props.key}
			sx={{ mr: 1 }}
			aria-label={props.ariaLabel}
			variant={props.variant}
			title={props.title}
			disabled={props.disabled}
			onClick={() => props.onClick(props.token)}
		>
			{props.children}
		</Button>
	);
}

type Props = {
	grpcClient: ConcordiumGRPCClient;
	token: Token;
	actions: ActionButtonProps[];
};
export default function TokenCardDisplay(props: Props) {
	const { grpcClient, token } = props;
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
		CIS2Contract.create(grpcClient, token.contract)
			.then((client) => client.tokenMetadata(token.id))
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
		getTokenMetadata(metadataUrl.url)
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
				title={`Token Id: ${token.id}`}
				subheader={`${token.contract.index}/${token.contract.subindex}`}
				avatar={
					<Avatar sx={{ bgcolor: red[500] }} aria-label="token">
						T
					</Avatar>
				}
			/>
			{imageUrl && (
				<CardMedia
					component="img"
					alt={`token: ${token.id} Image`}
					image={imageUrl}
				/>
			)}
			{loadingMetadataUrl || loadingMetadata ? (
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
				{props.actions.map((action, index) =>
					ActionButton({ ...action, key: index.toString() }),
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
