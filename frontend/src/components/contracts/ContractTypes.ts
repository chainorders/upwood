import { ContractAddress } from "@concordium/web-sdk";
import { CONTRACT_NAME as rwaSecurityNftContractName } from "../../lib/rwaSecurityNft";
import { CONTRACT_NAME as rwaSecuritySftContractName } from "../../lib/rwaSecuritySft";
import { CONTRACT_NAME as rwaComplianceContractName } from "../../lib/rwaCompliance";
import { CONTRACT_NAME as rwaComplianceModuleContractName } from "../../lib/rwaComplianceModuleAllowedNationalities";
import { CONTRACT_NAME as rwaIdentityRegistryContractName } from "../../lib/rwaIdentityRegistry";
import { CONTRACT_NAME as rwaSponsorContractName } from "../../lib/rwaSponsor";
import { CONTRACT_NAME as rwaMarketContractName } from "../../lib/rwaMarket";

export const enum ContractType {
	RwaIdentityRegistry = `${rwaIdentityRegistryContractName}`,
	RwaCompliance = `${rwaComplianceContractName}`,
	RwaComplianceModule = `${rwaComplianceModuleContractName}`,
	RwaSecurityNft = `${rwaSecurityNftContractName}`,
	RwaSecuritySft = `${rwaSecuritySftContractName}`,
	RwaSponsor = `${rwaSponsorContractName}`,
	RwaMarket = `${rwaMarketContractName}`,
}

export interface Contract {
	name: string;
	address: ContractAddress.Type;
	type: ContractType;
}
