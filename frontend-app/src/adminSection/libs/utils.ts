import { FilesService } from "../../apiClient";
import { Buffer } from "buffer/";

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