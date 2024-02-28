import {
	ContractEvent,
	ContractName,
	EntrypointName,
	ModuleReference,
} from "@concordium/web-sdk";
import { InitMethod, ReceiveMethod } from "./GenericContract";
export const CONTRACT_NAME = "rwa_sponsor";
export const initErrorSchemaBase64 =
	"FQUAAAAKAAAAUGFyc2VFcnJvcgIIAAAATG9nRXJyb3ICEgAAAFNlcmlhbGl6YXRpb25FcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICDgAAAENJUzNDaGVja0Vycm9yAg==";
export type BytesToSignError =
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { SerializationError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { CIS3CheckError: Record<string, never> };
export const bytesToSignErrorSchemaBase64 =
	"FQUAAAAKAAAAUGFyc2VFcnJvcgIIAAAATG9nRXJyb3ICEgAAAFNlcmlhbGl6YXRpb25FcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICDgAAAENJUzNDaGVja0Vycm9yAg==";
export type BytesToSignRequest = {
	contract_address: { index: number; subindex: number };
	nonce: bigint;
	timestamp: string;
	entry_point: string;
	payload: Array<number>;
};
export const bytesToSignRequestSchemaBase64 =
	"FAAFAAAAEAAAAGNvbnRyYWN0X2FkZHJlc3MMBQAAAG5vbmNlBQkAAAB0aW1lc3RhbXANCwAAAGVudHJ5X3BvaW50FgEHAAAAcGF5bG9hZBABAg==";
export type BytesToSignResponse = Array<number>;
export const bytesToSignResponseSchemaBase64 = "EyAAAAAC";
export type NonceError =
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { SerializationError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { CIS3CheckError: Record<string, never> };
export const nonceErrorSchemaBase64 =
	"FQUAAAAKAAAAUGFyc2VFcnJvcgIIAAAATG9nRXJyb3ICEgAAAFNlcmlhbGl6YXRpb25FcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICDgAAAENJUzNDaGVja0Vycm9yAg==";
export type NonceRequest = { account: string };
export const nonceRequestSchemaBase64 = "FAABAAAABwAAAGFjY291bnQL";
export type NonceResponse = bigint;
export const nonceResponseSchemaBase64 = "BQ==";
export type PermitError =
	| { Parse: Record<string, never> }
	| { Log: Record<string, never> }
	| { WrongContract: Record<string, never> }
	| { Expired: Record<string, never> }
	| { NonceMismatch: Record<string, never> }
	| { WrongSignature: Record<string, never> }
	| { Serialization: Record<string, never> }
	| { AccountMissing: Record<string, never> }
	| { CallContractAmountTooLarge: Record<string, never> }
	| { CallContractMissingAccount: Record<string, never> }
	| { CallContractMissingContract: Record<string, never> }
	| { CallContractMissingEntrypoint: Record<string, never> }
	| { CallContractMessageFailed: Record<string, never> }
	| { CallContractTrap: Record<string, never> }
	| { CallContractLogicReject: [number] };
export const permitErrorSchemaBase64 =
	"FQ8AAAAFAAAAUGFyc2UCAwAAAExvZwINAAAAV3JvbmdDb250cmFjdAIHAAAARXhwaXJlZAINAAAATm9uY2VNaXNtYXRjaAIOAAAAV3JvbmdTaWduYXR1cmUCDQAAAFNlcmlhbGl6YXRpb24CDgAAAEFjY291bnRNaXNzaW5nAhoAAABDYWxsQ29udHJhY3RBbW91bnRUb29MYXJnZQIaAAAAQ2FsbENvbnRyYWN0TWlzc2luZ0FjY291bnQCGwAAAENhbGxDb250cmFjdE1pc3NpbmdDb250cmFjdAIdAAAAQ2FsbENvbnRyYWN0TWlzc2luZ0VudHJ5cG9pbnQCGQAAAENhbGxDb250cmFjdE1lc3NhZ2VGYWlsZWQCEAAAAENhbGxDb250cmFjdFRyYXACFwAAAENhbGxDb250cmFjdExvZ2ljUmVqZWN0AQEAAAAI";
export type PermitRequest = {
	signature: [number, [number, { Ed25519: [string] }][]][];
	signer: string;
	message: {
		contract_address: { index: number; subindex: number };
		nonce: bigint;
		timestamp: string;
		entry_point: string;
		payload: Array<number>;
	};
};
export const permitRequestSchemaBase64 =
	"FAADAAAACQAAAHNpZ25hdHVyZRIAAhIAAhUBAAAABwAAAEVkMjU1MTkBAQAAAB5AAAAABgAAAHNpZ25lcgsHAAAAbWVzc2FnZRQABQAAABAAAABjb250cmFjdF9hZGRyZXNzDAUAAABub25jZQUJAAAAdGltZXN0YW1wDQsAAABlbnRyeV9wb2ludBYBBwAAAHBheWxvYWQQAQI=";
export type SupportsPermitError =
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { SerializationError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { CIS3CheckError: Record<string, never> };
export const supportsPermitErrorSchemaBase64 =
	"FQUAAAAKAAAAUGFyc2VFcnJvcgIIAAAATG9nRXJyb3ICEgAAAFNlcmlhbGl6YXRpb25FcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICDgAAAENJUzNDaGVja0Vycm9yAg==";
export type SupportsPermitRequest = { queries: Array<string> };
export const supportsPermitRequestSchemaBase64 = "FAABAAAABwAAAHF1ZXJpZXMQARYB";
export type SupportsPermitResponse = Array<boolean>;
export const supportsPermitResponseSchemaBase64 = "EAEB";
export type event = { Nonce: [{ account: string; nonce: bigint }] };
export const eventSchemaBase64 =
	"FQEAAAAFAAAATm9uY2UBAQAAABQAAgAAAAcAAABhY2NvdW50CwUAAABub25jZQU=";
export const ENTRYPOINTS: Record<string, EntrypointName.Type> = {
	bytesToSign: EntrypointName.fromString("bytesToSign"),
	nonce: EntrypointName.fromString("nonce"),
	permit: EntrypointName.fromString("permit"),
	supportsPermit: EntrypointName.fromString("supportsPermit"),
};
export const ENTRYPOINT_DISPLAY_NAMES: Record<string, string> = {
	bytesToSign: "Bytes To Sign",
	nonce: "Nonce",
	permit: "Permit",
	supportsPermit: "Supports Permit",
};
export const rwaSponsor = {
	init: new InitMethod<void>(
		ModuleReference.fromHexString(
			"918ae7df77db838f4c085ade260adc162ebc5576c9d83bbfb6c9ffe89db2a916",
		),
		ContractName.fromString("rwa_sponsor"),
	),
	bytesToSign: new ReceiveMethod<
		BytesToSignRequest,
		BytesToSignResponse,
		BytesToSignError
	>(
		ContractName.fromString("rwa_sponsor"),
		EntrypointName.fromString("bytesToSign"),
		bytesToSignRequestSchemaBase64,
		bytesToSignResponseSchemaBase64,
		bytesToSignErrorSchemaBase64,
	),
	nonce: new ReceiveMethod<NonceRequest, NonceResponse, NonceError>(
		ContractName.fromString("rwa_sponsor"),
		EntrypointName.fromString("nonce"),
		nonceRequestSchemaBase64,
		nonceResponseSchemaBase64,
		nonceErrorSchemaBase64,
	),
	permit: new ReceiveMethod<PermitRequest, never, PermitError>(
		ContractName.fromString("rwa_sponsor"),
		EntrypointName.fromString("permit"),
		permitRequestSchemaBase64,
		undefined,
		permitErrorSchemaBase64,
	),
	supportsPermit: new ReceiveMethod<
		SupportsPermitRequest,
		SupportsPermitResponse,
		SupportsPermitError
	>(
		ContractName.fromString("rwa_sponsor"),
		EntrypointName.fromString("supportsPermit"),
		supportsPermitRequestSchemaBase64,
		supportsPermitResponseSchemaBase64,
		supportsPermitErrorSchemaBase64,
	),
	deserializeEvent: (event: ContractEvent.Type): event => {
		return ContractEvent.parseWithSchemaTypeBase64(
			event,
			eventSchemaBase64,
		) as event;
	},
};
export default rwaSponsor;
