import {
	AccountAddress,
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
import ListRequestForm, { NonListedToken } from "./ListRequest";
import { useParams } from "react-router-dom";
import { WalletApi } from "@concordium/browser-wallet-api-helpers";
import { permit } from "../../lib/sponsorUtils";
import { useSponsorApi } from "../SponsorApiProvider";

type Props = {
	wallet: WalletApi;
	currentAccount: AccountAddress.Type;
	contract: ContractAddress.Type;
	sponsorContract?: ContractAddress.Type;
};
export default function TransferList(props: Props) {
	const { contract, wallet, currentAccount } = props;
	const { provider: grpcClient } = useNodeClient();
	const { listContractIndex, listContractSubIndex, listTokenId, listAmount } =
		useParams();
	const { provider: sponsorApi } = useSponsorApi();

	const sendTransaction = async (
		request: ListRequest,
		sponsorContract?: ContractAddress.Type,
	) => {
		const listRequestSerialized = serializeTypeValue(
			request,
			toBuffer(rwaMarket.list.paramsSchemaBase64!, "base64"),
		);
		const tokenContract = ContractAddress.create(
			request.token_id.contract.index,
			request.token_id.contract.subindex,
		);
		const cis2CLient = await CIS2Contract.create(grpcClient, tokenContract);
		const transfer = cis2CLient.createTransfer(
			{
				energy: Energy.create(
					rwaMarket.list.maxExecutionEnergy.value * BigInt(2),
				),
			},
			{
				from: currentAccount,
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

		if (!sponsorContract) {
			return wallet.sendTransaction(
				currentAccount,
				transfer.type,
				transfer.payload,
				transfer.parameter.json,
				transfer.schema,
			);
		} else {
			return permit(
				grpcClient,
				wallet,
				sponsorApi,
				sponsorContract!,
				currentAccount,
				tokenContract,
				transfer.parameter.hex,
			);
		}
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
			currentAccount={props.currentAccount}
			onSendTransaction={(req, sponsorContract) =>
				sendTransaction(req, sponsorContract)
			}
			nonListed={nonListed}
			sponsorContract={props.sponsorContract}
		/>
	);
}
