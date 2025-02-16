import { useOutletContext } from "react-router";
import PageHeader from "../components/PageHeader";
import DownloadIcon from "../assets/Icon.svg";
import Button from "../components/Button";
import { User } from "../lib/user";

export default function ContractsDetails() {
	const { user } = useOutletContext<{ user: User }>();
	const data = {
		id: "1256",
		tag: "Subscription agreement Nicgales 29.06.2024",
		content:
			"<p>Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.</p><p>There are many variations of passages of Lorem Ipsum available, but the majority have suffered alteration in some form, by injected humour, or randomised words which don't look even slightly believable. If you are going to use a passage of Lorem Ipsum, you need to be sure there isn't anything embarrassing hidden in the middle of text. All the Lorem Ipsum generators on the Internet tend to repeat predefined chunks as necessary, making this the first true generator on the Internet. It uses a dictionary of over 200 Latin words, combined with a handful of model sentence structures, to generate Lorem Ipsum which looks reasonable. The generated Lorem Ipsum is therefore always free from repetition, injected humour, or non-characteristic words etc.</p><p>There are many variations of passages of Lorem Ipsum available, but the majority have suffered alteration in some form, by injected humour, or randomised words which don't look even slightly believable. If you are going to use a passage of Lorem Ipsum, you need to be sure there isn't anything embarrassing hidden in the middle of text. All the Lorem Ipsum generators on the Internet tend to repeat predefined chunks as necessary, making this the first true generator on the Internet. It uses a dictionary of over 200 Latin words, combined with a handful of model sentence structures, to generate Lorem Ipsum which looks reasonable. The generated Lorem Ipsum is therefore always free from repetition, injected humour, or non-characteristic words etc.</p>",
		title: "Nīcgale cadaster NR: 487637827292",
		line: "Subscription agreement",
		signed_date: "29.06.2024",
		tokens: "129",
		edoc_link: "",
		pdf_link: "",
	};

	return (
		<div className="contract">
			<div className="clr"></div>
			<PageHeader user={user} parts={[{ name: "Contracts" }]} />
			<div className="container">
				<div className="container-in">
					<div className="col-12">
						<div className="contract-card">
							<div className="tag">{data.tag}</div>
							<div className="content2" dangerouslySetInnerHTML={{ __html: data.content }}></div>
							<div className="name">{data.title}</div>
							<div className="line">{data.line}</div>
							<div className="fs">
								<div className="fl">
									Signed date : <span>{data.signed_date}</span>
								</div>
								<div className="fr">
									Tokens : <span>{data.tokens}</span>
								</div>
								<div className="clr"></div>
							</div>
							<div className="space-20"></div>
							<div className="clr"></div>
							<div className="col-3 col-m-half fl">
								<Button icon={DownloadIcon} icononright={true} style="style5" text={"PDF"} link={""} active={false} />
							</div>
							<div className="col-3 col-m-half fr">
								<Button icon={DownloadIcon} icononright={true} style="style5" text={"EDOC"} link={""} active={false} />
							</div>
							<div className="clr"></div>
						</div>
					</div>
					<div className="clr"></div>
				</div>
			</div>
		</div>
	);
}
