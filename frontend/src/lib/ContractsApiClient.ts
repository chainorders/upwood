import { ContractsApi } from "./contracts-api-client";

const contractsApi = new ContractsApi({
	BASE: import.meta.env.VITE_CONTRACTS_API_URL!,
});
export default contractsApi;
