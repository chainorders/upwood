import {
	ContractEvent,
	ContractName,
	EntrypointName,
	ModuleReference,
} from "@concordium/web-sdk";
import { InitMethod, ReceiveMethod } from "./GenericContract";
export const CONTRACT_NAME = "rwa_security_nft";
export type initRequest = {
	identity_registry: { index: number; subindex: number };
	compliance: { index: number; subindex: number };
	sponsors: Array<{ index: number; subindex: number }>;
};
export const initRequestSchemaBase64 =
	"FAADAAAAEQAAAGlkZW50aXR5X3JlZ2lzdHJ5DAoAAABjb21wbGlhbmNlDAgAAABzcG9uc29ycxACDA==";
export const initErrorSchemaBase64 =
	"FQ4AAAAOAAAASW52YWxpZFRva2VuSWQCEQAAAEluc3VmZmljaWVudEZ1bmRzAgwAAABVbmF1dGhvcml6ZWQCCgAAAFBhcnNlRXJyb3ICCAAAAExvZ0Vycm9yAhIAAABVblZlcmlmaWVkSWRlbnRpdHkCEwAAAEluQ29tcGxpYW50VHJhbnNmZXICDwAAAENvbXBsaWFuY2VFcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICCwAAAFBhdXNlZFRva2VuAg0AAABJbnZhbGlkQW1vdW50Ag4AAABJbnZhbGlkQWRkcmVzcwISAAAAQWdlbnRBbHJlYWR5RXhpc3RzAg0AAABBZ2VudE5vdEZvdW5kAg==";
