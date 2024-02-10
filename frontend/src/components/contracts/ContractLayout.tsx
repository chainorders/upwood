import { Grid, Paper, Stack, Typography } from "@mui/material";
import { Contract } from "./ContractTypes";
import { Outlet, useParams } from "react-router-dom";
import { useEffect, useState } from "react";
import concordiumNodeClient from "../../lib/ConcordiumNodeClient";
import { ContractAddress, InstanceInfo } from "@concordium/web-sdk";
import CCDScanModuleLink from "../common/concordium/CCDScanModuleLink";
import CCDScanContractLink from "../common/concordium/CCDScanContractLink";
import { capitalCase } from "change-case";
import CCDScanAccountLink from "../common/concordium/CCDScanAccountLink";

export default function ContractLayout(props: { contracts: Contract[] }) {
	const { index, subIndex } = useParams();
	const contract = props.contracts.find((contract) => {
		return (
			contract.address.index.toString() === index &&
			contract.address.subindex.toString() === subIndex
		);
	});

	const [onChainInfo, setOncChainInfo] = useState<InstanceInfo>();
	useEffect(() => {
		if (onChainInfo) {
			return;
		}

		concordiumNodeClient
			.getInstanceInfo(
				ContractAddress.create(BigInt(index!), BigInt(subIndex!)),
			)
			.then((info) => {
				setOncChainInfo(info);
			});
	});

	if (!contract) {
		return (
			<div>
				<h1>
					{index}/{subIndex}
					<Typography variant="body1" color="error">
						Contract not found
					</Typography>
				</h1>
			</div>
		);
	}

	return (
		<Stack spacing={2}>
			<Stack>
				<Paper variant="outlined">
					<Typography variant="h2" fontSize={24}>
						{capitalCase(contract.type)}
					</Typography>
					<Typography variant="h3" fontSize={20}>
						<CCDScanContractLink
							text={contract.name}
							index={contract.address.index.toString()}
							subIndex={contract.address.subindex.toString()}
						/>
					</Typography>
					<Typography variant="h4" fontSize={18}></Typography>
				</Paper>
			</Stack>
			<Grid container spacing={1}>
				<Grid item xs={12} md={9}>
					<Outlet context={contract} />
				</Grid>
				<Grid item xs={0} md={3}>
					<Grid container spacing={2}>
						<Grid item xs={12} md={12}>
							<Paper>
								<Typography variant="h4" fontSize={18}>
									On Chain Information
								</Typography>
								{onChainInfo && (
									<>
										<Typography>
											Amount : {onChainInfo?.amount.toJSON()}
										</Typography>
										<Typography>
											Module Ref :{" "}
											<CCDScanModuleLink
												moduleRef={onChainInfo?.sourceModule.moduleRef}
											/>
										</Typography>
										<Typography>
											Name :{" "}
											<CCDScanContractLink
												text={onChainInfo?.name.value}
												index={contract.address.index.toString()}
												subIndex={contract.address.subindex.toString()}
											/>
										</Typography>
										<Typography>
											Owner:{" "}
											<CCDScanAccountLink
												account={onChainInfo?.owner.address}
											/>
										</Typography>
									</>
								)}
							</Paper>
						</Grid>
						<Grid item xs={12} md={12}>
							<Paper>
								<Typography variant="h4" fontSize={18}>
									Module Actions
								</Typography>
								{onChainInfo && (
									<>
										{onChainInfo.methods.map((method) => {
											return (
												<Typography key={method.value} variant="body2">
													{method.value}
												</Typography>
											);
										})}
									</>
								)}
							</Paper>
						</Grid>
					</Grid>
				</Grid>
			</Grid>
		</Stack>
	);
}
