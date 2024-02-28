import { RJSFSchema, RegistryWidgetsType, UiSchema } from "@rjsf/utils";
import React from "react";
import { ContractAddress } from "@concordium/web-sdk";
import { default as client } from "./rwaSponsor";
import * as types from "./rwaSponsor";
import { GenericInit, GenericInvoke, GenericUpdate } from "./GenericContractUI";
export const initErrorJsonSchema: RJSFSchema = {
	type: "object",
	title: "Init Error",
	properties: {
		tag: {
			type: "string",
			enum: [
				"ParseError",
				"LogError",
				"SerializationError",
				"CallContractError",
				"CIS3CheckError",
			],
		},
	},
	required: ["tag"],
	dependencies: {
		tag: {
			oneOf: [
				{
					properties: {
						tag: { enum: ["ParseError"] },
						ParseError: { type: "object", title: "ParseError", properties: {} },
					},
				},
				{
					properties: {
						tag: { enum: ["LogError"] },
						LogError: { type: "object", title: "LogError", properties: {} },
					},
				},
				{
					properties: {
						tag: { enum: ["SerializationError"] },
						SerializationError: {
							type: "object",
							title: "SerializationError",
							properties: {},
						},
					},
				},
				{
					properties: {
						tag: { enum: ["CallContractError"] },
						CallContractError: {
							type: "object",
							title: "CallContractError",
							properties: {},
						},
					},
				},
				{
					properties: {
						tag: { enum: ["CIS3CheckError"] },
						CIS3CheckError: {
							type: "object",
							title: "CIS3CheckError",
							properties: {},
						},
					},
				},
			],
		},
	},
};
export type initErrorUi =
	| { tag: "ParseError"; ParseError: never }
	| { tag: "LogError"; LogError: never }
	| { tag: "SerializationError"; SerializationError: never }
	| { tag: "CallContractError"; CallContractError: never }
	| { tag: "CIS3CheckError"; CIS3CheckError: never };
export const bytesToSignRequestJsonSchema: RJSFSchema = {
	type: "object",
	title: "Bytes To Sign Request",
	properties: {
		contract_address: {
			type: "object",
			title: "Contract Address",
			properties: {
				index: { type: "integer", minimum: 0 },
				subindex: { type: "integer", minimum: 0 },
			},
		},
		nonce: { type: "integer", minimum: 0, title: "Nonce" },
		timestamp: { type: "string", format: "date-time", title: "Timestamp" },
		entry_point: { type: "string", title: "Entry Point" },
		payload: {
			type: "array",
			items: { type: "integer", minimum: 0, maximum: 255, title: "" },
			title: "Payload",
		},
	},
};
export type BytesToSignRequestUi = {
	contract_address: { index: number; subindex: number };
	nonce: number;
	timestamp: string;
	entry_point: string;
	payload: number[];
};
export const bytesToSignResponseJsonSchema: RJSFSchema = {
	type: "array",
	items: { type: "integer", minimum: 0, maximum: 255, title: "" },
	title: "Bytes To Sign Response",
};
export type BytesToSignResponseUi = number[];
export const bytesToSignErrorJsonSchema: RJSFSchema = {
	type: "object",
	title: "Bytes To Sign Error",
	properties: {
		tag: {
			type: "string",
			enum: [
				"ParseError",
				"LogError",
				"SerializationError",
				"CallContractError",
				"CIS3CheckError",
			],
		},
	},
	required: ["tag"],
	dependencies: {
		tag: {
			oneOf: [
				{
					properties: {
						tag: { enum: ["ParseError"] },
						ParseError: { type: "object", title: "ParseError", properties: {} },
					},
				},
				{
					properties: {
						tag: { enum: ["LogError"] },
						LogError: { type: "object", title: "LogError", properties: {} },
					},
				},
				{
					properties: {
						tag: { enum: ["SerializationError"] },
						SerializationError: {
							type: "object",
							title: "SerializationError",
							properties: {},
						},
					},
				},
				{
					properties: {
						tag: { enum: ["CallContractError"] },
						CallContractError: {
							type: "object",
							title: "CallContractError",
							properties: {},
						},
					},
				},
				{
					properties: {
						tag: { enum: ["CIS3CheckError"] },
						CIS3CheckError: {
							type: "object",
							title: "CIS3CheckError",
							properties: {},
						},
					},
				},
			],
		},
	},
};
export type BytesToSignErrorUi =
	| { tag: "ParseError"; ParseError: never }
	| { tag: "LogError"; LogError: never }
	| { tag: "SerializationError"; SerializationError: never }
	| { tag: "CallContractError"; CallContractError: never }
	| { tag: "CIS3CheckError"; CIS3CheckError: never };
