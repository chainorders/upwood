import {
	AccountAddress,
	ConcordiumGRPCClient,
	ContractAddress,
	serializeTypeValue,
	toBuffer,
} from "@concordium/web-sdk";
import rwaSponsor from "./rwaSponsor";
import moment from "moment";
import { Buffer } from "buffer/";
import { WalletApi } from "@concordium/browser-wallet-api-helpers";
import { ApiCredentialSignature, ApiKeySignature, SponsorApi } from "./sponsor-api-client";

export const permit = async (
	grpcClient: ConcordiumGRPCClient,
	walletApi: WalletApi,
	sponsorApi: SponsorApi,
	sponsorContract: ContractAddress.Type,
	currentAccount: AccountAddress.Type,
	sponsoredContract: ContractAddress.Type,
	sponsoredPayloadHex: string,
) => {
	const nonce: bigint = await rwaSponsor.nonce
		.invoke(
			grpcClient,
			sponsorContract,
			{ account: currentAccount.address },
			currentAccount,
		)
		.then((res) => rwaSponsor.nonce.parseReturnValue(res.returnValue!)!);
	const consensus = await grpcClient.getConsensusStatus();
	const block = await grpcClient.getBlockInfo(consensus.bestBlock);
	const expiry = moment(block.blockSlotTime).add(1, "minute");
	const messageToSign = {
		contract_address: {
			index: Number(sponsoredContract.index),
			subindex: Number(sponsoredContract.subindex),
		},
		entry_point: "transfer",
		nonce,
		timestamp: expiry.toISOString(),
		payload: Array.from(Buffer.from(sponsoredPayloadHex, "hex")),
	};
	const bytesToSign = Buffer.from(
		serializeTypeValue(
			messageToSign,
			toBuffer(rwaSponsor.bytesToSign.paramsSchemaBase64!, "base64"),
		).buffer,
	);
	const signature = await walletApi.signMessage(currentAccount.address, {
		data: bytesToSign.toString("hex"),
		schema: rwaSponsor.bytesToSign.paramsSchemaBase64!,
	});
	const apiRes = await sponsorApi.default.postSponsorPermit({
		requestBody: {
			signer: currentAccount.address,
			message: {
				contract_address: messageToSign.contract_address,
				nonce: Number(messageToSign.nonce),
				entry_point: messageToSign.entry_point,
				payload: messageToSign.payload,
				timestamp: expiry.valueOf(),
			},
			signature: {
				sigs: Object.entries(signature).map(
					([credentialIndex, sigs]) =>
						({
							credential_index: Number(credentialIndex),
							sigs: Object.entries(sigs).map(
								([keyIndex, signature]) =>
									({
										key_index: Number(keyIndex),
										signature,
									}) as ApiKeySignature,
							),
						}) as ApiCredentialSignature,
				),
			},
		},
	});

	return apiRes.txn_hash;
};
