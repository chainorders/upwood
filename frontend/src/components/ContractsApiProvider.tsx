import { createContext, useContext } from "react";
import { ContractsApi } from "../lib/contracts-api-client";
import { contractsApi } from "../lib/ContractsApiClient";

export type ContractsApiContext = {
	provider: ContractsApi;
};
const ContractsApiContext = createContext<ContractsApiContext>({
	provider: contractsApi,
});

export const useContractsApi = () => {
	return useContext(ContractsApiContext);
};

export default function ContractsApiProvider({ children }: { children: React.ReactNode }): React.ReactNode {
	return (
		<ContractsApiContext.Provider
			value={{
				provider: contractsApi,
			}}>
			{children}
		</ContractsApiContext.Provider>
	);
}