export const nonceRequestJsonSchema: RJSFSchema = {
	type: "object",
	title: "Nonce Request",
	properties: { account: { type: "string", title: "Account" } },
};
export type NonceRequestUi = { account: string };
export const nonceResponseJsonSchema: RJSFSchema = {
	type: "integer",
	minimum: 0,
	title: "Nonce Response",
};
export type NonceResponseUi = number;
export const nonceErrorJsonSchema: RJSFSchema = {
	type: "object",
	title: "Nonce Error",
	properties: {
		tag: {
			type: "string",
			enum: [
				"ParseError",
				"LogError",
				"SerializationError",
				"CallContractError",
				"CIS3CheckError",
			],
		},
	},
	required: ["tag"],
	dependencies: {
		tag: {
			oneOf: [
				{
					properties: {
						tag: { enum: ["ParseError"] },
						ParseError: { type: "object", title: "ParseError", properties: {} },
					},
				},
				{
					properties: {
						tag: { enum: ["LogError"] },
						LogError: { type: "object", title: "LogError", properties: {} },
					},
				},
				{
					properties: {
						tag: { enum: ["SerializationError"] },
						SerializationError: {
							type: "object",
							title: "SerializationError",
							properties: {},
						},
					},
				},
				{
					properties: {
						tag: { enum: ["CallContractError"] },
						CallContractError: {
							type: "object",
							title: "CallContractError",
							properties: {},
						},
					},
				},
				{
					properties: {
						tag: { enum: ["CIS3CheckError"] },
						CIS3CheckError: {
							type: "object",
							title: "CIS3CheckError",
							properties: {},
						},
					},
				},
			],
		},
	},
};
export type NonceErrorUi =
	| { tag: "ParseError"; ParseError: never }
	| { tag: "LogError"; LogError: never }
	| { tag: "SerializationError"; SerializationError: never }
	| { tag: "CallContractError"; CallContractError: never }
	| { tag: "CIS3CheckError"; CIS3CheckError: never };
