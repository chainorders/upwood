import { ContractEvent, ContractName, EntrypointName, ModuleReference } from "@concordium/web-sdk";
import { InitMethod, ReceiveMethod } from "./GenericContract";
export const CONTRACT_NAME = "rwa_sponsor";
export const initErrorSchemaBase64 =
	"FQsAAAAKAAAAUGFyc2VFcnJvcgIIAAAATG9nRXJyb3ICDQAAAFdyb25nQ29udHJhY3QCBwAAAEV4cGlyZWQCDQAAAE5vbmNlTWlzbWF0Y2gCDgAAAFdyb25nU2lnbmF0dXJlAhIAAABTZXJpYWxpemF0aW9uRXJyb3ICDgAAAEFjY291bnRNaXNzaW5nAhEAAABDYWxsQ29udHJhY3RFcnJvcgISAAAAQ0lTM05vdEltcGxlbWVudGVkAg4AAABDSVMzQ2hlY2tFcnJvcgI=";
export type PermitError =
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { WrongContract: Record<string, never> }
	| { Expired: Record<string, never> }
	| { NonceMismatch: Record<string, never> }
	| { WrongSignature: Record<string, never> }
	| { SerializationError: Record<string, never> }
	| { AccountMissing: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { CIS3NotImplemented: Record<string, never> }
	| { CIS3CheckError: Record<string, never> };
export const permitErrorSchemaBase64 =
	"FQsAAAAKAAAAUGFyc2VFcnJvcgIIAAAATG9nRXJyb3ICDQAAAFdyb25nQ29udHJhY3QCBwAAAEV4cGlyZWQCDQAAAE5vbmNlTWlzbWF0Y2gCDgAAAFdyb25nU2lnbmF0dXJlAhIAAABTZXJpYWxpemF0aW9uRXJyb3ICDgAAAEFjY291bnRNaXNzaW5nAhEAAABDYWxsQ29udHJhY3RFcnJvcgISAAAAQ0lTM05vdEltcGxlbWVudGVkAg4AAABDSVMzQ2hlY2tFcnJvcgI=";
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
	| { WrongContract: Record<string, never> }
	| { Expired: Record<string, never> }
	| { NonceMismatch: Record<string, never> }
	| { WrongSignature: Record<string, never> }
	| { SerializationError: Record<string, never> }
	| { AccountMissing: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { CIS3NotImplemented: Record<string, never> }
	| { CIS3CheckError: Record<string, never> };
export const supportsPermitErrorSchemaBase64 =
	"FQsAAAAKAAAAUGFyc2VFcnJvcgIIAAAATG9nRXJyb3ICDQAAAFdyb25nQ29udHJhY3QCBwAAAEV4cGlyZWQCDQAAAE5vbmNlTWlzbWF0Y2gCDgAAAFdyb25nU2lnbmF0dXJlAhIAAABTZXJpYWxpemF0aW9uRXJyb3ICDgAAAEFjY291bnRNaXNzaW5nAhEAAABDYWxsQ29udHJhY3RFcnJvcgISAAAAQ0lTM05vdEltcGxlbWVudGVkAg4AAABDSVMzQ2hlY2tFcnJvcgI=";
export type SupportsPermitRequest = { queries: Array<string> };
export const supportsPermitRequestSchemaBase64 = "FAABAAAABwAAAHF1ZXJpZXMQARYB";
export type SupportsPermitResponse = Array<boolean>;
export const supportsPermitResponseSchemaBase64 = "EAEB";
export type event = { Nonce: [{ account: string; nonce: bigint }] };
export const eventSchemaBase64 = "FQEAAAAFAAAATm9uY2UBAQAAABQAAgAAAAcAAABhY2NvdW50CwUAAABub25jZQU=";
export const ENTRYPOINTS: Record<string, EntrypointName.Type> = {
	permit: EntrypointName.fromString("permit"),
	supportsPermit: EntrypointName.fromString("supportsPermit"),
};
export const ENTRYPOINT_DISPLAY_NAMES: Record<string, string> = { permit: "Permit", supportsPermit: "Supports Permit" };
export const rwaSponsor = {
	init: new InitMethod<void>(
		ModuleReference.fromHexString("e0a6627a8be6686c06a18aa981567cef48ae0c60849d6d08a1335082e09525e8"),
		ContractName.fromString("rwa_sponsor")
	),
	permit: new ReceiveMethod<PermitRequest, never, PermitError>(
		ContractName.fromString("rwa_sponsor"),
		EntrypointName.fromString("permit"),
		permitRequestSchemaBase64,
		undefined,
		permitErrorSchemaBase64
	),
	supportsPermit: new ReceiveMethod<SupportsPermitRequest, SupportsPermitResponse, SupportsPermitError>(
		ContractName.fromString("rwa_sponsor"),
		EntrypointName.fromString("supportsPermit"),
		supportsPermitRequestSchemaBase64,
		supportsPermitResponseSchemaBase64,
		supportsPermitErrorSchemaBase64
	),
	deserializeEvent: (event: ContractEvent.Type): event => {
		return ContractEvent.parseWithSchemaTypeBase64(event, eventSchemaBase64) as event;
	},
};
export default rwaSponsor;
