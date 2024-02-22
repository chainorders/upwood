import { sha256 } from "@concordium/web-sdk";

export type UrlJson = {
	url: string;
	hash?: string;
};
export type AttributeJson = {
	type: string;
	name: string;
	value: string;
};
export type TokenMetadata = {
	name?: string;
	description?: string;
	symbol?: string;
	unique?: boolean;
	decimals?: number;
	thumbnail?: UrlJson;
	display?: UrlJson;
	artifact?: UrlJson;
	assets?: TokenMetadata[];
	attributes?: AttributeJson[];
	localizations?: { [key: string]: UrlJson };
};

export const getTokenMetadata = (url: string): Promise<TokenMetadata> => {
	const httpUrl = toHttpUrl(url);
	return fetch(httpUrl, {
		cache: "force-cache",
	}).then((response) => response.json());
};

export const getTokenMetadataHash = (url: string): Promise<string> => {
	return fetch(toHttpUrl(url), {
		cache: "force-cache",
	})
		.then((response) => response.blob())
		.then((blob) => blob.arrayBuffer())
		.then((buffer) => {
			const uint8Array = new Uint8Array(buffer);
			return sha256([uint8Array]).toString("hex");
		});
};

export const toHttpUrl = (url: string): string => {
	if (url.startsWith("ipfs://")) {
		return `https://ipfs.io/ipfs/${url.slice(7)}`;
	} else if (url.startsWith("ipfs:")) {
		return `https://ipfs.io/ipfs/${url.slice(5)}`;
	} else if (url.startsWith("https://") || url.startsWith("http://")) {
		return url;
	}

	return url;
};

export const isValidUrl = (url: string): boolean => {
	if (!url) {
		return false;
	}

	return (
		url.startsWith("ipfs://") ||
		url.startsWith("ipfs:") ||
		url.startsWith("https://") ||
		url.startsWith("http://")
	);
};

export const toDataUrl = async (url: string): Promise<string> => {
	const httpUrl = toHttpUrl(url);
	return new Promise((resolve, reject) => {
		try {
			fetch(httpUrl, {
				cache: "force-cache",
			})
				.then((response) => response.blob())
				.then((blob) => {
					const reader = new FileReader();
					reader.onloadend = function () {
						resolve(reader.result as string);
					};
					reader.readAsDataURL(blob);
				});
		} catch (error) {
			reject(error);
		}
	});
};

export async function fetchJsonString(metadataUrl: string): Promise<string> {
	const res = await fetch(metadataUrl);

	if (!res.ok) {
		return Promise.reject(new Error("Could not load Metadata"));
	}

	return res.text();
}

export const tokenIdToArtifactFileName = (
	originalFileName: string,
	tokenId: string,
) => {
	const ext = originalFileName.substring(originalFileName.lastIndexOf("."));

	return `token_artifact_${tokenId}.${ext}`;
};
export const tokenIdToTokenMetadataFileName = (tokenId: string) => {
	return `token_${tokenId}_metadata.json`;
};
export const tokenIdToTokenImageFileName = (
	originalFileName: string,
	tokenId: string,
) => {
	const ext = originalFileName.substring(originalFileName.lastIndexOf("."));

	return `token_${tokenId}.${ext}`;
};
