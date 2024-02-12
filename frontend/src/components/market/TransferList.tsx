import {
	CIS2,
	CIS2Contract,
	ContractAddress,
	Energy,
	EntrypointName,
	serializeTypeValue,
	toBuffer,
} from "@concordium/web-sdk";
import { Buffer } from "buffer/";
import rwaMarket, { ListRequest } from "../../lib/rwaMarket";
import { useNodeClient } from "../NodeClientProvider";
import { useWallet } from "../WalletProvider";
import ListRequestForm, { NonListedToken } from "./ListRequest";
import { useParams } from "react-router-dom";

type Props = {
	contract: ContractAddress.Type;
};
export default function TransferList(props: Props) {
	const { contract } = props;
	const { currentAccount, provider: walletApi } = useWallet();
	const { provider: grpcClient } = useNodeClient();
	const { listContractIndex, listContractSubIndex, listTokenId, listAmount } =
		useParams();

	const sendTransaction = async (request: ListRequest) => {
		const listRequestSerialized = serializeTypeValue(
			request,
			toBuffer(rwaMarket.list.paramsSchemaBase64!, "base64"),
		);
		const cis2CLient = await CIS2Contract.create(
			grpcClient,
			ContractAddress.create(
				request.token_id.contract.index,
				request.token_id.contract.subindex,
			),
		);
		const transfer = cis2CLient.createTransfer(
			{
				energy: Energy.create(
					rwaMarket.list.maxExecutionEnergy.value * BigInt(2),
				),
			},
			{
				from: currentAccount!,
				to: {
					address: contract,
					hookName: EntrypointName.fromString("deposit"),
				},
				amount: BigInt(0),
				tokenId: request.token_id.id,
				tokenAmount: BigInt(request.supply),
				data: Buffer.from(listRequestSerialized.buffer).toString("hex"),
			} as CIS2.Transfer,
		);
		return walletApi!.sendTransaction(
			currentAccount!,
			transfer.type,
			transfer.payload,
			transfer.parameter.json,
			transfer.schema,
		);
	};

	const nonListed: NonListedToken | undefined = (listContractIndex &&
		listContractSubIndex &&
		listTokenId &&
		listAmount &&
		({
			id: listTokenId,
			contract: ContractAddress.create(
				BigInt(listContractIndex),
				BigInt(listContractSubIndex),
			),
			amount: Number(listAmount),
		} as NonListedToken)) as NonListedToken | undefined;
	return (
		<ListRequestForm
			contract={contract}
			currentAccount={currentAccount!}
			onSendTransaction={(req) => sendTransaction(req)}
			nonListed={nonListed}
		/>
	);
}
