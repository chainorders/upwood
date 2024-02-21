import {
	IconButton,
	List,
	ListItem,
	ListItemButton,
	ListItemIcon,
	ListItemText,
	ListSubheader,
	Typography,
} from "@mui/material";
import { Contract, ContractType } from "./ContractTypes";
import {
	AccountCircle,
	AddBoxOutlined,
	CurrencyExchange,
	Delete,
	Handshake,
	OpenInBrowser,
	Token,
	ViewModule,
} from "@mui/icons-material";
import { Link } from "react-router-dom";
import { ReactNode } from "react";

function ContractLink(props: { contract: Contract }) {
	const contractAddressString = `${props.contract.address.index.toString()}/${props.contract.address.subindex.toString()}`;
	return (
		<Link to={`${props.contract.type}/${contractAddressString}`} target="_self">
			{props.contract.name} ({contractAddressString})
		</Link>
	);
}

const InitListItem = (props: { onClick: () => void; children: ReactNode }) => {
	if (!props.onClick) return <></>;

	<ListItem
		secondaryAction={
			<IconButton edge="end" aria-label="add" onClick={() => props.onClick()}>
				<AddBoxOutlined />
			</IconButton>
		}
	>
		<ListItemText
			primary={<Typography align="right">{props.children}</Typography>}
		/>
	</ListItem>;
};

interface Props {
	contracts: Contract[];
	onDelete?: (contract: Contract) => void;
	onInit?: (type: ContractType) => void;
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
		<List>
			<ListSubheader>Identity Registries</ListSubheader>
			{identityRegistries.map((contract) => {
				return (
					<ListItem
						key={contract.address.index.toString()}
						secondaryAction={
							props.onDelete && (
								<IconButton
									edge="end"
									aria-label="delete"
									onClick={() => props.onDelete!(contract)}
								>
									<Delete />
								</IconButton>
							)
						}
					>
						<ListItemButton>
							<ListItemIcon>
								<AccountCircle />
							</ListItemIcon>
							<ListItemText primary={<ContractLink contract={contract} />} />
						</ListItemButton>
					</ListItem>
				);
			})}
			{props.onInit && (
				<InitListItem
					onClick={() => props.onInit!(ContractType.RwaIdentityRegistry)}
				>
					Initialize new Identity Registry
				</InitListItem>
			)}
			<ListSubheader>Compliance Modules</ListSubheader>
			{complianceModules.map((contract) => {
				return (
					<ListItem
						key={contract.address.index.toString()}
						secondaryAction={
							props.onDelete && (
								<IconButton
									edge="end"
									aria-label="delete"
									onClick={() => props.onDelete!(contract)}
								>
									<Delete />
								</IconButton>
							)
						}
					>
						<ListItemButton>
							<ListItemIcon>
								<ViewModule />
							</ListItemIcon>
							<ListItemText primary={<ContractLink contract={contract} />} />
						</ListItemButton>
					</ListItem>
				);
			})}
			{props.onInit && (
				<InitListItem
					onClick={() => props.onInit!(ContractType.RwaComplianceModule)}
				>
					Initialize new Compliance Module
				</InitListItem>
			)}
			<ListSubheader>Compliance Contracts</ListSubheader>
			{complianceContracts.map((contract) => {
				return (
					<ListItem
						key={contract.address.index.toString()}
						secondaryAction={
							props.onDelete && (
								<IconButton
									edge="end"
									aria-label="delete"
									onClick={() => props.onDelete!(contract)}
								>
									<Delete />
								</IconButton>
							)
						}
					>
						<ListItemButton>
							<ListItemIcon>
								<Handshake />
							</ListItemIcon>
							<ListItemText primary={<ContractLink contract={contract} />} />
						</ListItemButton>
					</ListItem>
				);
			})}
			{props.onInit && (
				<InitListItem onClick={() => props.onInit!(ContractType.RwaCompliance)}>
					Initialize new Compliance Contract
				</InitListItem>
			)}
			<ListSubheader>NFT Contracts</ListSubheader>
			{nftContracts.map((contract) => {
				return (
					<ListItem
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
								{props.onDelete && (
									<IconButton
										edge="end"
										aria-label="delete"
										onClick={() => props.onDelete!(contract)}
									>
										<Delete />
									</IconButton>
								)}
							</>
						}
					>
						<ListItemButton>
							<ListItemIcon>
								<Token />
							</ListItemIcon>
							<ListItemText primary={<ContractLink contract={contract} />} />
						</ListItemButton>
					</ListItem>
				);
			})}
			{props.onInit && (
				<InitListItem
					onClick={() => props.onInit!(ContractType.RwaSecurityNft)}
				>
					Initialize new NFT Contract
				</InitListItem>
			)}
			<ListSubheader>Fractionalizer Contracts</ListSubheader>
			{sftContracts.map((contract) => {
				return (
					<ListItem
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

								{props.onDelete && (
									<IconButton
										edge="end"
										aria-label="delete"
										onClick={() => props.onDelete!(contract)}
									>
										<Delete />
									</IconButton>
								)}
							</>
						}
					>
						<ListItemButton>
							<ListItemIcon>
								<Token />
							</ListItemIcon>
							<ListItemText primary={<ContractLink contract={contract} />} />
						</ListItemButton>
					</ListItem>
				);
			})}
			{props.onInit && (
				<InitListItem
					onClick={() => props.onInit!(ContractType.RwaSecuritySft)}
				>
					Initialize new SFT Contract
				</InitListItem>
			)}
			<ListSubheader>Market Contracts</ListSubheader>
			{marketContracts.map((contract) => {
				return (
					<ListItem
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

								{props.onDelete && (
									<IconButton
										edge="end"
										aria-label="delete"
										onClick={() => props.onDelete!(contract)}
									>
										<Delete />
									</IconButton>
								)}
							</>
						}
					>
						<ListItemButton>
							<ListItemIcon>
								<CurrencyExchange />
							</ListItemIcon>
							<ListItemText primary={<ContractLink contract={contract} />} />
						</ListItemButton>
					</ListItem>
				);
			})}
			{props.onInit && (
				<InitListItem onClick={() => props.onInit!(ContractType.RwaMarket)}>
					Initialize new Market
				</InitListItem>
			)}
		</List>
	);
}
