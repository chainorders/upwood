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
	shares_available: number;
	property_media_header: string;
	property_media_footer: string;
	created_at: string;
	updated_at: string;
	offering_doc_title?: string;
	offering_doc_header?: string;
	offering_doc_img_url?: string;
	offering_doc_footer?: string;
	financial_projection_title?: string;
	financial_projection_header?: string;
	financial_projection_img_url?: string;
	financial_projection_footer?: string;
	geo_title?: string;
	geo_header?: string;
	geo_img_url?: string;
	geo_footer?: string;
};
