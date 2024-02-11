import {
	IconButton,
	List,
	ListItem,
	ListItemButton,
	ListItemIcon,
	ListItemText,
	Paper,
	Stack,
	Typography,
} from "@mui/material";
import { Contract, ContractType } from "./ContractTypes";
import {
	AccountCircle,
	CurrencyExchange,
	Delete,
	Handshake,
	OpenInBrowser,
	Token,
	ViewModule,
} from "@mui/icons-material";
import { Link } from "react-router-dom";

interface Props {
	contracts: Contract[];
	onDelete: (contract: Contract) => void;
}

function ContractLink(props: { contract: Contract }) {
	const contractAddressString = `${props.contract.address.index.toString()}/${props.contract.address.subindex.toString()}`;
	return (
		<Link to={`${props.contract.type}/${contractAddressString}`}>
			{props.contract.name} ({contractAddressString})
		</Link>
	);
}

export default function ContractsList(props: Props) {
	const identityRegistries = props.contracts.filter(
		(c) => c.type == ContractType.RwaIdentityRegistry,
	);
	const complianceModules = props.contracts.filter(
		(c) => c.type == ContractType.RwaComplianceModule,
	);
	const complianceContracts = props.contracts.filter(
		(c) => c.type == ContractType.RwaCompliance,
	);
	const marketContracts = props.contracts.filter(
		(c) => c.type == ContractType.RwaMarket,
	);
	const nftContracts = props.contracts.filter(
		(c) => c.type == ContractType.RwaSecurityNft,
	);
	const sftContracts = props.contracts.filter(
		(c) => c.type == ContractType.RwaSecuritySft,
	);

	return (
		<Stack spacing={1}>
			<Paper sx={{ padding: 2 }} variant="outlined">
				<Typography variant="h2" fontSize={20}>
					Identity Registries
				</Typography>
				<List>
					{identityRegistries.map((contract) => {
						return (
							<ListItem
								disablePadding
								key={contract.address.index.toString()}
								secondaryAction={
									<IconButton
										edge="end"
										aria-label="delete"
										onClick={() => props.onDelete(contract)}
									>
										<Delete />
									</IconButton>
								}
							>
								<ListItemButton>
									<ListItemIcon>
										<AccountCircle />
									</ListItemIcon>
									<ListItemText
										primary={<ContractLink contract={contract} />}
									/>
								</ListItemButton>
							</ListItem>
						);
					})}
				</List>
			</Paper>
			<Paper sx={{ padding: 2 }} variant="outlined">
				<Typography variant="h2" fontSize={20}>
					Compliance Modules
				</Typography>
				<List>
					{complianceModules.map((contract) => {
						return (
							<ListItem
								disablePadding
								key={contract.address.index.toString()}
								secondaryAction={
									<IconButton
										edge="end"
										aria-label="delete"
										onClick={() => props.onDelete(contract)}
									>
										<Delete />
									</IconButton>
								}
							>
								<ListItemButton>
									<ListItemIcon>
										<ViewModule />
									</ListItemIcon>
									<ListItemText
										primary={<ContractLink contract={contract} />}
									/>
								</ListItemButton>
							</ListItem>
						);
					})}
				</List>
			</Paper>
			<Paper sx={{ padding: 2 }} variant="outlined">
				<Typography variant="h2" fontSize={20}>
					Compliance Contracts
				</Typography>
				<List>
					{complianceContracts.map((contract) => {
						return (
							<ListItem
								disablePadding
								key={contract.address.index.toString()}
								secondaryAction={
									<IconButton
										edge="end"
										aria-label="delete"
										onClick={() => props.onDelete(contract)}
									>
										<Delete />
									</IconButton>
								}
							>
								<ListItemButton>
									<ListItemIcon>
										<Handshake />
									</ListItemIcon>
									<ListItemText
										primary={<ContractLink contract={contract} />}
									/>
								</ListItemButton>
							</ListItem>
						);
					})}
				</List>
			</Paper>
			<Paper sx={{ padding: 2 }} variant="outlined">
				<Typography variant="h2" fontSize={20}>
					Security NFT Contracts
				</Typography>
				<List>
					{nftContracts.map((contract) => {
						return (
							<ListItem
								disablePadding
								key={contract.address.index.toString()}
								secondaryAction={
									<>
										<Link
											to={`/nft/${contract.address.index.toString()}/${contract.address.subindex.toString()}`}
											target="_blank"
										>
											<IconButton edge="end" aria-label="open nft contract">
												<OpenInBrowser />
											</IconButton>
										</Link>
										<IconButton
											edge="end"
											aria-label="delete"
											onClick={() => props.onDelete(contract)}
										>
											<Delete />
										</IconButton>
									</>
								}
							>
								<ListItemButton>
									<ListItemIcon>
										<Token />
									</ListItemIcon>
									<ListItemText
										primary={<ContractLink contract={contract} />}
									/>
								</ListItemButton>
							</ListItem>
						);
					})}
				</List>
			</Paper>
			<Paper sx={{ padding: 2 }} variant="outlined">
				<Typography variant="h2" fontSize={20}>
					Security SFT Contracts
				</Typography>
				<List>
					{sftContracts.map((contract) => {
						return (
							<ListItem
								disablePadding
								key={contract.address.index.toString()}
								secondaryAction={
									<>
										<Link
											to={`/sft/${contract.address.index.toString()}/${contract.address.subindex.toString()}`}
											target="_blank"
										>
											<IconButton edge="end" aria-label="open sft contract">
												<OpenInBrowser />
											</IconButton>
										</Link>

										<IconButton
											edge="end"
											aria-label="delete"
											onClick={() => props.onDelete(contract)}
										>
											<Delete />
										</IconButton>
									</>
								}
							>
								<ListItemButton>
									<ListItemIcon>
										<Token />
									</ListItemIcon>
									<ListItemText
										primary={<ContractLink contract={contract} />}
									/>
								</ListItemButton>
							</ListItem>
						);
					})}
				</List>
			</Paper>
			<Paper sx={{ padding: 2 }} variant="outlined">
				<Typography variant="h2" fontSize={20}>
					Market Contracts
				</Typography>
				<List>
					{marketContracts.map((contract) => {
						return (
							<ListItem
								disablePadding
								key={contract.address.index.toString()}
								secondaryAction={
									<>
										<Link
											to={`/market/${contract.address.index.toString()}/${contract.address.subindex.toString()}`}
											target="_blank"
										>
											<IconButton edge="end" aria-label="open market">
												<OpenInBrowser />
											</IconButton>
										</Link>

										<IconButton
											edge="end"
											aria-label="delete"
											onClick={() => props.onDelete(contract)}
										>
											<Delete />
										</IconButton>
									</>
								}
							>
								<ListItemButton>
									<ListItemIcon>
										<CurrencyExchange />
									</ListItemIcon>
									<ListItemText
										primary={<ContractLink contract={contract} />}
									/>
								</ListItemButton>
							</ListItem>
						);
					})}
				</List>
			</Paper>
		</Stack>
	);
}
