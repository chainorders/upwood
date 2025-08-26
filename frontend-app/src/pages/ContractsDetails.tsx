import { useEffect, useState } from "react";
import { useLocation, useParams } from "react-router";
import PageHeader from "../components/PageHeader";
import DownloadIcon from "../assets/Icon.svg";
import Button from "../components/Button";
import { User } from "../lib/user";
import { LegalContractUserModel, ForestProjectService } from "../apiClient";

interface ContractsDetailsProps {
	user: User;
}

export default function ContractsDetails({ user }: ContractsDetailsProps) {
	const { id } = useParams<{ id: string }>();
	const location = useLocation();
	const [contract, setContract] = useState<LegalContractUserModel>(location.state?.legalContract);
	const [htmlContent, setHtmlContent] = useState("");

	useEffect(() => {
		if (id) {
			ForestProjectService.getForestProjectsLegalContract(id).then(setContract);
		}
	}, [id]);

	useEffect(() => {
		if (contract?.text_url) {
			fetch(contract.text_url)
				.then((res) => res.text())
				.then((data) => setHtmlContent(data));
		}
	}, [contract?.text_url]);

	const formattedSignedDate = contract
		? new Date(contract.signed_date).toLocaleDateString("en-US", {
				year: "numeric",
				month: "long",
				day: "numeric",
			})
		: "";

	return (
		<div className="contract">
			<div className="clr"></div>
			<PageHeader user={user} parts={[{ name: "Contracts" }]} />
			<div className="container">
				<div className="container-in">
					<div className="col-12">
						{contract && (
							<div className="contract-card">
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
								<div className="space-20"></div>
								<div className="clr"></div>
								<div className="col-3 col-m-half fl">
									<Button
										icon={DownloadIcon}
										icononright={true}
										style="style5"
										text="PDF"
										link={contract.pdf_url}
										linkTarget="_blank"
									/>
								</div>
								<div className="col-3 col-m-half fr">
									<Button
										icon={DownloadIcon}
										icononright={true}
										style="style5"
										text="EDOC"
										link={contract.edoc_url}
										linkTarget="_blank"
									/>
								</div>
								<div className="clr"></div>
							</div>
						)}
					</div>
					<div className="clr"></div>
				</div>
			</div>
		</div>
	);
}
