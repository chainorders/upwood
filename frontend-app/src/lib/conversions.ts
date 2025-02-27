export const formatDate = (date: Date) => {
	return date.toISOString().slice(0, 19);
};

import {
	BaseAccountTransactionSummary,
	BlockItemSummaryInBlock,
	ContractAddress,
	RejectedInit,
	RejectedReceive,
	RejectReasonTag,
	TransactionKindString,
	TransactionSummaryType,
	UpdateContractSummary,
} from "@concordium/web-sdk";

export function parseContractAddress(outcome: BlockItemSummaryInBlock): ContractAddress.Type {
	switch (outcome.summary.type) {
		case TransactionSummaryType.AccountTransaction:
			switch (outcome.summary.transactionType) {
				case TransactionKindString.InitContract:
					return outcome.summary.contractInitialized.address;
				case TransactionKindString.Failed: {
					switch (outcome.summary.rejectReason.tag) {
						case RejectReasonTag.RejectedInit: {
							throw new Error(`Rejected init: ${outcome.summary.rejectReason.rejectReason}`);
						}
						default:
							throw new Error(`Unknown reject reason ${outcome.summary.rejectReason.tag}`);
					}
				}
				default:
					throw new Error(`Unknown account transaction type: ${outcome.summary.transactionType}`);
			}
		default:
			throw new Error(`Unknown transaction type: ${outcome.summary.type}`);
	}
}

export function contractAddToString(contractAddress: ContractAddress.Type): string {
	return `${contractAddress.index.toString()}/${contractAddress.subindex.toString()}`;
}

export type ParsedError<TErr> = {
	message: string;
	error?: TErr;
};
export type ParsedFinalizedInitSuccess = {
	tag: "success";
	value: ContractAddress.Type;
};
export type ParsedFinalizedContractInitTxnError = {
	tag: "error";
	value: RejectedInit;
};

export const parseFinalizedInit: (
	txnSummary: BlockItemSummaryInBlock,
) => ParsedFinalizedInitSuccess | ParsedFinalizedContractInitTxnError = (txnSummary) => {
	switch (txnSummary.summary.type) {
		case TransactionSummaryType.AccountTransaction: {
			switch (txnSummary.summary.transactionType) {
				case TransactionKindString.InitContract: {
					return {
						tag: "success",
						value: txnSummary.summary.contractInitialized.address,
					};
				}
				case TransactionKindString.Failed: {
					switch (txnSummary.summary.rejectReason.tag) {
						case RejectReasonTag.RejectedInit: {
							return { tag: "error", value: txnSummary.summary.rejectReason };
						}
						default:
							throw new Error(`Unknown reject reason ${txnSummary.summary.rejectReason.tag}`);
					}
				}
				default:
					throw new Error(`"Unknown account transaction type: ${txnSummary.summary.transactionType}`);
			}
			break;
		}
		default:
			throw new Error(`Unknown transaction type: ${txnSummary.summary.type}`);
	}
};

export const toTokenId = (integer: number | bigint, tokenIdByteSize: number) => {
	if (tokenIdByteSize === 0) {
		return "";
	}

	const tokenIdHex = integer.toString(16).padStart(tokenIdByteSize * 2, "0");
	let tokenIdHexReversed = "";
	for (let i = tokenIdHex.length - 2; i >= 0; i -= 2) {
		tokenIdHexReversed += tokenIdHex.slice(i, i + 2);
	}
	return tokenIdHexReversed;
};

type ParsedFinalizedUpdateSuccess = {
	tag: "success";
	value: BaseAccountTransactionSummary & UpdateContractSummary;
};
type ParsedFinalizedContractUpdateTxnError = {
	tag: "error";
	value: RejectedReceive;
};

export const parseFinalizedUpdate: (
	txnSummary: BlockItemSummaryInBlock,
) => ParsedFinalizedUpdateSuccess | ParsedFinalizedContractUpdateTxnError = (txnSummary) => {
	switch (txnSummary.summary.type) {
		case TransactionSummaryType.AccountTransaction: {
			switch (txnSummary.summary.transactionType) {
				case TransactionKindString.Update: {
					return { tag: "success", value: txnSummary.summary };
				}
				case TransactionKindString.Failed: {
					switch (txnSummary.summary.rejectReason.tag) {
						case RejectReasonTag.RejectedReceive: {
							return { tag: "error", value: txnSummary.summary.rejectReason };
						}
						default:
							console.error(txnSummary.summary);
							throw new Error("Unknown reject reason");
					}
				}
				default: {
					throw new Error("Unknown account transaction type");
				}
			}
			break;
		}
		default:
			throw new Error("Unknown transaction type");
	}
};

export function toDisplayAmount(amount: string, decimals: number, roundToDecimal = 2) {
	const amountNum = BigInt(amount);
	const amountStr = amountNum.toString().padStart(decimals + 1, "0");
	const amountStrLen = amountStr.length;
	const integerPart = amountStr.slice(0, amountStrLen - decimals);
	const decimalPart = amountStr.slice(amountStrLen - decimals);
	const displayAmount = `${integerPart}.${decimalPart}`;
	return parseFloat(displayAmount).toFixed(roundToDecimal);
}

export function daysSince(date: Date | string) {
	const dateObj = new Date(date);
	const now = new Date();
	const diff = now.getTime() - dateObj.getTime();
	return Math.floor(diff / (1000 * 60 * 60 * 24));
}

export function toDisplayRate(
	numerator: string,
	denominator: string,
	tokenDecimals: number,
	currencyDecimals: number,
	roundToDecimal = 2,
) {
	const numeratorNum = parseFloat(numerator);
	const numeratorNumWithDecimals = numeratorNum / Math.pow(10, currencyDecimals);
	const denominatorNum = parseFloat(denominator);
	const denominatorNumWithDecimals = denominatorNum / Math.pow(10, tokenDecimals);
	const rate = numeratorNumWithDecimals / denominatorNumWithDecimals;
	return rate.toFixed(roundToDecimal);
}

export type Signature = {
	signature: string;
	signatureScheme: string;
};

export type SigsApi = {
	sigs: {
		[key: number]: {
			sigs: {
				[key: number]: Signature;
			};
		};
	};
};

export type SigsContract = [number, [number, { Ed25519: [string] }][]][];

export function sigsApiToContract(sigsApi: SigsApi): SigsContract {
	console.log(sigsApi);
	const sigs: SigsContract = [];
	for (const [key, value] of Object.entries(sigsApi.sigs)) {
		const keyNum = parseInt(key);
		const sigsArray: [number, { Ed25519: [string] }][] = [];
		for (const [key2, value2] of Object.entries(value.sigs)) {
			const key2Num = parseInt(key2);
			const sig = value2.signature;
			const sigScheme = value2.signatureScheme;
			sigsArray.push([key2Num, { Ed25519: [sig] }]);
		}
		sigs.push([keyNum, sigsArray]);
	}
	return sigs;
}
