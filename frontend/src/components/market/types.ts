import { ContractAddress } from "@concordium/web-sdk";
import { ListRequest } from "../../lib/rwaMarket";

export type Flatten<T> = T extends unknown[] ? T[number] : T;
export type ContractExchangeRates = ListRequest["exchange_rates"];
export type ContractExchangeRate = Flatten<ContractExchangeRates>;
export const toContractRate = (rate: Rate): { numerator: bigint; denominator: bigint } => {
	return {
		numerator: BigInt(rate.numerator),
		denominator: BigInt(rate.denominator),
	};
};
export const fromContractRate = (rate: { numerator: bigint | number; denominator: bigint | number }): Rate => {
	return {
		numerator: BigInt(rate.numerator),
		denominator: BigInt(rate.denominator),
	};
};

export type Rate = { numerator: bigint; denominator: bigint };
export type Cis2PaymentToken = { type: "Cis2"; id: string; contract: ContractAddress.Type; rate?: Rate };
export type CcdPaymentToken = { type: "Ccd"; id?: undefined; contract?: undefined; rate?: Rate };
export type PaymentToken = Cis2PaymentToken | CcdPaymentToken;

export const arePaymentTokensEqual = (a: PaymentToken, b?: PaymentToken) => {
	if (a === b) {
		return true;
	}

	if (!b) {
		return false;
	}

	if (a.type !== b.type) {
		return false;
	}

	if (a.type === "Cis2") {
		return (
			a.id === b.id &&
			a.contract?.index === b.contract?.index &&
			a.contract?.subindex === b.contract?.subindex &&
			((a.rate === undefined && b.rate === undefined) ||
				(a.rate !== undefined &&
					b.rate !== undefined &&
					a.rate.numerator === b.rate.numerator &&
					a.rate.denominator === b.rate.denominator))
		);
	} else {
		return (
			(a.rate === undefined && b.rate === undefined) ||
			(a.rate !== undefined &&
				b.rate !== undefined &&
				a.rate.numerator === b.rate.numerator &&
				a.rate.denominator === b.rate.denominator)
		);
	}
};
export const toContractExchangeRate = (token: PaymentToken): ContractExchangeRate | undefined => {
	if (!token.rate) {
		return undefined;
	}

	switch (token.type) {
		case "Ccd":
			return { Ccd: [toContractRate(token.rate)] };
		case "Cis2":
			return {
				Cis2: [
					[
						{
							contract: { index: Number(token.contract.index), subindex: Number(token.contract.subindex) },
							id: token.id,
						},
						toContractRate(token.rate),
					],
				],
			};
	}
};
export const fromContractExchangeRate = (token: ContractExchangeRate): PaymentToken | undefined => {
	if ("Ccd" in token) {
		return { type: "Ccd", rate: fromContractRate(token.Ccd[0]) };
	} else if ("Cis2" in token) {
		return {
			type: "Cis2",
			rate: fromContractRate(token.Cis2[0][1]),
			id: token.Cis2[0][0].id,
			contract: ContractAddress.create(token.Cis2[0][0].contract.index, token.Cis2[0][0].contract.subindex),
		};
	}
};