export type AddAgentError =
	| { InvalidTokenId: Record<string, never> }
	| { InsufficientFunds: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { UnVerifiedIdentity: Record<string, never> }
	| { InCompliantTransfer: Record<string, never> }
	| { ComplianceError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { PausedToken: Record<string, never> }
	| { InvalidAmount: Record<string, never> }
	| { InvalidAddress: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const addAgentErrorSchemaBase64 =
	"FQ4AAAAOAAAASW52YWxpZFRva2VuSWQCEQAAAEluc3VmZmljaWVudEZ1bmRzAgwAAABVbmF1dGhvcml6ZWQCCgAAAFBhcnNlRXJyb3ICCAAAAExvZ0Vycm9yAhIAAABVblZlcmlmaWVkSWRlbnRpdHkCEwAAAEluQ29tcGxpYW50VHJhbnNmZXICDwAAAENvbXBsaWFuY2VFcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICCwAAAFBhdXNlZFRva2VuAg0AAABJbnZhbGlkQW1vdW50Ag4AAABJbnZhbGlkQWRkcmVzcwISAAAAQWdlbnRBbHJlYWR5RXhpc3RzAg0AAABBZ2VudE5vdEZvdW5kAg==";
export type AddAgentRequest =
	| { Account: [string] }
	| { Contract: [{ index: number; subindex: number }] };
export const addAgentRequestSchemaBase64 =
	"FQIAAAAHAAAAQWNjb3VudAEBAAAACwgAAABDb250cmFjdAEBAAAADA==";
export type AgentsError =
	| { InvalidTokenId: Record<string, never> }
	| { InsufficientFunds: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { UnVerifiedIdentity: Record<string, never> }
	| { InCompliantTransfer: Record<string, never> }
	| { ComplianceError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { PausedToken: Record<string, never> }
	| { InvalidAmount: Record<string, never> }
	| { InvalidAddress: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const agentsErrorSchemaBase64 =
	"FQ4AAAAOAAAASW52YWxpZFRva2VuSWQCEQAAAEluc3VmZmljaWVudEZ1bmRzAgwAAABVbmF1dGhvcml6ZWQCCgAAAFBhcnNlRXJyb3ICCAAAAExvZ0Vycm9yAhIAAABVblZlcmlmaWVkSWRlbnRpdHkCEwAAAEluQ29tcGxpYW50VHJhbnNmZXICDwAAAENvbXBsaWFuY2VFcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICCwAAAFBhdXNlZFRva2VuAg0AAABJbnZhbGlkQW1vdW50Ag4AAABJbnZhbGlkQWRkcmVzcwISAAAAQWdlbnRBbHJlYWR5RXhpc3RzAg0AAABBZ2VudE5vdEZvdW5kAg==";
export type AgentsResponse = Array<
	{ Account: [string] } | { Contract: [{ index: number; subindex: number }] }
>;
export const agentsResponseSchemaBase64 =
	"EAIVAgAAAAcAAABBY2NvdW50AQEAAAALCAAAAENvbnRyYWN0AQEAAAAM";
export type BalanceOfError =
	| { InvalidTokenId: Record<string, never> }
	| { InsufficientFunds: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { UnVerifiedIdentity: Record<string, never> }
	| { InCompliantTransfer: Record<string, never> }
	| { ComplianceError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { PausedToken: Record<string, never> }
	| { InvalidAmount: Record<string, never> }
	| { InvalidAddress: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const balanceOfErrorSchemaBase64 =
	"FQ4AAAAOAAAASW52YWxpZFRva2VuSWQCEQAAAEluc3VmZmljaWVudEZ1bmRzAgwAAABVbmF1dGhvcml6ZWQCCgAAAFBhcnNlRXJyb3ICCAAAAExvZ0Vycm9yAhIAAABVblZlcmlmaWVkSWRlbnRpdHkCEwAAAEluQ29tcGxpYW50VHJhbnNmZXICDwAAAENvbXBsaWFuY2VFcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICCwAAAFBhdXNlZFRva2VuAg0AAABJbnZhbGlkQW1vdW50Ag4AAABJbnZhbGlkQWRkcmVzcwISAAAAQWdlbnRBbHJlYWR5RXhpc3RzAg0AAABBZ2VudE5vdEZvdW5kAg==";
export type BalanceOfRequest = Array<{
	token_id: string;
	address:
		| { Account: [string] }
		| { Contract: [{ index: number; subindex: number }] };
}>;
export const balanceOfRequestSchemaBase64 =
	"EAEUAAIAAAAIAAAAdG9rZW5faWQdAAcAAABhZGRyZXNzFQIAAAAHAAAAQWNjb3VudAEBAAAACwgAAABDb250cmFjdAEBAAAADA==";
export type BalanceOfResponse = Array<string>;
export const balanceOfResponseSchemaBase64 = "EAEbJQAAAA==";
export type BalanceOfFrozenError =
	| { InvalidTokenId: Record<string, never> }
	| { InsufficientFunds: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { UnVerifiedIdentity: Record<string, never> }
	| { InCompliantTransfer: Record<string, never> }
	| { ComplianceError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { PausedToken: Record<string, never> }
	| { InvalidAmount: Record<string, never> }
	| { InvalidAddress: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const balanceOfFrozenErrorSchemaBase64 =
	"FQ4AAAAOAAAASW52YWxpZFRva2VuSWQCEQAAAEluc3VmZmljaWVudEZ1bmRzAgwAAABVbmF1dGhvcml6ZWQCCgAAAFBhcnNlRXJyb3ICCAAAAExvZ0Vycm9yAhIAAABVblZlcmlmaWVkSWRlbnRpdHkCEwAAAEluQ29tcGxpYW50VHJhbnNmZXICDwAAAENvbXBsaWFuY2VFcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICCwAAAFBhdXNlZFRva2VuAg0AAABJbnZhbGlkQW1vdW50Ag4AAABJbnZhbGlkQWRkcmVzcwISAAAAQWdlbnRBbHJlYWR5RXhpc3RzAg0AAABBZ2VudE5vdEZvdW5kAg==";
export type BalanceOfFrozenRequest = {
	owner:
		| { Account: [string] }
		| { Contract: [{ index: number; subindex: number }] };
	tokens: Array<string>;
};
export const balanceOfFrozenRequestSchemaBase64 =
	"FAACAAAABQAAAG93bmVyFQIAAAAHAAAAQWNjb3VudAEBAAAACwgAAABDb250cmFjdAEBAAAADAYAAAB0b2tlbnMQAh0A";
export type BalanceOfFrozenResponse = { tokens: Array<string> };
export const balanceOfFrozenResponseSchemaBase64 =
	"FAABAAAABgAAAHRva2VucxACGyUAAAA=";
export type BalanceOfUnFrozenError =
	| { InvalidTokenId: Record<string, never> }
	| { InsufficientFunds: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { UnVerifiedIdentity: Record<string, never> }
	| { InCompliantTransfer: Record<string, never> }
	| { ComplianceError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { PausedToken: Record<string, never> }
	| { InvalidAmount: Record<string, never> }
	| { InvalidAddress: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const balanceOfUnFrozenErrorSchemaBase64 =
	"FQ4AAAAOAAAASW52YWxpZFRva2VuSWQCEQAAAEluc3VmZmljaWVudEZ1bmRzAgwAAABVbmF1dGhvcml6ZWQCCgAAAFBhcnNlRXJyb3ICCAAAAExvZ0Vycm9yAhIAAABVblZlcmlmaWVkSWRlbnRpdHkCEwAAAEluQ29tcGxpYW50VHJhbnNmZXICDwAAAENvbXBsaWFuY2VFcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICCwAAAFBhdXNlZFRva2VuAg0AAABJbnZhbGlkQW1vdW50Ag4AAABJbnZhbGlkQWRkcmVzcwISAAAAQWdlbnRBbHJlYWR5RXhpc3RzAg0AAABBZ2VudE5vdEZvdW5kAg==";
export type BalanceOfUnFrozenRequest = {
	owner:
		| { Account: [string] }
		| { Contract: [{ index: number; subindex: number }] };
	tokens: Array<string>;
};
export const balanceOfUnFrozenRequestSchemaBase64 =
	"FAACAAAABQAAAG93bmVyFQIAAAAHAAAAQWNjb3VudAEBAAAACwgAAABDb250cmFjdAEBAAAADAYAAAB0b2tlbnMQAh0A";
export type BalanceOfUnFrozenResponse = { tokens: Array<string> };
export const balanceOfUnFrozenResponseSchemaBase64 =
	"FAABAAAABgAAAHRva2VucxACGyUAAAA=";
export type BurnError =
	| { InvalidTokenId: Record<string, never> }
	| { InsufficientFunds: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { UnVerifiedIdentity: Record<string, never> }
	| { InCompliantTransfer: Record<string, never> }
	| { ComplianceError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { PausedToken: Record<string, never> }
	| { InvalidAmount: Record<string, never> }
	| { InvalidAddress: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const burnErrorSchemaBase64 =
	"FQ4AAAAOAAAASW52YWxpZFRva2VuSWQCEQAAAEluc3VmZmljaWVudEZ1bmRzAgwAAABVbmF1dGhvcml6ZWQCCgAAAFBhcnNlRXJyb3ICCAAAAExvZ0Vycm9yAhIAAABVblZlcmlmaWVkSWRlbnRpdHkCEwAAAEluQ29tcGxpYW50VHJhbnNmZXICDwAAAENvbXBsaWFuY2VFcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICCwAAAFBhdXNlZFRva2VuAg0AAABJbnZhbGlkQW1vdW50Ag4AAABJbnZhbGlkQWRkcmVzcwISAAAAQWdlbnRBbHJlYWR5RXhpc3RzAg0AAABBZ2VudE5vdEZvdW5kAg==";
export type BurnRequest = Array<{
	token_id: string;
	amount: string;
	owner:
		| { Account: [string] }
		| { Contract: [{ index: number; subindex: number }] };
}>;
export const burnRequestSchemaBase64 =
	"EAEUAAMAAAAIAAAAdG9rZW5faWQdAAYAAABhbW91bnQbJQAAAAUAAABvd25lchUCAAAABwAAAEFjY291bnQBAQAAAAsIAAAAQ29udHJhY3QBAQAAAAw=";
export type ComplianceResponse = { index: number; subindex: number };
export const complianceResponseSchemaBase64 = "DA==";
export type ForcedTransferError =
	| { InvalidTokenId: Record<string, never> }
	| { InsufficientFunds: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { UnVerifiedIdentity: Record<string, never> }
	| { InCompliantTransfer: Record<string, never> }
	| { ComplianceError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { PausedToken: Record<string, never> }
	| { InvalidAmount: Record<string, never> }
	| { InvalidAddress: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const forcedTransferErrorSchemaBase64 =
	"FQ4AAAAOAAAASW52YWxpZFRva2VuSWQCEQAAAEluc3VmZmljaWVudEZ1bmRzAgwAAABVbmF1dGhvcml6ZWQCCgAAAFBhcnNlRXJyb3ICCAAAAExvZ0Vycm9yAhIAAABVblZlcmlmaWVkSWRlbnRpdHkCEwAAAEluQ29tcGxpYW50VHJhbnNmZXICDwAAAENvbXBsaWFuY2VFcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICCwAAAFBhdXNlZFRva2VuAg0AAABJbnZhbGlkQW1vdW50Ag4AAABJbnZhbGlkQWRkcmVzcwISAAAAQWdlbnRBbHJlYWR5RXhpc3RzAg0AAABBZ2VudE5vdEZvdW5kAg==";
export type ForcedTransferRequest = Array<{
	token_id: string;
	amount: string;
	from:
		| { Account: [string] }
		| { Contract: [{ index: number; subindex: number }] };
	to:
		| { Account: [string] }
		| { Contract: [{ index: number; subindex: number }, string] };
	data: string;
}>;
export const forcedTransferRequestSchemaBase64 =
	"EAEUAAUAAAAIAAAAdG9rZW5faWQdAAYAAABhbW91bnQbJQAAAAQAAABmcm9tFQIAAAAHAAAAQWNjb3VudAEBAAAACwgAAABDb250cmFjdAEBAAAADAIAAAB0bxUCAAAABwAAAEFjY291bnQBAQAAAAsIAAAAQ29udHJhY3QBAgAAAAwWAQQAAABkYXRhHQE=";
export type FreezeError =
	| { InvalidTokenId: Record<string, never> }
	| { InsufficientFunds: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { UnVerifiedIdentity: Record<string, never> }
	| { InCompliantTransfer: Record<string, never> }
	| { ComplianceError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { PausedToken: Record<string, never> }
	| { InvalidAmount: Record<string, never> }
	| { InvalidAddress: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const freezeErrorSchemaBase64 =
	"FQ4AAAAOAAAASW52YWxpZFRva2VuSWQCEQAAAEluc3VmZmljaWVudEZ1bmRzAgwAAABVbmF1dGhvcml6ZWQCCgAAAFBhcnNlRXJyb3ICCAAAAExvZ0Vycm9yAhIAAABVblZlcmlmaWVkSWRlbnRpdHkCEwAAAEluQ29tcGxpYW50VHJhbnNmZXICDwAAAENvbXBsaWFuY2VFcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICCwAAAFBhdXNlZFRva2VuAg0AAABJbnZhbGlkQW1vdW50Ag4AAABJbnZhbGlkQWRkcmVzcwISAAAAQWdlbnRBbHJlYWR5RXhpc3RzAg0AAABBZ2VudE5vdEZvdW5kAg==";
export type FreezeRequest = {
	owner:
		| { Account: [string] }
		| { Contract: [{ index: number; subindex: number }] };
	tokens: Array<{ token_id: string; token_amount: string }>;
};
export const freezeRequestSchemaBase64 =
	"FAACAAAABQAAAG93bmVyFQIAAAAHAAAAQWNjb3VudAEBAAAACwgAAABDb250cmFjdAEBAAAADAYAAAB0b2tlbnMQAhQAAgAAAAgAAAB0b2tlbl9pZB0ADAAAAHRva2VuX2Ftb3VudBslAAAA";
export type IdentityRegistryResponse = { index: number; subindex: number };
export const identityRegistryResponseSchemaBase64 = "DA==";
export type IsAgentError =
	| { InvalidTokenId: Record<string, never> }
	| { InsufficientFunds: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { UnVerifiedIdentity: Record<string, never> }
	| { InCompliantTransfer: Record<string, never> }
	| { ComplianceError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { PausedToken: Record<string, never> }
	| { InvalidAmount: Record<string, never> }
	| { InvalidAddress: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const isAgentErrorSchemaBase64 =
	"FQ4AAAAOAAAASW52YWxpZFRva2VuSWQCEQAAAEluc3VmZmljaWVudEZ1bmRzAgwAAABVbmF1dGhvcml6ZWQCCgAAAFBhcnNlRXJyb3ICCAAAAExvZ0Vycm9yAhIAAABVblZlcmlmaWVkSWRlbnRpdHkCEwAAAEluQ29tcGxpYW50VHJhbnNmZXICDwAAAENvbXBsaWFuY2VFcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICCwAAAFBhdXNlZFRva2VuAg0AAABJbnZhbGlkQW1vdW50Ag4AAABJbnZhbGlkQWRkcmVzcwISAAAAQWdlbnRBbHJlYWR5RXhpc3RzAg0AAABBZ2VudE5vdEZvdW5kAg==";
export type IsAgentRequest =
	| { Account: [string] }
	| { Contract: [{ index: number; subindex: number }] };
export const isAgentRequestSchemaBase64 =
	"FQIAAAAHAAAAQWNjb3VudAEBAAAACwgAAABDb250cmFjdAEBAAAADA==";
export type IsAgentResponse = boolean;
export const isAgentResponseSchemaBase64 = "AQ==";
export type IsPausedError =
	| { InvalidTokenId: Record<string, never> }
	| { InsufficientFunds: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { UnVerifiedIdentity: Record<string, never> }
	| { InCompliantTransfer: Record<string, never> }
	| { ComplianceError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { PausedToken: Record<string, never> }
	| { InvalidAmount: Record<string, never> }
	| { InvalidAddress: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const isPausedErrorSchemaBase64 =
	"FQ4AAAAOAAAASW52YWxpZFRva2VuSWQCEQAAAEluc3VmZmljaWVudEZ1bmRzAgwAAABVbmF1dGhvcml6ZWQCCgAAAFBhcnNlRXJyb3ICCAAAAExvZ0Vycm9yAhIAAABVblZlcmlmaWVkSWRlbnRpdHkCEwAAAEluQ29tcGxpYW50VHJhbnNmZXICDwAAAENvbXBsaWFuY2VFcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICCwAAAFBhdXNlZFRva2VuAg0AAABJbnZhbGlkQW1vdW50Ag4AAABJbnZhbGlkQWRkcmVzcwISAAAAQWdlbnRBbHJlYWR5RXhpc3RzAg0AAABBZ2VudE5vdEZvdW5kAg==";
export type IsPausedRequest = { tokens: Array<string> };
export const isPausedRequestSchemaBase64 = "FAABAAAABgAAAHRva2VucxACHQA=";
export type IsPausedResponse = { tokens: Array<boolean> };
export const isPausedResponseSchemaBase64 = "FAABAAAABgAAAHRva2VucxACAQ==";
export type MintError =
	| { InvalidTokenId: Record<string, never> }
	| { InsufficientFunds: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { UnVerifiedIdentity: Record<string, never> }
	| { InCompliantTransfer: Record<string, never> }
	| { ComplianceError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { PausedToken: Record<string, never> }
	| { InvalidAmount: Record<string, never> }
	| { InvalidAddress: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const mintErrorSchemaBase64 =
	"FQ4AAAAOAAAASW52YWxpZFRva2VuSWQCEQAAAEluc3VmZmljaWVudEZ1bmRzAgwAAABVbmF1dGhvcml6ZWQCCgAAAFBhcnNlRXJyb3ICCAAAAExvZ0Vycm9yAhIAAABVblZlcmlmaWVkSWRlbnRpdHkCEwAAAEluQ29tcGxpYW50VHJhbnNmZXICDwAAAENvbXBsaWFuY2VFcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICCwAAAFBhdXNlZFRva2VuAg0AAABJbnZhbGlkQW1vdW50Ag4AAABJbnZhbGlkQWRkcmVzcwISAAAAQWdlbnRBbHJlYWR5RXhpc3RzAg0AAABBZ2VudE5vdEZvdW5kAg==";
export type MintRequest = {
	owner:
		| { Account: [string] }
		| { Contract: [{ index: number; subindex: number }, string] };
	tokens: Array<{
		metadata_url: {
			url: string;
			hash: { None: Record<string, never> } | { Some: [string] };
		};
	}>;
};
export const mintRequestSchemaBase64 =
	"FAACAAAABQAAAG93bmVyFQIAAAAHAAAAQWNjb3VudAEBAAAACwgAAABDb250cmFjdAECAAAADBYBBgAAAHRva2VucxACFAABAAAADAAAAG1ldGFkYXRhX3VybBQAAgAAAAMAAAB1cmwWAgQAAABoYXNoFQIAAAAEAAAATm9uZQIEAAAAU29tZQEBAAAAFgI=";
export type OperatorOfError =
	| { InvalidTokenId: Record<string, never> }
	| { InsufficientFunds: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { UnVerifiedIdentity: Record<string, never> }
	| { InCompliantTransfer: Record<string, never> }
	| { ComplianceError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { PausedToken: Record<string, never> }
	| { InvalidAmount: Record<string, never> }
	| { InvalidAddress: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const operatorOfErrorSchemaBase64 =
	"FQ4AAAAOAAAASW52YWxpZFRva2VuSWQCEQAAAEluc3VmZmljaWVudEZ1bmRzAgwAAABVbmF1dGhvcml6ZWQCCgAAAFBhcnNlRXJyb3ICCAAAAExvZ0Vycm9yAhIAAABVblZlcmlmaWVkSWRlbnRpdHkCEwAAAEluQ29tcGxpYW50VHJhbnNmZXICDwAAAENvbXBsaWFuY2VFcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICCwAAAFBhdXNlZFRva2VuAg0AAABJbnZhbGlkQW1vdW50Ag4AAABJbnZhbGlkQWRkcmVzcwISAAAAQWdlbnRBbHJlYWR5RXhpc3RzAg0AAABBZ2VudE5vdEZvdW5kAg==";
export type OperatorOfRequest = Array<{
	owner:
		| { Account: [string] }
		| { Contract: [{ index: number; subindex: number }] };
	address:
		| { Account: [string] }
		| { Contract: [{ index: number; subindex: number }] };
}>;
export const operatorOfRequestSchemaBase64 =
	"EAEUAAIAAAAFAAAAb3duZXIVAgAAAAcAAABBY2NvdW50AQEAAAALCAAAAENvbnRyYWN0AQEAAAAMBwAAAGFkZHJlc3MVAgAAAAcAAABBY2NvdW50AQEAAAALCAAAAENvbnRyYWN0AQEAAAAM";
export type OperatorOfResponse = Array<boolean>;
export const operatorOfResponseSchemaBase64 = "EAEB";
export type PauseError =
	| { InvalidTokenId: Record<string, never> }
	| { InsufficientFunds: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { UnVerifiedIdentity: Record<string, never> }
	| { InCompliantTransfer: Record<string, never> }
	| { ComplianceError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { PausedToken: Record<string, never> }
	| { InvalidAmount: Record<string, never> }
	| { InvalidAddress: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const pauseErrorSchemaBase64 =
	"FQ4AAAAOAAAASW52YWxpZFRva2VuSWQCEQAAAEluc3VmZmljaWVudEZ1bmRzAgwAAABVbmF1dGhvcml6ZWQCCgAAAFBhcnNlRXJyb3ICCAAAAExvZ0Vycm9yAhIAAABVblZlcmlmaWVkSWRlbnRpdHkCEwAAAEluQ29tcGxpYW50VHJhbnNmZXICDwAAAENvbXBsaWFuY2VFcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICCwAAAFBhdXNlZFRva2VuAg0AAABJbnZhbGlkQW1vdW50Ag4AAABJbnZhbGlkQWRkcmVzcwISAAAAQWdlbnRBbHJlYWR5RXhpc3RzAg0AAABBZ2VudE5vdEZvdW5kAg==";
export type PauseRequest = { tokens: Array<string> };
export const pauseRequestSchemaBase64 = "FAABAAAABgAAAHRva2VucxACHQA=";
export type RecoverError =
	| { InvalidTokenId: Record<string, never> }
	| { InsufficientFunds: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { UnVerifiedIdentity: Record<string, never> }
	| { InCompliantTransfer: Record<string, never> }
	| { ComplianceError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { PausedToken: Record<string, never> }
	| { InvalidAmount: Record<string, never> }
	| { InvalidAddress: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const recoverErrorSchemaBase64 =
	"FQ4AAAAOAAAASW52YWxpZFRva2VuSWQCEQAAAEluc3VmZmljaWVudEZ1bmRzAgwAAABVbmF1dGhvcml6ZWQCCgAAAFBhcnNlRXJyb3ICCAAAAExvZ0Vycm9yAhIAAABVblZlcmlmaWVkSWRlbnRpdHkCEwAAAEluQ29tcGxpYW50VHJhbnNmZXICDwAAAENvbXBsaWFuY2VFcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICCwAAAFBhdXNlZFRva2VuAg0AAABJbnZhbGlkQW1vdW50Ag4AAABJbnZhbGlkQWRkcmVzcwISAAAAQWdlbnRBbHJlYWR5RXhpc3RzAg0AAABBZ2VudE5vdEZvdW5kAg==";
export type RecoverRequest = {
	lost_account:
		| { Account: [string] }
		| { Contract: [{ index: number; subindex: number }] };
	new_account:
		| { Account: [string] }
		| { Contract: [{ index: number; subindex: number }] };
};
export const recoverRequestSchemaBase64 =
	"FAACAAAADAAAAGxvc3RfYWNjb3VudBUCAAAABwAAAEFjY291bnQBAQAAAAsIAAAAQ29udHJhY3QBAQAAAAwLAAAAbmV3X2FjY291bnQVAgAAAAcAAABBY2NvdW50AQEAAAALCAAAAENvbnRyYWN0AQEAAAAM";
export type RecoveryAddressError =
	| { InvalidTokenId: Record<string, never> }
	| { InsufficientFunds: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { UnVerifiedIdentity: Record<string, never> }
	| { InCompliantTransfer: Record<string, never> }
	| { ComplianceError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { PausedToken: Record<string, never> }
	| { InvalidAmount: Record<string, never> }
	| { InvalidAddress: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const recoveryAddressErrorSchemaBase64 =
	"FQ4AAAAOAAAASW52YWxpZFRva2VuSWQCEQAAAEluc3VmZmljaWVudEZ1bmRzAgwAAABVbmF1dGhvcml6ZWQCCgAAAFBhcnNlRXJyb3ICCAAAAExvZ0Vycm9yAhIAAABVblZlcmlmaWVkSWRlbnRpdHkCEwAAAEluQ29tcGxpYW50VHJhbnNmZXICDwAAAENvbXBsaWFuY2VFcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICCwAAAFBhdXNlZFRva2VuAg0AAABJbnZhbGlkQW1vdW50Ag4AAABJbnZhbGlkQWRkcmVzcwISAAAAQWdlbnRBbHJlYWR5RXhpc3RzAg0AAABBZ2VudE5vdEZvdW5kAg==";
export type RecoveryAddressRequest =
	| { Account: [string] }
	| { Contract: [{ index: number; subindex: number }] };
export const recoveryAddressRequestSchemaBase64 =
	"FQIAAAAHAAAAQWNjb3VudAEBAAAACwgAAABDb250cmFjdAEBAAAADA==";
export type RecoveryAddressResponse =
	| { None: Record<string, never> }
	| {
			Some: [
				| { Account: [string] }
				| { Contract: [{ index: number; subindex: number }] },
			];
	  };
export const recoveryAddressResponseSchemaBase64 =
	"FQIAAAAEAAAATm9uZQIEAAAAU29tZQEBAAAAFQIAAAAHAAAAQWNjb3VudAEBAAAACwgAAABDb250cmFjdAEBAAAADA==";
export type RemoveAgentError =
	| { InvalidTokenId: Record<string, never> }
	| { InsufficientFunds: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { UnVerifiedIdentity: Record<string, never> }
	| { InCompliantTransfer: Record<string, never> }
	| { ComplianceError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { PausedToken: Record<string, never> }
	| { InvalidAmount: Record<string, never> }
	| { InvalidAddress: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const removeAgentErrorSchemaBase64 =
	"FQ4AAAAOAAAASW52YWxpZFRva2VuSWQCEQAAAEluc3VmZmljaWVudEZ1bmRzAgwAAABVbmF1dGhvcml6ZWQCCgAAAFBhcnNlRXJyb3ICCAAAAExvZ0Vycm9yAhIAAABVblZlcmlmaWVkSWRlbnRpdHkCEwAAAEluQ29tcGxpYW50VHJhbnNmZXICDwAAAENvbXBsaWFuY2VFcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICCwAAAFBhdXNlZFRva2VuAg0AAABJbnZhbGlkQW1vdW50Ag4AAABJbnZhbGlkQWRkcmVzcwISAAAAQWdlbnRBbHJlYWR5RXhpc3RzAg0AAABBZ2VudE5vdEZvdW5kAg==";
export type RemoveAgentRequest =
	| { Account: [string] }
	| { Contract: [{ index: number; subindex: number }] };
export const removeAgentRequestSchemaBase64 =
	"FQIAAAAHAAAAQWNjb3VudAEBAAAACwgAAABDb250cmFjdAEBAAAADA==";
export type SetComplianceError =
	| { InvalidTokenId: Record<string, never> }
	| { InsufficientFunds: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { UnVerifiedIdentity: Record<string, never> }
	| { InCompliantTransfer: Record<string, never> }
	| { ComplianceError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { PausedToken: Record<string, never> }
	| { InvalidAmount: Record<string, never> }
	| { InvalidAddress: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const setComplianceErrorSchemaBase64 =
	"FQ4AAAAOAAAASW52YWxpZFRva2VuSWQCEQAAAEluc3VmZmljaWVudEZ1bmRzAgwAAABVbmF1dGhvcml6ZWQCCgAAAFBhcnNlRXJyb3ICCAAAAExvZ0Vycm9yAhIAAABVblZlcmlmaWVkSWRlbnRpdHkCEwAAAEluQ29tcGxpYW50VHJhbnNmZXICDwAAAENvbXBsaWFuY2VFcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICCwAAAFBhdXNlZFRva2VuAg0AAABJbnZhbGlkQW1vdW50Ag4AAABJbnZhbGlkQWRkcmVzcwISAAAAQWdlbnRBbHJlYWR5RXhpc3RzAg0AAABBZ2VudE5vdEZvdW5kAg==";
export type SetComplianceRequest = { index: number; subindex: number };
export const setComplianceRequestSchemaBase64 = "DA==";
export type SetIdentityRegistryError =
	| { InvalidTokenId: Record<string, never> }
	| { InsufficientFunds: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { UnVerifiedIdentity: Record<string, never> }
	| { InCompliantTransfer: Record<string, never> }
	| { ComplianceError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { PausedToken: Record<string, never> }
	| { InvalidAmount: Record<string, never> }
	| { InvalidAddress: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const setIdentityRegistryErrorSchemaBase64 =
	"FQ4AAAAOAAAASW52YWxpZFRva2VuSWQCEQAAAEluc3VmZmljaWVudEZ1bmRzAgwAAABVbmF1dGhvcml6ZWQCCgAAAFBhcnNlRXJyb3ICCAAAAExvZ0Vycm9yAhIAAABVblZlcmlmaWVkSWRlbnRpdHkCEwAAAEluQ29tcGxpYW50VHJhbnNmZXICDwAAAENvbXBsaWFuY2VFcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICCwAAAFBhdXNlZFRva2VuAg0AAABJbnZhbGlkQW1vdW50Ag4AAABJbnZhbGlkQWRkcmVzcwISAAAAQWdlbnRBbHJlYWR5RXhpc3RzAg0AAABBZ2VudE5vdEZvdW5kAg==";
export type SetIdentityRegistryRequest = { index: number; subindex: number };
export const setIdentityRegistryRequestSchemaBase64 = "DA==";
export type SupportsError =
	| { InvalidTokenId: Record<string, never> }
	| { InsufficientFunds: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { UnVerifiedIdentity: Record<string, never> }
	| { InCompliantTransfer: Record<string, never> }
	| { ComplianceError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { PausedToken: Record<string, never> }
	| { InvalidAmount: Record<string, never> }
	| { InvalidAddress: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const supportsErrorSchemaBase64 =
	"FQ4AAAAOAAAASW52YWxpZFRva2VuSWQCEQAAAEluc3VmZmljaWVudEZ1bmRzAgwAAABVbmF1dGhvcml6ZWQCCgAAAFBhcnNlRXJyb3ICCAAAAExvZ0Vycm9yAhIAAABVblZlcmlmaWVkSWRlbnRpdHkCEwAAAEluQ29tcGxpYW50VHJhbnNmZXICDwAAAENvbXBsaWFuY2VFcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICCwAAAFBhdXNlZFRva2VuAg0AAABJbnZhbGlkQW1vdW50Ag4AAABJbnZhbGlkQWRkcmVzcwISAAAAQWdlbnRBbHJlYWR5RXhpc3RzAg0AAABBZ2VudE5vdEZvdW5kAg==";
export type SupportsRequest = Array<string>;
export const supportsRequestSchemaBase64 = "EAEWAA==";
export type SupportsResponse = Array<
	| { NoSupport: Record<string, never> }
	| { Support: Record<string, never> }
	| { SupportBy: [Array<{ index: number; subindex: number }>] }
>;
export const supportsResponseSchemaBase64 =
	"EAEVAwAAAAkAAABOb1N1cHBvcnQCBwAAAFN1cHBvcnQCCQAAAFN1cHBvcnRCeQEBAAAAEAAM";
export type TokenMetadataError =
	| { InvalidTokenId: Record<string, never> }
	| { InsufficientFunds: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { UnVerifiedIdentity: Record<string, never> }
	| { InCompliantTransfer: Record<string, never> }
	| { ComplianceError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { PausedToken: Record<string, never> }
	| { InvalidAmount: Record<string, never> }
	| { InvalidAddress: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const tokenMetadataErrorSchemaBase64 =
	"FQ4AAAAOAAAASW52YWxpZFRva2VuSWQCEQAAAEluc3VmZmljaWVudEZ1bmRzAgwAAABVbmF1dGhvcml6ZWQCCgAAAFBhcnNlRXJyb3ICCAAAAExvZ0Vycm9yAhIAAABVblZlcmlmaWVkSWRlbnRpdHkCEwAAAEluQ29tcGxpYW50VHJhbnNmZXICDwAAAENvbXBsaWFuY2VFcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICCwAAAFBhdXNlZFRva2VuAg0AAABJbnZhbGlkQW1vdW50Ag4AAABJbnZhbGlkQWRkcmVzcwISAAAAQWdlbnRBbHJlYWR5RXhpc3RzAg0AAABBZ2VudE5vdEZvdW5kAg==";
export type TokenMetadataRequest = Array<string>;
export const tokenMetadataRequestSchemaBase64 = "EAEdAA==";
export type TokenMetadataResponse = Array<{
	url: string;
	hash: { None: Record<string, never> } | { Some: [string] };
}>;
export const tokenMetadataResponseSchemaBase64 =
	"EAEUAAIAAAADAAAAdXJsFgEEAAAAaGFzaBUCAAAABAAAAE5vbmUCBAAAAFNvbWUBAQAAAB4gAAAA";
export type TransferError =
	| { InvalidTokenId: Record<string, never> }
	| { InsufficientFunds: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { UnVerifiedIdentity: Record<string, never> }
	| { InCompliantTransfer: Record<string, never> }
	| { ComplianceError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { PausedToken: Record<string, never> }
	| { InvalidAmount: Record<string, never> }
	| { InvalidAddress: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const transferErrorSchemaBase64 =
	"FQ4AAAAOAAAASW52YWxpZFRva2VuSWQCEQAAAEluc3VmZmljaWVudEZ1bmRzAgwAAABVbmF1dGhvcml6ZWQCCgAAAFBhcnNlRXJyb3ICCAAAAExvZ0Vycm9yAhIAAABVblZlcmlmaWVkSWRlbnRpdHkCEwAAAEluQ29tcGxpYW50VHJhbnNmZXICDwAAAENvbXBsaWFuY2VFcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICCwAAAFBhdXNlZFRva2VuAg0AAABJbnZhbGlkQW1vdW50Ag4AAABJbnZhbGlkQWRkcmVzcwISAAAAQWdlbnRBbHJlYWR5RXhpc3RzAg0AAABBZ2VudE5vdEZvdW5kAg==";
export type TransferRequest = Array<{
	token_id: string;
	amount: string;
	from:
		| { Account: [string] }
		| { Contract: [{ index: number; subindex: number }] };
	to:
		| { Account: [string] }
		| { Contract: [{ index: number; subindex: number }, string] };
	data: string;
}>;
export const transferRequestSchemaBase64 =
	"EAEUAAUAAAAIAAAAdG9rZW5faWQdAAYAAABhbW91bnQbJQAAAAQAAABmcm9tFQIAAAAHAAAAQWNjb3VudAEBAAAACwgAAABDb250cmFjdAEBAAAADAIAAAB0bxUCAAAABwAAAEFjY291bnQBAQAAAAsIAAAAQ29udHJhY3QBAgAAAAwWAQQAAABkYXRhHQE=";
export type UnFreezeError =
	| { InvalidTokenId: Record<string, never> }
	| { InsufficientFunds: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { UnVerifiedIdentity: Record<string, never> }
	| { InCompliantTransfer: Record<string, never> }
	| { ComplianceError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { PausedToken: Record<string, never> }
	| { InvalidAmount: Record<string, never> }
	| { InvalidAddress: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const unFreezeErrorSchemaBase64 =
	"FQ4AAAAOAAAASW52YWxpZFRva2VuSWQCEQAAAEluc3VmZmljaWVudEZ1bmRzAgwAAABVbmF1dGhvcml6ZWQCCgAAAFBhcnNlRXJyb3ICCAAAAExvZ0Vycm9yAhIAAABVblZlcmlmaWVkSWRlbnRpdHkCEwAAAEluQ29tcGxpYW50VHJhbnNmZXICDwAAAENvbXBsaWFuY2VFcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICCwAAAFBhdXNlZFRva2VuAg0AAABJbnZhbGlkQW1vdW50Ag4AAABJbnZhbGlkQWRkcmVzcwISAAAAQWdlbnRBbHJlYWR5RXhpc3RzAg0AAABBZ2VudE5vdEZvdW5kAg==";
export type UnFreezeRequest = {
	owner:
		| { Account: [string] }
		| { Contract: [{ index: number; subindex: number }] };
	tokens: Array<{ token_id: string; token_amount: string }>;
};
export const unFreezeRequestSchemaBase64 =
	"FAACAAAABQAAAG93bmVyFQIAAAAHAAAAQWNjb3VudAEBAAAACwgAAABDb250cmFjdAEBAAAADAYAAAB0b2tlbnMQAhQAAgAAAAgAAAB0b2tlbl9pZB0ADAAAAHRva2VuX2Ftb3VudBslAAAA";
export type UnPauseError =
	| { InvalidTokenId: Record<string, never> }
	| { InsufficientFunds: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { UnVerifiedIdentity: Record<string, never> }
	| { InCompliantTransfer: Record<string, never> }
	| { ComplianceError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { PausedToken: Record<string, never> }
	| { InvalidAmount: Record<string, never> }
	| { InvalidAddress: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const unPauseErrorSchemaBase64 =
	"FQ4AAAAOAAAASW52YWxpZFRva2VuSWQCEQAAAEluc3VmZmljaWVudEZ1bmRzAgwAAABVbmF1dGhvcml6ZWQCCgAAAFBhcnNlRXJyb3ICCAAAAExvZ0Vycm9yAhIAAABVblZlcmlmaWVkSWRlbnRpdHkCEwAAAEluQ29tcGxpYW50VHJhbnNmZXICDwAAAENvbXBsaWFuY2VFcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICCwAAAFBhdXNlZFRva2VuAg0AAABJbnZhbGlkQW1vdW50Ag4AAABJbnZhbGlkQWRkcmVzcwISAAAAQWdlbnRBbHJlYWR5RXhpc3RzAg0AAABBZ2VudE5vdEZvdW5kAg==";
export type UnPauseRequest = { tokens: Array<string> };
export const unPauseRequestSchemaBase64 = "FAABAAAABgAAAHRva2VucxACHQA=";
export type UpdateOperatorError =
	| { InvalidTokenId: Record<string, never> }
	| { InsufficientFunds: Record<string, never> }
	| { Unauthorized: Record<string, never> }
	| { ParseError: Record<string, never> }
	| { LogError: Record<string, never> }
	| { UnVerifiedIdentity: Record<string, never> }
	| { InCompliantTransfer: Record<string, never> }
	| { ComplianceError: Record<string, never> }
	| { CallContractError: Record<string, never> }
	| { PausedToken: Record<string, never> }
	| { InvalidAmount: Record<string, never> }
	| { InvalidAddress: Record<string, never> }
	| { AgentAlreadyExists: Record<string, never> }
	| { AgentNotFound: Record<string, never> };
export const updateOperatorErrorSchemaBase64 =
	"FQ4AAAAOAAAASW52YWxpZFRva2VuSWQCEQAAAEluc3VmZmljaWVudEZ1bmRzAgwAAABVbmF1dGhvcml6ZWQCCgAAAFBhcnNlRXJyb3ICCAAAAExvZ0Vycm9yAhIAAABVblZlcmlmaWVkSWRlbnRpdHkCEwAAAEluQ29tcGxpYW50VHJhbnNmZXICDwAAAENvbXBsaWFuY2VFcnJvcgIRAAAAQ2FsbENvbnRyYWN0RXJyb3ICCwAAAFBhdXNlZFRva2VuAg0AAABJbnZhbGlkQW1vdW50Ag4AAABJbnZhbGlkQWRkcmVzcwISAAAAQWdlbnRBbHJlYWR5RXhpc3RzAg0AAABBZ2VudE5vdEZvdW5kAg==";
export type UpdateOperatorRequest = Array<{
	update: { Remove: Record<string, never> } | { Add: Record<string, never> };
	operator:
		| { Account: [string] }
		| { Contract: [{ index: number; subindex: number }] };
}>;
export const updateOperatorRequestSchemaBase64 =
	"EAEUAAIAAAAGAAAAdXBkYXRlFQIAAAAGAAAAUmVtb3ZlAgMAAABBZGQCCAAAAG9wZXJhdG9yFQIAAAAHAAAAQWNjb3VudAEBAAAACwgAAABDb250cmFjdAEBAAAADA==";
export type event =
	| {
			Recovered: [
				{
					lost_account:
						| { Account: [string] }
						| { Contract: [{ index: number; subindex: number }] };
					new_account:
						| { Account: [string] }
						| { Contract: [{ index: number; subindex: number }] };
				},
			];
	  }
	| { IdentityRegistryAdded: [{ index: number; subindex: number }] }
	| { ComplianceAdded: [{ index: number; subindex: number }] }
	| { UnPaused: [{ token_id: string }] }
	| { Paused: [{ token_id: string }] }
	| {
			TokenFrozen: [
				{
					token_id: string;
					amount: string;
					address:
						| { Account: [string] }
						| { Contract: [{ index: number; subindex: number }] };
				},
			];
	  }
	| {
			TokenUnFrozen: [
				{
					token_id: string;
					amount: string;
					address:
						| { Account: [string] }
						| { Contract: [{ index: number; subindex: number }] };
				},
			];
	  }
	| {
			AgentRemoved: [
				{
					agent:
						| { Account: [string] }
						| { Contract: [{ index: number; subindex: number }] };
				},
			];
	  }
	| {
			AgentAdded: [
				{
					agent:
						| { Account: [string] }
						| { Contract: [{ index: number; subindex: number }] };
				},
			];
	  }
	| {
			TokenMetadata: {
				token_id: string;
				metadata_url: {
					url: string;
					hash: { None: Record<string, never> } | { Some: [string] };
				};
			};
	  }
	| {
			UpdateOperator: {
				update:
					| { Remove: Record<string, never> }
					| { Add: Record<string, never> };
				owner:
					| { Account: [string] }
					| { Contract: [{ index: number; subindex: number }] };
				operator:
					| { Account: [string] }
					| { Contract: [{ index: number; subindex: number }] };
			};
	  }
	| {
			Burn: {
				token_id: string;
				amount: string;
				owner:
					| { Account: [string] }
					| { Contract: [{ index: number; subindex: number }] };
			};
	  }
	| {
			Mint: {
				token_id: string;
				amount: string;
				owner:
					| { Account: [string] }
					| { Contract: [{ index: number; subindex: number }] };
			};
	  }
	| {
			Transfer: {
				token_id: string;
				amount: string;
				from:
					| { Account: [string] }
					| { Contract: [{ index: number; subindex: number }] };
				to:
					| { Account: [string] }
					| { Contract: [{ index: number; subindex: number }] };
			};
	  };
export const eventSchemaBase64 =
	"Hw4AAADyCQAAAFJlY292ZXJlZAEBAAAAFAACAAAADAAAAGxvc3RfYWNjb3VudBUCAAAABwAAAEFjY291bnQBAQAAAAsIAAAAQ29udHJhY3QBAQAAAAwLAAAAbmV3X2FjY291bnQVAgAAAAcAAABBY2NvdW50AQEAAAALCAAAAENvbnRyYWN0AQEAAAAM8xUAAABJZGVudGl0eVJlZ2lzdHJ5QWRkZWQBAQAAAAz0DwAAAENvbXBsaWFuY2VBZGRlZAEBAAAADPUIAAAAVW5QYXVzZWQBAQAAABQAAQAAAAgAAAB0b2tlbl9pZB0A9gYAAABQYXVzZWQBAQAAABQAAQAAAAgAAAB0b2tlbl9pZB0A9wsAAABUb2tlbkZyb3plbgEBAAAAFAADAAAACAAAAHRva2VuX2lkHQAGAAAAYW1vdW50GyUAAAAHAAAAYWRkcmVzcxUCAAAABwAAAEFjY291bnQBAQAAAAsIAAAAQ29udHJhY3QBAQAAAAz4DQAAAFRva2VuVW5Gcm96ZW4BAQAAABQAAwAAAAgAAAB0b2tlbl9pZB0ABgAAAGFtb3VudBslAAAABwAAAGFkZHJlc3MVAgAAAAcAAABBY2NvdW50AQEAAAALCAAAAENvbnRyYWN0AQEAAAAM+QwAAABBZ2VudFJlbW92ZWQBAQAAABQAAQAAAAUAAABhZ2VudBUCAAAABwAAAEFjY291bnQBAQAAAAsIAAAAQ29udHJhY3QBAQAAAAz6CgAAAEFnZW50QWRkZWQBAQAAABQAAQAAAAUAAABhZ2VudBUCAAAABwAAAEFjY291bnQBAQAAAAsIAAAAQ29udHJhY3QBAQAAAAz7DQAAAFRva2VuTWV0YWRhdGEAAgAAAAgAAAB0b2tlbl9pZB0ADAAAAG1ldGFkYXRhX3VybBQAAgAAAAMAAAB1cmwWAQQAAABoYXNoFQIAAAAEAAAATm9uZQIEAAAAU29tZQEBAAAAHiAAAAD8DgAAAFVwZGF0ZU9wZXJhdG9yAAMAAAAGAAAAdXBkYXRlFQIAAAAGAAAAUmVtb3ZlAgMAAABBZGQCBQAAAG93bmVyFQIAAAAHAAAAQWNjb3VudAEBAAAACwgAAABDb250cmFjdAEBAAAADAgAAABvcGVyYXRvchUCAAAABwAAAEFjY291bnQBAQAAAAsIAAAAQ29udHJhY3QBAQAAAAz9BAAAAEJ1cm4AAwAAAAgAAAB0b2tlbl9pZB0ABgAAAGFtb3VudBslAAAABQAAAG93bmVyFQIAAAAHAAAAQWNjb3VudAEBAAAACwgAAABDb250cmFjdAEBAAAADP4EAAAATWludAADAAAACAAAAHRva2VuX2lkHQAGAAAAYW1vdW50GyUAAAAFAAAAb3duZXIVAgAAAAcAAABBY2NvdW50AQEAAAALCAAAAENvbnRyYWN0AQEAAAAM/wgAAABUcmFuc2ZlcgAEAAAACAAAAHRva2VuX2lkHQAGAAAAYW1vdW50GyUAAAAEAAAAZnJvbRUCAAAABwAAAEFjY291bnQBAQAAAAsIAAAAQ29udHJhY3QBAQAAAAwCAAAAdG8VAgAAAAcAAABBY2NvdW50AQEAAAALCAAAAENvbnRyYWN0AQEAAAAM";
export const ENTRYPOINTS: Record<string, EntrypointName.Type> = {
	addAgent: EntrypointName.fromString("addAgent"),
	agents: EntrypointName.fromString("agents"),
	balanceOf: EntrypointName.fromString("balanceOf"),
	balanceOfFrozen: EntrypointName.fromString("balanceOfFrozen"),
	balanceOfUnFrozen: EntrypointName.fromString("balanceOfUnFrozen"),
	burn: EntrypointName.fromString("burn"),
	compliance: EntrypointName.fromString("compliance"),
	forcedTransfer: EntrypointName.fromString("forcedTransfer"),
	freeze: EntrypointName.fromString("freeze"),
	identityRegistry: EntrypointName.fromString("identityRegistry"),
	isAgent: EntrypointName.fromString("isAgent"),
	isPaused: EntrypointName.fromString("isPaused"),
	mint: EntrypointName.fromString("mint"),
	operatorOf: EntrypointName.fromString("operatorOf"),
	pause: EntrypointName.fromString("pause"),
	recover: EntrypointName.fromString("recover"),
	recoveryAddress: EntrypointName.fromString("recoveryAddress"),
	removeAgent: EntrypointName.fromString("removeAgent"),
	setCompliance: EntrypointName.fromString("setCompliance"),
	setIdentityRegistry: EntrypointName.fromString("setIdentityRegistry"),
	supports: EntrypointName.fromString("supports"),
	tokenMetadata: EntrypointName.fromString("tokenMetadata"),
	transfer: EntrypointName.fromString("transfer"),
	unFreeze: EntrypointName.fromString("unFreeze"),
	unPause: EntrypointName.fromString("unPause"),
	updateOperator: EntrypointName.fromString("updateOperator"),
};
export const ENTRYPOINT_DISPLAY_NAMES: Record<string, string> = {
	addAgent: "Add Agent",
	agents: "Agents",
	balanceOf: "Balance Of",
	balanceOfFrozen: "Balance Of Frozen",
	balanceOfUnFrozen: "Balance Of Un Frozen",
	burn: "Burn",
	compliance: "Compliance",
	forcedTransfer: "Forced Transfer",
	freeze: "Freeze",
	identityRegistry: "Identity Registry",
	isAgent: "Is Agent",
	isPaused: "Is Paused",
	mint: "Mint",
	operatorOf: "Operator Of",
	pause: "Pause",
	recover: "Recover",
	recoveryAddress: "Recovery Address",
	removeAgent: "Remove Agent",
	setCompliance: "Set Compliance",
	setIdentityRegistry: "Set Identity Registry",
	supports: "Supports",
	tokenMetadata: "Token Metadata",
	transfer: "Transfer",
	unFreeze: "Un Freeze",
	unPause: "Un Pause",
	updateOperator: "Update Operator",
};
export const rwaSecurityNft = {
	init: new InitMethod<initRequest>(
		ModuleReference.fromHexString(
			"4891e4463617bfa259f8a4c8f9835dd8e326491f0591f1ab89c09a9f8a836695",
		),
		ContractName.fromString("rwa_security_nft"),
		initRequestSchemaBase64,
	),
	addAgent: new ReceiveMethod<AddAgentRequest, never, AddAgentError>(
		ContractName.fromString("rwa_security_nft"),
		EntrypointName.fromString("addAgent"),
		addAgentRequestSchemaBase64,
		undefined,
		addAgentErrorSchemaBase64,
	),
	agents: new ReceiveMethod<never, AgentsResponse, AgentsError>(
		ContractName.fromString("rwa_security_nft"),
		EntrypointName.fromString("agents"),
		undefined,
		agentsResponseSchemaBase64,
		agentsErrorSchemaBase64,
	),
	balanceOf: new ReceiveMethod<
		BalanceOfRequest,
		BalanceOfResponse,
		BalanceOfError
	>(
		ContractName.fromString("rwa_security_nft"),
		EntrypointName.fromString("balanceOf"),
		balanceOfRequestSchemaBase64,
		balanceOfResponseSchemaBase64,
		balanceOfErrorSchemaBase64,
	),
	balanceOfFrozen: new ReceiveMethod<
		BalanceOfFrozenRequest,
		BalanceOfFrozenResponse,
		BalanceOfFrozenError
	>(
		ContractName.fromString("rwa_security_nft"),
		EntrypointName.fromString("balanceOfFrozen"),
		balanceOfFrozenRequestSchemaBase64,
		balanceOfFrozenResponseSchemaBase64,
		balanceOfFrozenErrorSchemaBase64,
	),
	balanceOfUnFrozen: new ReceiveMethod<
		BalanceOfUnFrozenRequest,
		BalanceOfUnFrozenResponse,
		BalanceOfUnFrozenError
	>(
		ContractName.fromString("rwa_security_nft"),
		EntrypointName.fromString("balanceOfUnFrozen"),
		balanceOfUnFrozenRequestSchemaBase64,
		balanceOfUnFrozenResponseSchemaBase64,
		balanceOfUnFrozenErrorSchemaBase64,
	),
	burn: new ReceiveMethod<BurnRequest, never, BurnError>(
		ContractName.fromString("rwa_security_nft"),
		EntrypointName.fromString("burn"),
		burnRequestSchemaBase64,
		undefined,
		burnErrorSchemaBase64,
	),
	compliance: new ReceiveMethod<void, ComplianceResponse>(
		ContractName.fromString("rwa_security_nft"),
		EntrypointName.fromString("compliance"),
		undefined,
		complianceResponseSchemaBase64,
	),
	forcedTransfer: new ReceiveMethod<
		ForcedTransferRequest,
		never,
		ForcedTransferError
	>(
		ContractName.fromString("rwa_security_nft"),
		EntrypointName.fromString("forcedTransfer"),
		forcedTransferRequestSchemaBase64,
		undefined,
		forcedTransferErrorSchemaBase64,
	),
	freeze: new ReceiveMethod<FreezeRequest, never, FreezeError>(
		ContractName.fromString("rwa_security_nft"),
		EntrypointName.fromString("freeze"),
		freezeRequestSchemaBase64,
		undefined,
		freezeErrorSchemaBase64,
	),
	identityRegistry: new ReceiveMethod<void, IdentityRegistryResponse>(
		ContractName.fromString("rwa_security_nft"),
		EntrypointName.fromString("identityRegistry"),
		undefined,
		identityRegistryResponseSchemaBase64,
	),
	isAgent: new ReceiveMethod<IsAgentRequest, IsAgentResponse, IsAgentError>(
		ContractName.fromString("rwa_security_nft"),
		EntrypointName.fromString("isAgent"),
		isAgentRequestSchemaBase64,
		isAgentResponseSchemaBase64,
		isAgentErrorSchemaBase64,
	),
	isPaused: new ReceiveMethod<IsPausedRequest, IsPausedResponse, IsPausedError>(
		ContractName.fromString("rwa_security_nft"),
		EntrypointName.fromString("isPaused"),
		isPausedRequestSchemaBase64,
		isPausedResponseSchemaBase64,
		isPausedErrorSchemaBase64,
	),
	mint: new ReceiveMethod<MintRequest, never, MintError>(
		ContractName.fromString("rwa_security_nft"),
		EntrypointName.fromString("mint"),
		mintRequestSchemaBase64,
		undefined,
		mintErrorSchemaBase64,
	),
	operatorOf: new ReceiveMethod<
		OperatorOfRequest,
		OperatorOfResponse,
		OperatorOfError
	>(
		ContractName.fromString("rwa_security_nft"),
		EntrypointName.fromString("operatorOf"),
		operatorOfRequestSchemaBase64,
		operatorOfResponseSchemaBase64,
		operatorOfErrorSchemaBase64,
	),
	pause: new ReceiveMethod<PauseRequest, never, PauseError>(
		ContractName.fromString("rwa_security_nft"),
		EntrypointName.fromString("pause"),
		pauseRequestSchemaBase64,
		undefined,
		pauseErrorSchemaBase64,
	),
	recover: new ReceiveMethod<RecoverRequest, never, RecoverError>(
		ContractName.fromString("rwa_security_nft"),
		EntrypointName.fromString("recover"),
		recoverRequestSchemaBase64,
		undefined,
		recoverErrorSchemaBase64,
	),
	recoveryAddress: new ReceiveMethod<
		RecoveryAddressRequest,
		RecoveryAddressResponse,
		RecoveryAddressError
	>(
		ContractName.fromString("rwa_security_nft"),
		EntrypointName.fromString("recoveryAddress"),
		recoveryAddressRequestSchemaBase64,
		recoveryAddressResponseSchemaBase64,
		recoveryAddressErrorSchemaBase64,
	),
	removeAgent: new ReceiveMethod<RemoveAgentRequest, never, RemoveAgentError>(
		ContractName.fromString("rwa_security_nft"),
		EntrypointName.fromString("removeAgent"),
		removeAgentRequestSchemaBase64,
		undefined,
		removeAgentErrorSchemaBase64,
	),
	setCompliance: new ReceiveMethod<
		SetComplianceRequest,
		never,
		SetComplianceError
	>(
		ContractName.fromString("rwa_security_nft"),
		EntrypointName.fromString("setCompliance"),
		setComplianceRequestSchemaBase64,
		undefined,
		setComplianceErrorSchemaBase64,
	),
	setIdentityRegistry: new ReceiveMethod<
		SetIdentityRegistryRequest,
		never,
		SetIdentityRegistryError
	>(
		ContractName.fromString("rwa_security_nft"),
		EntrypointName.fromString("setIdentityRegistry"),
		setIdentityRegistryRequestSchemaBase64,
		undefined,
		setIdentityRegistryErrorSchemaBase64,
	),
	supports: new ReceiveMethod<SupportsRequest, SupportsResponse, SupportsError>(
		ContractName.fromString("rwa_security_nft"),
		EntrypointName.fromString("supports"),
		supportsRequestSchemaBase64,
		supportsResponseSchemaBase64,
		supportsErrorSchemaBase64,
	),
	tokenMetadata: new ReceiveMethod<
		TokenMetadataRequest,
		TokenMetadataResponse,
		TokenMetadataError
	>(
		ContractName.fromString("rwa_security_nft"),
		EntrypointName.fromString("tokenMetadata"),
		tokenMetadataRequestSchemaBase64,
		tokenMetadataResponseSchemaBase64,
		tokenMetadataErrorSchemaBase64,
	),
	transfer: new ReceiveMethod<TransferRequest, never, TransferError>(
		ContractName.fromString("rwa_security_nft"),
		EntrypointName.fromString("transfer"),
		transferRequestSchemaBase64,
		undefined,
		transferErrorSchemaBase64,
	),
	unFreeze: new ReceiveMethod<UnFreezeRequest, never, UnFreezeError>(
		ContractName.fromString("rwa_security_nft"),
		EntrypointName.fromString("unFreeze"),
		unFreezeRequestSchemaBase64,
		undefined,
		unFreezeErrorSchemaBase64,
	),
	unPause: new ReceiveMethod<UnPauseRequest, never, UnPauseError>(
		ContractName.fromString("rwa_security_nft"),
		EntrypointName.fromString("unPause"),
		unPauseRequestSchemaBase64,
		undefined,
		unPauseErrorSchemaBase64,
	),
	updateOperator: new ReceiveMethod<
		UpdateOperatorRequest,
		never,
		UpdateOperatorError
	>(
		ContractName.fromString("rwa_security_nft"),
		EntrypointName.fromString("updateOperator"),
		updateOperatorRequestSchemaBase64,
		undefined,
		updateOperatorErrorSchemaBase64,
	),
	deserializeEvent: (event: ContractEvent.Type): event => {
		return ContractEvent.parseWithSchemaTypeBase64(
			event,
			eventSchemaBase64,
		) as event;
	},
};
export default rwaSecurityNft;
