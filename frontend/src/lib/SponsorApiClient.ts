import { SponsorApi } from "./sponsor-api-client";

export const sponsorApi = new SponsorApi({
	BASE: import.meta.env.VITE_SPONSOR_API_URL!,
});
export default sponsorApi;
