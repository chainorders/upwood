import { createContext, useContext } from "react";
import verifierApi from "../lib/VerifierApiClient";
import { VerifierApi } from "../lib/verifier-api-client";

export type VerifierApiContext = {
	provider: VerifierApi;
};
const VerifierApiContext = createContext<VerifierApiContext>({
	provider: verifierApi,
});

export const useVerifierApi = () => {
	return useContext(VerifierApiContext);
};

export default function VerifierApiProvider({
	children,
}: {
	children: React.ReactNode;
}): React.ReactNode {
	return (
		<VerifierApiContext.Provider
			value={{
				provider: verifierApi,
			}}
		>
			{children}
		</VerifierApiContext.Provider>
	);
}