export const permitRequestJsonSchema: RJSFSchema = {
	type: "object",
	title: "Permit Request",
	properties: {
		signature: {
			type: "array",
			items: [
				{ type: "integer", minimum: 0, maximum: 255, title: "Key" },
				{
					type: "array",
					items: [
						{ type: "integer", minimum: 0, maximum: 255, title: "Key" },
						{
							type: "object",
							title: "Value",
							properties: { tag: { type: "string", enum: ["Ed25519"] } },
							required: ["tag"],
							dependencies: {
								tag: {
									oneOf: [
										{
											properties: {
												tag: { enum: ["Ed25519"] },
												Ed25519: {
													type: "array",
													items: [{ type: "string", title: "" }],
												},
											},
										},
									],
								},
							},
						},
					],
					title: "Value",
				},
			],
			title: "Signature",
		},
		signer: { type: "string", title: "Signer" },
		message: {
			type: "object",
			title: "Message",
			properties: {
				contract_address: {
					type: "object",
					title: "Contract Address",
					properties: {
						index: { type: "integer", minimum: 0 },
						subindex: { type: "integer", minimum: 0 },
					},
				},
				nonce: { type: "integer", minimum: 0, title: "Nonce" },
				timestamp: { type: "string", format: "date-time", title: "Timestamp" },
				entry_point: { type: "string", title: "Entry Point" },
				payload: {
					type: "array",
					items: { type: "integer", minimum: 0, maximum: 255, title: "" },
					title: "Payload",
				},
			},
		},
	},
};
export type PermitRequestUi = {
	signature: [number, [number, { tag: "Ed25519"; Ed25519: [string] }][]][];
	signer: string;
	message: {
		contract_address: { index: number; subindex: number };
		nonce: number;
		timestamp: string;
		entry_point: string;
		payload: number[];
	};
};
export const permitErrorJsonSchema: RJSFSchema = {
	type: "object",
	title: "Permit Error",
	properties: {
		tag: {
			type: "string",
			enum: [
				"Parse",
				"Log",
				"WrongContract",
				"Expired",
				"NonceMismatch",
				"WrongSignature",
				"Serialization",
				"AccountMissing",
				"CallContractAmountTooLarge",
				"CallContractMissingAccount",
				"CallContractMissingContract",
				"CallContractMissingEntrypoint",
				"CallContractMessageFailed",
				"CallContractTrap",
				"CallContractLogicReject",
			],
		},
	},
	required: ["tag"],
	dependencies: {
		tag: {
			oneOf: [
				{
					properties: {
						tag: { enum: ["Parse"] },
						Parse: { type: "object", title: "Parse", properties: {} },
					},
				},
				{
					properties: {
						tag: { enum: ["Log"] },
						Log: { type: "object", title: "Log", properties: {} },
					},
				},
				{
					properties: {
						tag: { enum: ["WrongContract"] },
						WrongContract: {
							type: "object",
							title: "WrongContract",
							properties: {},
						},
					},
				},
				{
					properties: {
						tag: { enum: ["Expired"] },
						Expired: { type: "object", title: "Expired", properties: {} },
					},
				},
				{
					properties: {
						tag: { enum: ["NonceMismatch"] },
						NonceMismatch: {
							type: "object",
							title: "NonceMismatch",
							properties: {},
						},
					},
				},
				{
					properties: {
						tag: { enum: ["WrongSignature"] },
						WrongSignature: {
							type: "object",
							title: "WrongSignature",
							properties: {},
						},
					},
				},
				{
					properties: {
						tag: { enum: ["Serialization"] },
						Serialization: {
							type: "object",
							title: "Serialization",
							properties: {},
						},
					},
				},
				{
					properties: {
						tag: { enum: ["AccountMissing"] },
						AccountMissing: {
							type: "object",
							title: "AccountMissing",
							properties: {},
						},
					},
				},
				{
					properties: {
						tag: { enum: ["CallContractAmountTooLarge"] },
						CallContractAmountTooLarge: {
							type: "object",
							title: "CallContractAmountTooLarge",
							properties: {},
						},
					},
				},
				{
					properties: {
						tag: { enum: ["CallContractMissingAccount"] },
						CallContractMissingAccount: {
							type: "object",
							title: "CallContractMissingAccount",
							properties: {},
						},
					},
				},
				{
					properties: {
						tag: { enum: ["CallContractMissingContract"] },
						CallContractMissingContract: {
							type: "object",
							title: "CallContractMissingContract",
							properties: {},
						},
					},
				},
				{
					properties: {
						tag: { enum: ["CallContractMissingEntrypoint"] },
						CallContractMissingEntrypoint: {
							type: "object",
							title: "CallContractMissingEntrypoint",
							properties: {},
						},
					},
				},
				{
					properties: {
						tag: { enum: ["CallContractMessageFailed"] },
						CallContractMessageFailed: {
							type: "object",
							title: "CallContractMessageFailed",
							properties: {},
						},
					},
				},
				{
					properties: {
						tag: { enum: ["CallContractTrap"] },
						CallContractTrap: {
							type: "object",
							title: "CallContractTrap",
							properties: {},
						},
					},
				},
				{
					properties: {
						tag: { enum: ["CallContractLogicReject"] },
						CallContractLogicReject: {
							type: "array",
							items: [
								{
									type: "integer",
									minimum: -2147483648,
									maximum: 2147483647,
									title: "",
								},
							],
						},
					},
				},
			],
		},
	},
};
export type PermitErrorUi =
	| { tag: "Parse"; Parse: never }
	| { tag: "Log"; Log: never }
	| { tag: "WrongContract"; WrongContract: never }
	| { tag: "Expired"; Expired: never }
	| { tag: "NonceMismatch"; NonceMismatch: never }
	| { tag: "WrongSignature"; WrongSignature: never }
	| { tag: "Serialization"; Serialization: never }
	| { tag: "AccountMissing"; AccountMissing: never }
	| { tag: "CallContractAmountTooLarge"; CallContractAmountTooLarge: never }
	| { tag: "CallContractMissingAccount"; CallContractMissingAccount: never }
	| { tag: "CallContractMissingContract"; CallContractMissingContract: never }
	| {
			tag: "CallContractMissingEntrypoint";
			CallContractMissingEntrypoint: never;
	  }
	| { tag: "CallContractMessageFailed"; CallContractMessageFailed: never }
	| { tag: "CallContractTrap"; CallContractTrap: never }
	| { tag: "CallContractLogicReject"; CallContractLogicReject: [number] };
export const supportsPermitRequestJsonSchema: RJSFSchema = {
	type: "object",
	title: "Supports Permit Request",
	properties: {
		queries: {
			type: "array",
			items: { type: "string", title: "" },
			title: "Queries",
		},
	},
};
export type SupportsPermitRequestUi = { queries: string[] };
export const supportsPermitResponseJsonSchema: RJSFSchema = {
	type: "array",
	items: { type: "boolean", title: "" },
	title: "Supports Permit Response",
};
export type SupportsPermitResponseUi = boolean[];
export const supportsPermitErrorJsonSchema: RJSFSchema = {
	type: "object",
	title: "Supports Permit Error",
	properties: {
		tag: {
			type: "string",
			enum: [
				"ParseError",
				"LogError",
				"SerializationError",
				"CallContractError",
				"CIS3CheckError",
			],
		},
	},
	required: ["tag"],
	dependencies: {
		tag: {
			oneOf: [
				{
					properties: {
						tag: { enum: ["ParseError"] },
						ParseError: { type: "object", title: "ParseError", properties: {} },
					},
				},
				{
					properties: {
						tag: { enum: ["LogError"] },
						LogError: { type: "object", title: "LogError", properties: {} },
					},
				},
				{
					properties: {
						tag: { enum: ["SerializationError"] },
						SerializationError: {
							type: "object",
							title: "SerializationError",
							properties: {},
						},
					},
				},
				{
					properties: {
						tag: { enum: ["CallContractError"] },
						CallContractError: {
							type: "object",
							title: "CallContractError",
							properties: {},
						},
					},
				},
				{
					properties: {
						tag: { enum: ["CIS3CheckError"] },
						CIS3CheckError: {
							type: "object",
							title: "CIS3CheckError",
							properties: {},
						},
					},
				},
			],
		},
	},
};
export type SupportsPermitErrorUi =
	| { tag: "ParseError"; ParseError: never }
	| { tag: "LogError"; LogError: never }
	| { tag: "SerializationError"; SerializationError: never }
	| { tag: "CallContractError"; CallContractError: never }
	| { tag: "CIS3CheckError"; CIS3CheckError: never };
