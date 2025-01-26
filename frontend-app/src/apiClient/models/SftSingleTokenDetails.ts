/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { Agent } from "./Agent";
import type { ListenerContract } from "./ListenerContract";

export type SftSingleTokenDetails = {
	supply: string;
	holder_count: number;
	token_id: string;
	contract_agents: Array<Agent>;
	contract: ListenerContract;
};
