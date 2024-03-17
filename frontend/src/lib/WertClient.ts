import { Buffer } from "buffer/";
import { v4 as uuidv4 } from "uuid";
import { ContractAddress } from "@concordium/web-sdk";
import WertWidget from "@wert-io/widget-initializer";
import { signSmartContractData } from "@wert-io/widget-sc-signer";

const WERT_NETWORK = import.meta.env.VITE_WERT_NETWORK!;
const WERT_ORIGIN = import.meta.env.VITE_WERT_ORIGIN!;
const WERT_PARTNER_ID = import.meta.env.VITE_WERT_PARTNER_ID!;
const WERT_PRIVATE_KEY = import.meta.env.VITE_WERT_PRIVATE_KEY!;

//@ts-expect-error : adding buffer property to window
window.Buffer = Buffer;

const toHex = (obj: Record<string, unknown>) => {
	return Buffer.from(JSON.stringify(obj)).toString("hex");
};

export function toParamContractAddress(contractAddress: ContractAddress.Type): {
	index: number;
	subindex: number;
} {
	return {
		index: parseInt(contractAddress.index.toString()),
		subindex: parseInt(contractAddress.subindex.toString()),
	};
}

export type ParamContractAddress = { index: number; subindex: number };

export const initWert = async (
	account: string,
	contractAddress: ContractAddress.Type,
	contractName: string,
	contractEntrypoint: string,
	contractParameter: Buffer,
	totalPaymentCcd: bigint,
	containerId = "widget",
	width = 400,
	height = 600,
): Promise<{ status: "closed" | "success"; tx_id?: string }> => {
	return new Promise((res, rej) => {
		if (!validWertConfig()) {
			rej(new Error("Wert config is not valid"));
			return;
		}

		const signedData = signSmartContractData(
			{
				address: account,
				commodity: "CCD",
				commodity_amount: parseInt(totalPaymentCcd.toString()), // typescript does accept this as a string
				sc_address: toHex({
					index: parseInt(contractAddress.index.toString()),
					subindex: parseInt(contractAddress.subindex.toString()),
				}),
				sc_input_data: toHex({
					entrypoint: `${contractName}.${contractEntrypoint}`,
					params: contractParameter.toString("hex"),
				}),
				network: WERT_NETWORK!,
			},
			WERT_PRIVATE_KEY!,
		);
		const otherWidgetOptions = {
			pk_id: "key1",
			sc_id: uuidv4(), //this is the request id. Needs to be diff for every request. just keeping click_id doesnt work.
			partner_id: WERT_PARTNER_ID,
			container_id: containerId,
			click_id: uuidv4(),
			origin: WERT_ORIGIN,
			width,
			height,
			listeners: {
				close: () => {
					console.log("close");
					res({ status: "closed" });
				},
				error: (err: { name: string; message: string }) => {
					console.error("error", err);
					rej(err);
				},
				loaded: () => {
					console.log("loaded");
				},
				"payment-status": (status: {
					status: string;
					payment_id: string;
					order_id: string;
					tx_id: string;
				}) => {
					console.log("payment-status", status);
					if (status.status === "success" && status.tx_id) {
						res({ status: "success", tx_id: status.tx_id });
					}
				},
				position: (pos: { step: string }) => {
					console.log("position", pos);
				},
			},
		};
		const wertWidget = new WertWidget({
			...signedData,
			...otherWidgetOptions,
		});

		wertWidget.open();
	});
};

function validWertConfig() {
	console.log(WERT_PARTNER_ID, WERT_PRIVATE_KEY, WERT_NETWORK, WERT_ORIGIN);
	return WERT_PARTNER_ID && WERT_PRIVATE_KEY && WERT_NETWORK && WERT_ORIGIN;
}