export const init = (props: {
	onInitialize: (contract: ContractAddress.Type) => void;
	uiSchema?: UiSchema;
	uiWidgets?: RegistryWidgetsType;
}) =>
	GenericInit<never, never>({
		onContractInitialized: props.onInitialize,
		uiSchema: props.uiSchema,
		uiWidgets: props.uiWidgets,
		method: client.init,
	});
export const ENTRYPOINTS_UI: {
	[key: keyof typeof types.ENTRYPOINTS]: (props: {
		contract: ContractAddress.Type;
		uiSchema?: UiSchema;
		uiWidgets?: RegistryWidgetsType;
	}) => React.JSX.Element;
} = {
	bytesToSign: (props: {
		contract: ContractAddress.Type;
		uiSchema?: UiSchema;
		uiWidgets?: RegistryWidgetsType;
	}) =>
		GenericInvoke<
			types.BytesToSignRequest,
			BytesToSignRequestUi,
			types.BytesToSignResponse,
			BytesToSignResponseUi,
			types.BytesToSignError,
			BytesToSignErrorUi
		>({
			...props,
			method: client.bytesToSign,
			requestJsonSchema: bytesToSignRequestJsonSchema,
			requestSchemaBase64: types.bytesToSignRequestSchemaBase64,
			responseJsonSchema: bytesToSignResponseJsonSchema,
			responseSchemaBase64: types.bytesToSignResponseSchemaBase64,
			errorJsonSchema: bytesToSignErrorJsonSchema,
			errorSchemaBase64: types.bytesToSignErrorSchemaBase64,
		}),
	nonce: (props: {
		contract: ContractAddress.Type;
		uiSchema?: UiSchema;
		uiWidgets?: RegistryWidgetsType;
	}) =>
		GenericInvoke<
			types.NonceRequest,
			NonceRequestUi,
			types.NonceResponse,
			NonceResponseUi,
			types.NonceError,
			NonceErrorUi
		>({
			...props,
			method: client.nonce,
			requestJsonSchema: nonceRequestJsonSchema,
			requestSchemaBase64: types.nonceRequestSchemaBase64,
			responseJsonSchema: nonceResponseJsonSchema,
			responseSchemaBase64: types.nonceResponseSchemaBase64,
			errorJsonSchema: nonceErrorJsonSchema,
			errorSchemaBase64: types.nonceErrorSchemaBase64,
		}),
	permit: (props: {
		contract: ContractAddress.Type;
		uiSchema?: UiSchema;
		uiWidgets?: RegistryWidgetsType;
	}) =>
		GenericUpdate<
			types.PermitRequest,
			PermitRequestUi,
			types.PermitError,
			PermitErrorUi
		>({
			...props,
			method: client.permit,
			requestJsonSchema: permitRequestJsonSchema,
			requestSchemaBase64: types.permitRequestSchemaBase64,
			errorJsonSchema: permitErrorJsonSchema,
			errorSchemaBase64: types.permitErrorSchemaBase64,
		}),
	supportsPermit: (props: {
		contract: ContractAddress.Type;
		uiSchema?: UiSchema;
		uiWidgets?: RegistryWidgetsType;
	}) =>
		GenericInvoke<
			types.SupportsPermitRequest,
			SupportsPermitRequestUi,
			types.SupportsPermitResponse,
			SupportsPermitResponseUi,
			types.SupportsPermitError,
			SupportsPermitErrorUi
		>({
			...props,
			method: client.supportsPermit,
			requestJsonSchema: supportsPermitRequestJsonSchema,
			requestSchemaBase64: types.supportsPermitRequestSchemaBase64,
			responseJsonSchema: supportsPermitResponseJsonSchema,
			responseSchemaBase64: types.supportsPermitResponseSchemaBase64,
			errorJsonSchema: supportsPermitErrorJsonSchema,
			errorSchemaBase64: types.supportsPermitErrorSchemaBase64,
		}),
};
