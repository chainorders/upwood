/**
 * Represents a URL JSON object
 */
interface UrlObject {
	url: string;
	mimeType?: string;
}

/**
 * Represents an attribute for a token
 */
interface TokenAttribute {
	type: string;
	name: string;
	value: string | number | boolean;
}

/**
 * Represents token metadata according to a standard CIS-2 format
 */
interface TokenMetadata {
	name?: string;
	symbol?: string;
	unique?: boolean;
	decimals?: number;
	description?: string;
	thumbnail?: UrlObject;
	display?: UrlObject;
	artifact?: UrlObject;
	assets?: TokenMetadata[];
	attributes?: TokenAttribute[];
}

export { TokenMetadata, TokenAttribute, UrlObject };
