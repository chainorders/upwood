import { useEffect, useState } from "react";

import {
	ForestProjectService,
	ForestProjectState,
	PagedResponse_ForestProjectAggApiModel,
	SystemContractsConfigApiModel,
	UserService,
} from "../apiClient";
import PageHeader from "../components/PageHeader";
import ProjectCardFunded from "../components/ProjectCardFunded";
import { User } from "../lib/user";

interface FundedForestProjectsListProps {
	user: User;
}

export default function FundedForestProjectsList({ user }: FundedForestProjectsListProps) {
	const [projects, setProjects] = useState<PagedResponse_ForestProjectAggApiModel>();
	const [contracts, setContracts] = useState<SystemContractsConfigApiModel>();

	useEffect(() => {
		ForestProjectService.getForestProjectsList(ForestProjectState.FUNDED, 0).then(setProjects);
		UserService.getSystemConfig().then(setContracts);
	}, [user]);

	return (
		<>
			<div className="clr"></div>
			<div className="projects">
				<PageHeader user={user} parts={[{ name: "Funded Projects" }]} />
				<div className="container">
					<div className="container-in">
						{contracts &&
							projects?.data.map((project) => (
								<div className="col-6 col-m-full fl" key={project.forest_project.id}>
									<ProjectCardFunded project={project} user={user} />
								</div>
							))}
						<div className="clr"></div>
					</div>
				</div>
			</div>
		</>
	);
}
