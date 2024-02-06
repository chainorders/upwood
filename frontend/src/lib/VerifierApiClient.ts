import { VerifierApi } from "./verifier-api-client";

export const verifierApi = new VerifierApi({
	BASE: import.meta.env.VITE_VERIFIER_API_URL!,
});
export default verifierApi;
