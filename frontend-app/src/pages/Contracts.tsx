import { useEffect, useState } from "react";
import PageHeader from "../components/PageHeader";
import ContractsCard from "../components/ContractsCard";
import { User } from "../lib/user";
import { ForestProjectService, PagedResponse_LegalContractUserModel } from "../apiClient";
import { MAX_PAGE_SIZE } from "../lib/constants";

interface ContractsProps {
	user: User;
}

export default function Contracts({ user }: ContractsProps) {
	const [legalContracts, setLegalContracts] = useState<PagedResponse_LegalContractUserModel>();
	useEffect(() => {
		ForestProjectService.getForestProjectsLegalContractList(0, MAX_PAGE_SIZE).then(setLegalContracts);
	}, [user]);

	return (
		<>
			<div className="clr"></div>
			<div className="contract">
				<PageHeader user={user} parts={[{ name: "Contracts" }]} />
				<div className="container">
					<div className="container-in">
						{legalContracts?.data.map((item, index) => (
							<div className="col-6 col-m-full fl" key={index}>
								<ContractsCard contract={item} />
							</div>
						))}
						<div className="clr"></div>
					</div>
				</div>
			</div>
		</>
	);
}
