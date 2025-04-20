import { useEffect, useState } from "react";
import { LegalContractUserModel } from "../apiClient";
import DownloadIcon from "../assets/Icon.svg";

import Button from "./Button";
interface ItemProps {
	contract: LegalContractUserModel;
}

export default function ContractsCard({ contract }: ItemProps) {
	const [htmlContent, setHtmlContent] = useState("");
	useEffect(() => {
		fetch(contract.text_url)
			.then((res) => res.text())
			.then((data) => setHtmlContent(data));
	}, [contract.text_url]);

	const formattedSignedDate = new Date(contract.signed_date).toLocaleDateString("en-US", {
		year: "numeric",
		month: "long",
		day: "numeric",
	});

	return (
		<>
			<div className="contract-card">
				<div className={`container`}>
					<div className="container-in">
						<div className="col-12">
							<div className="tag">
								{contract.tag}
								&nbsp;{formattedSignedDate}
							</div>
							<div className="content" dangerouslySetInnerHTML={{ __html: htmlContent }}></div>
							<div className="name">{contract.name}</div>
							<div className="line">{contract.tag}</div>
							<div className="fs">
								<div className="fl">
									Signed date : <span>{formattedSignedDate}</span>
								</div>
								<div className="fr">
									Tokens : <span>{contract.user_token_balance || "-"}</span>
								</div>
								<div className="clr"></div>
							</div>
						</div>
						<div className="space-20"></div>
						<div className="clr"></div>
						<div className="col-4 fl col-m-full col-mr-bottom-10">
							<Button text="SEE PREVIEW" link={`/contracts/${contract.project_id}`} linkState={{ legalContract: contract }} />
						</div>
						<div className="col-3 col-m-half fr">
							<Button icon={DownloadIcon} icononright style="style5" text="PDF" link={contract.pdf_url} linkTarget="_blank" />
						</div>
						<div className="col-3 col-m-half fr">
							<Button
								icon={DownloadIcon}
								icononright
								style="style5"
								text="EDOC"
								link={contract.edoc_url}
								linkTarget="_blank"
							/>
						</div>

						<div className="clr"></div>
					</div>
				</div>
			</div>
		</>
	);
}
