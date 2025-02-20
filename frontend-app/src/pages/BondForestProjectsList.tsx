import { useEffect, useState } from "react";
import { useOutletContext } from "react-router";

import {
	ForestProjectService,
	ForestProjectState,
	PagedResponse_ForestProjectAggApiModel_,
	SystemContractsConfigApiModel,
	UserService,
} from "../apiClient";
import PageHeader from "../components/PageHeader";
import ProjectCardBond from "../components/ProjectCardBond";
import { User } from "../lib/user";

export default function BondForestProjectsList() {
	const { user } = useOutletContext<{ user: User }>();
	const [projects, setProjects] = useState<PagedResponse_ForestProjectAggApiModel_>();
	const [contracts, setContracts] = useState<SystemContractsConfigApiModel>();

	useEffect(() => {
		ForestProjectService.getForestProjectsList(ForestProjectState.BOND, 0).then(setProjects);
		UserService.getSystemConfig().then(setContracts);
	}, [user]);

	return (
		<>
			<div className="clr"></div>
			<div className="projects">
				<PageHeader user={user} parts={[{ name: "Investment Bonds" }]} />
				<div className="container">
					<div className="container-in">
						{contracts &&
							projects?.data.map((project, index) => (
								<div className="col-6 col-m-full fl" key={index}>
									<ProjectCardBond project={project} user={user} contracts={contracts} />
								</div>
							))}
						<div className="clr"></div>
					</div>
				</div>
			</div>
		</>
	);
}
