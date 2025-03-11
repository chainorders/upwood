import { Area } from "react-easy-crop";

/**
 * Creates a cropped version of an image using canvas
 */
export default async function getCroppedImg(
	imageSrc: string,
	pixelCrop: { x: number; y: number; width: number; height: number },
): Promise<string> {
	const image = await createImage(imageSrc);
	const canvas = document.createElement("canvas");
	const ctx = canvas.getContext("2d");

	if (!ctx) {
		throw new Error("Could not get canvas context");
	}

	// Set canvas dimensions to the cropped size
	canvas.width = pixelCrop.width;
	canvas.height = pixelCrop.height;

	// Draw the cropped image onto the canvas
	ctx.drawImage(
		image,
		pixelCrop.x,
		pixelCrop.y,
		pixelCrop.width,
		pixelCrop.height,
		0,
		0,
		pixelCrop.width,
		pixelCrop.height,
	);

	// Return as a base64 data URL
	return canvas.toDataURL("image/jpeg");
}

/**
 * Creates an Image object from a source URL and waits for it to load
 */
function createImage(url: string): Promise<HTMLImageElement> {
	return new Promise((resolve, reject) => {
		const image = new Image();
		image.addEventListener("load", () => resolve(image));
		image.addEventListener("error", (error) => reject(error));
		image.src = url;
	});
}
