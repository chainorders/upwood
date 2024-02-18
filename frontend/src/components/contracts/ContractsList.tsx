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
import { Link, useNavigate } from "react-router-dom";

interface Props {
	contracts: Contract[];
	onDelete: (contract: Contract) => void;
}

function ContractLink(props: { contract: Contract }) {
	const contractAddressString = `${props.contract.address.index.toString()}/${props.contract.address.subindex.toString()}`;
	return (
		<Link to={`${props.contract.type}/${contractAddressString}`} target="_self">
			{props.contract.name} ({contractAddressString})
		</Link>
	);
}

export default function ContractsList(props: Props) {
	const navigate = useNavigate();

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
							<ListItemText primary={<ContractLink contract={contract} />} />
						</ListItemButton>
					</ListItem>
				);
			})}
			<ListItem
				secondaryAction={
					<IconButton
						edge="end"
						aria-label="add"
						onClick={() => navigate(ContractType.RwaIdentityRegistry + "/init")}
					>
						<AddBoxOutlined />
					</IconButton>
				}
			>
				<ListItemText
					primary={
						<Typography align="right">
							Initialize new Identity Registry
						</Typography>
					}
				/>
			</ListItem>
			<ListSubheader>Compliance Modules</ListSubheader>
			{complianceModules.map((contract) => {
				return (
					<ListItem
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
							<ListItemText primary={<ContractLink contract={contract} />} />
						</ListItemButton>
					</ListItem>
				);
			})}
			<ListItem
				secondaryAction={
					<IconButton
						edge="end"
						aria-label="add"
						onClick={() => navigate(ContractType.RwaComplianceModule + "/init")}
					>
						<AddBoxOutlined />
					</IconButton>
				}
			>
				<ListItemText
					primary={
						<Typography align="right">
							Initialize new Compliance Module
						</Typography>
					}
				/>
			</ListItem>
			<ListSubheader>Compliance Contracts</ListSubheader>
			{complianceContracts.map((contract) => {
				return (
					<ListItem
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
							<ListItemText primary={<ContractLink contract={contract} />} />
						</ListItemButton>
					</ListItem>
				);
			})}
			<ListItem
				secondaryAction={
					<IconButton
						edge="end"
						aria-label="add"
						onClick={() => navigate(ContractType.RwaCompliance + "/init")}
					>
						<AddBoxOutlined />
					</IconButton>
				}
			>
				<ListItemText
					primary={
						<Typography align="right">
							Initialize new Compliance Contract
						</Typography>
					}
				/>
			</ListItem>
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
							<ListItemText primary={<ContractLink contract={contract} />} />
						</ListItemButton>
					</ListItem>
				);
			})}
			<ListItem
				secondaryAction={
					<IconButton
						edge="end"
						aria-label="add"
						onClick={() => navigate(ContractType.RwaSecurityNft + "/init")}
					>
						<AddBoxOutlined />
					</IconButton>
				}
			>
				<ListItemText
					primary={
						<Typography align="right">Initialize new Nft Contract</Typography>
					}
				/>
			</ListItem>
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
							<ListItemText primary={<ContractLink contract={contract} />} />
						</ListItemButton>
					</ListItem>
				);
			})}
			<ListItem
				secondaryAction={
					<IconButton
						edge="end"
						aria-label="add"
						onClick={() => navigate(ContractType.RwaSecuritySft + "/init")}
					>
						<AddBoxOutlined />
					</IconButton>
				}
			>
				<ListItemText
					primary={
						<Typography align="right">
							Initialize new Fractionalizer Contract
						</Typography>
					}
				/>
			</ListItem>
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
							<ListItemText primary={<ContractLink contract={contract} />} />
						</ListItemButton>
					</ListItem>
				);
			})}
			<ListItem
				secondaryAction={
					<IconButton
						edge="end"
						aria-label="add"
						onClick={() => navigate(ContractType.RwaMarket + "/init")}
					>
						<AddBoxOutlined />
					</IconButton>
				}
			>
				<ListItemText
					primary={
						<Typography align="right">
							Initialize new Market Contract
						</Typography>
					}
				/>
			</ListItem>
		</List>
	);
}
