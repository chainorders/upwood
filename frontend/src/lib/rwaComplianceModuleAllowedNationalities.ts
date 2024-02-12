import {
	ContractEvent,
	ContractName,
	EntrypointName,
	ModuleReference,
} from "@concordium/web-sdk";
import { InitMethod, ReceiveMethod } from "./GenericContract";
export const CONTRACT_NAME = "rwa_compliance_module_allowed_nationalities";
export type initRequest = {
	nationalities: Array<string>;
	identity_registry: { index: number; subindex: number };
};
export const initRequestSchemaBase64 =
	"FAACAAAADQAAAG5hdGlvbmFsaXRpZXMQAhYCEQAAAGlkZW50aXR5X3JlZ2lzdHJ5DA==";
export const initErrorSchemaBase64 =
	"FQcAAAAKAAAAUGFyc2VFcnJvcgIIAAAATG9nRXJyb3ICDQAAAEludmFsaWRNb2R1bGUCEQAAAENhbGxDb250cmFjdEVycm9yAgwAAABVbmF1dGhvcml6ZWQCEgAAAEFnZW50QWxyZWFkeUV4aXN0cwINAAAAQWdlbnROb3RGb3VuZAI=";
export type BurnedError =
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { InvalidModule: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const burnedErrorSchemaBase64 =
	"FQcAAAAKAAAAUGFyc2VFcnJvcgIIAAAATG9nRXJyb3ICDQAAAEludmFsaWRNb2R1bGUCEQAAAENhbGxDb250cmFjdEVycm9yAgwAAABVbmF1dGhvcml6ZWQCEgAAAEFnZW50QWxyZWFkeUV4aXN0cwINAAAAQWdlbnROb3RGb3VuZAI=";
export type CanTransferError =
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { InvalidModule: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const canTransferErrorSchemaBase64 =
	"FQcAAAAKAAAAUGFyc2VFcnJvcgIIAAAATG9nRXJyb3ICDQAAAEludmFsaWRNb2R1bGUCEQAAAENhbGxDb250cmFjdEVycm9yAgwAAABVbmF1dGhvcml6ZWQCEgAAAEFnZW50QWxyZWFkeUV4aXN0cwINAAAAQWdlbnROb3RGb3VuZAI=";
export type CanTransferRequest = {
	token_id: { token_id: string; contract: { index: number; subindex: number } };
	to:
		| { Account: [string] }
		| { Contract: [{ index: number; subindex: number }] };
	amount: string;
};
export const canTransferRequestSchemaBase64 =
	"FAADAAAACAAAAHRva2VuX2lkFAACAAAACAAAAHRva2VuX2lkHQAIAAAAY29udHJhY3QMAgAAAHRvFQIAAAAHAAAAQWNjb3VudAEBAAAACwgAAABDb250cmFjdAEBAAAADAYAAABhbW91bnQbJQAAAA==";
export type CanTransferResponse = boolean;
export const canTransferResponseSchemaBase64 = "AQ==";
export type MintedError =
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { InvalidModule: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const mintedErrorSchemaBase64 =
	"FQcAAAAKAAAAUGFyc2VFcnJvcgIIAAAATG9nRXJyb3ICDQAAAEludmFsaWRNb2R1bGUCEQAAAENhbGxDb250cmFjdEVycm9yAgwAAABVbmF1dGhvcml6ZWQCEgAAAEFnZW50QWxyZWFkeUV4aXN0cwINAAAAQWdlbnROb3RGb3VuZAI=";
export type TransferredError =
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { InvalidModule: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const transferredErrorSchemaBase64 =
	"FQcAAAAKAAAAUGFyc2VFcnJvcgIIAAAATG9nRXJyb3ICDQAAAEludmFsaWRNb2R1bGUCEQAAAENhbGxDb250cmFjdEVycm9yAgwAAABVbmF1dGhvcml6ZWQCEgAAAEFnZW50QWxyZWFkeUV4aXN0cwINAAAAQWdlbnROb3RGb3VuZAI=";
export const ENTRYPOINTS: Record<string, EntrypointName.Type> = {
	burned: EntrypointName.fromString("burned"),
	canTransfer: EntrypointName.fromString("canTransfer"),
	minted: EntrypointName.fromString("minted"),
	transferred: EntrypointName.fromString("transferred"),
};
export const ENTRYPOINT_DISPLAY_NAMES: Record<string, string> = {
	burned: "Burned",
	canTransfer: "Can Transfer",
	minted: "Minted",
	transferred: "Transferred",
};
export const rwaComplianceModuleAllowedNationalities = {
	init: new InitMethod<initRequest>(
		ModuleReference.fromHexString(
			"f42e0435c7e9bef137e5cf780bdd6fbb2be732baa1c086ed819b2ad86180017a",
		),
		ContractName.fromString("rwa_compliance_module_allowed_nationalities"),
		initRequestSchemaBase64,
	),
	burned: new ReceiveMethod<never, never, BurnedError>(
		ContractName.fromString("rwa_compliance_module_allowed_nationalities"),
		EntrypointName.fromString("burned"),
		undefined,
		undefined,
		burnedErrorSchemaBase64,
	),
	canTransfer: new ReceiveMethod<
		CanTransferRequest,
		CanTransferResponse,
		CanTransferError
	>(
		ContractName.fromString("rwa_compliance_module_allowed_nationalities"),
		EntrypointName.fromString("canTransfer"),
		canTransferRequestSchemaBase64,
		canTransferResponseSchemaBase64,
		canTransferErrorSchemaBase64,
	),
	minted: new ReceiveMethod<never, never, MintedError>(
		ContractName.fromString("rwa_compliance_module_allowed_nationalities"),
		EntrypointName.fromString("minted"),
		undefined,
		undefined,
		mintedErrorSchemaBase64,
	),
	transferred: new ReceiveMethod<never, never, TransferredError>(
		ContractName.fromString("rwa_compliance_module_allowed_nationalities"),
		EntrypointName.fromString("transferred"),
		undefined,
		undefined,
		transferredErrorSchemaBase64,
	),
	deserializeEvent: (event: ContractEvent.Type): event => {
		return ContractEvent.parseWithSchemaTypeBase64(
			event,
			eventSchemaBase64,
		) as event;
	},
};
export default rwaComplianceModuleAllowedNationalities;
