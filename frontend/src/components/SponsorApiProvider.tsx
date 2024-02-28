import { createContext, useContext } from "react";
import sponsorApi from "../lib/SponsorApiClient";
import { SponsorApi } from "../lib/sponsor-api-client";

export type SponsorApiContent = {
	provider: SponsorApi;
};
const context = createContext<SponsorApiContent>({
	provider: sponsorApi,
});

export const useSponsorApi = () => {
	return useContext(context);
};

export default function SponsorApiProvider({
	children,
}: {
	children: React.ReactNode;
}): React.ReactNode {
	return (
		<context.Provider
			value={{
				provider: sponsorApi,
			}}
		>
			{children}
		</context.Provider>
	);
}
