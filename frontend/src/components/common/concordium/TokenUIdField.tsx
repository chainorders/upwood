import { CIS2, ContractAddress } from "@concordium/web-sdk";
import { MenuItem, Select, Stack, Typography } from "@mui/material";
import { useEffect, useState } from "react";
import TokenIdField from "./TokenIdField";
import ContractAddressField from "./ContractAddressField";

export type TokenUId = {
	id: CIS2.TokenId;
	contract: ContractAddress.Type;
};

type Props = {
	value?: TokenUId;
	onChange: (value?: TokenUId) => void;
	helperText?: string;
	name?: string;
};

export default function TokenUIdField(props: Props) {
	const [tokenIdSize, setTokenIdSize] = useState(0);
	const [tokenId, setTokenId] = useState<CIS2.TokenId>("");
	const [contract, setContract] = useState<ContractAddress.Type | undefined>(
		undefined,
	);
	const { onChange } = props;
	const isValid =
		(tokenIdSize === 0 && tokenId.length === 0 && contract !== undefined) ||
		(tokenIdSize !== 0 && tokenId.length !== 0 && contract !== undefined);

	useEffect(() => {
		if (isValid) {
			onChange({
				id: tokenId!,
				contract: contract!,
			});
		} else {
			onChange(undefined);
		}
	}, [tokenIdSize, tokenId, contract, isValid, onChange]);

	return (
		<Stack spacing={1}>
			<Typography variant="caption">Token Id Size</Typography>
			<Select
				onChange={(e) => {
					setTokenIdSize(Number(e.target.value));
					setTokenId("");
				}}
				value={tokenIdSize}
				fullWidth
			>
				<MenuItem value={0}>Unit</MenuItem>
				<MenuItem value={1}>U8</MenuItem>
				<MenuItem value={2}>U16</MenuItem>
				<MenuItem value={3}>U32</MenuItem>
				<MenuItem value={4}>U64</MenuItem>
			</Select>
			<Typography variant="caption">Token Id</Typography>
			<TokenIdField
				sizeByte={tokenIdSize}
				onChange={setTokenId}
				name="Token Id"
				id="tokenId"
				disabled={tokenIdSize === 0}
				value={tokenId}
				fullWidth
			/>
			<Typography variant="caption">Token Contract</Typography>
			<ContractAddressField
				onChange={setContract}
				indexName="Token Contract Index"
				subIndexName="Token Contract Sub Index"
				indexHelperText="The index of the token contract"
				subIndexHelperText="The subindex of the token contract"
			/>
		</Stack>
	);
}
