/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { ForestProjectState } from "./ForestProjectState";

export type ForestProject = {
	id: string;
	name: string;
	label: string;
	desc_short: string;
	desc_long: string;
	area: string;
	carbon_credits: number;
	roi_percent: number;
	state: ForestProjectState;
	image_large_url: string;
	image_small_url: string;
	geo_spatial_url?: string;
	shares_available: number;
	offering_doc_link?: string;
	property_media_header: string;
	property_media_footer: string;
	created_at: string;
	updated_at: string;
};
