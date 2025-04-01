import { useState } from "react";
import { FilesService } from "../apiClient";
import Avatar from "react-avatar-edit";
import { Buffer } from "buffer/";

export default function ProfilePicture({
	picture,
	initials,
	fileBaseUrl,
	onDeleted,
	update,
	defaultState = "default",
}: {
	picture?: string;
	initials: string;
	fileBaseUrl: string;
	update: (fileUrl: string) => Promise<unknown>;
	onDeleted: () => Promise<unknown>;
	defaultState?: "default" | "upload";
}) {
	const [state, setState] = useState<"default" | "upload" | "uploading" | "deleting">(defaultState);
	const [inputPicture, setInputPicture] = useState(picture);
	const [imageData, setImageData] = useState<string>();
	const [_error, setError] = useState<string>();

	const changeProfilePicture = (imageData: string) => {
		const buf = Buffer.from(imageData.replace(/^data:image\/\w+;base64,/, ""), "base64");
		setState("uploading");
		FilesService.postFilesS3ProfilePictureUploadUrl()
			.then((res) =>
				fetch(res.presigned_url, { method: "PUT", body: buf, headers: { "Content-Type": "image/png" } }).then(
					() => `${fileBaseUrl}/${res.file_name}`,
				),
			)
			.then(async (fileUrl) => {
				await update(fileUrl);
				setInputPicture(fileUrl);
			})
			.then(() => {
				setState("default");
			})
			.catch((err) => {
				console.log(err);
				alert("Error uploading image");
				setError("Error uploading image");
				setState("default");
			});
	};

	const removeProfilePicture = () => {
		setState("deleting");
		onDeleted().then(() => {
			setState("default");
			setInputPicture("");
		});
	};

	return (
		<>
			<div style={{ display: "flex", justifyContent: "center", flexDirection: "column", alignItems: "center" }}>
				{
					{
						["default"]: inputPicture ? (
							<img src={inputPicture} alt="" className="Avatar" />
						) : (
							<div className="letter">{initials}</div>
						),
						["upload"]: (
							<Avatar
								width={150}
								height={150}
								imageWidth={88}
								onCrop={setImageData}
								cropRadius={44}
								label="upload"
								onClose={() => setImageData(undefined)}
								exportMimeType="image/png"
							/>
						),
						["uploading"]: <img src={imageData} alt="" className="Avatar uploading" />,
						["deleting"]: <img src={inputPicture} alt="" className="Avatar deleting" />,
					}[state]
				}
			</div>
			<div className="space-15"></div>
			<div className="links">
				{
					{
						["default"]: (
							<>
								<button className="reset-button link-button" onClick={() => setState("upload")}>
									CHANGE
								</button>
								<button
									className="reset-button link-button danger"
									onClick={() => removeProfilePicture()}
									disabled={!inputPicture}
								>
									DELETE
								</button>
							</>
						),
						["upload"]: (
							<>
								<button
									className="reset-button link-button"
									disabled={!imageData}
									onClick={() => changeProfilePicture(imageData!)}
								>
									SET
								</button>
								<button
									className="reset-button link-button danger"
									onClick={() => setState(defaultState)}
									disabled={state === defaultState}
								>
									CANCEL
								</button>
							</>
						),
						["uploading"]: (
							<>
								<button className="reset-button link-button" disabled>
									UPLOADING
								</button>
								<button className="reset-button link-button danger" disabled>
									CANCEL
								</button>
							</>
						),
						["deleting"]: (
							<>
								<button className="reset-button link-button" disabled>
									CHANGE
								</button>
								<button className="reset-button link-button danger" disabled>
									DELETING
								</button>
							</>
						),
					}[state]
				}
			</div>
		</>
	);
}
