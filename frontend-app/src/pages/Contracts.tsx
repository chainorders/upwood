import { useOutletContext } from "react-router";
import PageHeader from "../components/PageHeader";
import ContractsCard from "../components/ContractsCard";
import { User } from "../lib/user";

export default function Contracts() {
	const { user } = useOutletContext<{ user: User }>();
	const data = [
		{
			id: "1256",
			tag: "Subscription agreement Nicgales 29.06.2024",
			content:
				"loprem ipsum<div>asdlkajsdlasjdalksdjalskdj</div><div><br></div><h1>asdlasdlkajsdlaksjdasd</h1><div><br></div><div><br></div><div><b>asdlkjasdlkjaslkajsd</b></div><div><br></div><div>asdlkajsdlkajsdlakjsd</div>",
			title: "Nīcgale cadaster NR: 487637827292",
			line: "Subscription agreement",
			signed_date: "29.06.2024",
			tokens: "129",
			edoc_link: "",
			pdf_link: "",
		},
		{
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
		},
	];

	return (
		<>
			<div className="clr"></div>
			<div className="contract">
				<PageHeader user={user} parts={[{ name: "Contracts" }]} />
				<div className="container">
					<div className="container-in">
						{data.map((item, index) => (
							<div className="col-6 col-m-full fl" key={index}>
								<ContractsCard item={item} />
							</div>
						))}
						<div className="clr"></div>
					</div>
				</div>
			</div>
		</>
	);
}
