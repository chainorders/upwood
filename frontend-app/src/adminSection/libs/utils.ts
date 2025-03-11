import { FilesService } from "../../apiClient";
import { Buffer } from "buffer/";
import { TokenMetadata } from "./types";

export const adminUploadImage = async (fileBaseUrl: string, field: string, imageData?: string): Promise<string> => {
	if (!imageData) {
		return Promise.resolve("");
	}

	return new Promise<string>((resolve, reject) => {
		const buf = Buffer.from(imageData.replace(/^data:image\/\w+;base64,/, ""), "base64");
		FilesService.postAdminFilesS3UploadUrl()
			.then((res) =>
				fetch(res.presigned_url, { method: "PUT", body: buf, headers: { "Content-Type": "image/png" } }).then(
					() => `${fileBaseUrl}/${res.file_name}`,
				),
			)
			.then(resolve)
			.catch(reject);
	});
};

export const adminUploadJson = async (fileBaseUrl: string, field: string, jsonData?: string): Promise<string> => {
	if (!jsonData) {
		return Promise.resolve("");
	}

	return new Promise<string>((resolve, reject) => {
		const buf = Buffer.from(jsonData, "utf-8");
		FilesService.postAdminFilesS3UploadUrl()
			.then((res) =>
				fetch(res.presigned_url, { method: "PUT", body: buf, headers: { "Content-Type": "application/json" } }).then(
					() => `${fileBaseUrl}/${res.file_name}`,
				),
			)
			.then(resolve)
			.catch(reject);
	});
};

/**
 * Generates a SHA-256 hash of the metadata object
 * @param data The metadata object to hash
 * @returns The hexadecimal representation of the SHA-256 hash
 */
export const hashMetadata = async (data: TokenMetadata): Promise<string> => {
	// Convert the data to a JSON string
	const jsonString = JSON.stringify(data);

	// Convert the string to UTF-8 encoded bytes
	const encoder = new TextEncoder();
	const bytes = encoder.encode(jsonString);

	// Calculate SHA-256 hash using Web Crypto API
	const hashBuffer = await crypto.subtle.digest("SHA-256", bytes);

	// Convert the hash to a hexadecimal string
	const hashArray = Array.from(new Uint8Array(hashBuffer));
	const hashHex = hashArray.map((b) => b.toString(16).padStart(2, "0")).join("");

	return hashHex;
};
