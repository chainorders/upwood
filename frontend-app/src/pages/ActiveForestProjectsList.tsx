import { useEffect, useState } from "react";
import { ForestProjectAggApiModel } from "../apiClient/models/ForestProjectAggApiModel";
import { ForestProjectService, ForestProjectState, LoginRes } from "../apiClient";
import ProjectCardActive from "../components/ProjectCardActive";
import PageHeader from "../components/PageHeader";
import { useOutletContext } from "react-router";

export default function ActiveForestProjectsList() {
	const { user } = useOutletContext<{ user: LoginRes }>();
	const [projects, setProjects] = useState<ForestProjectAggApiModel[]>([]);
	useEffect(() => {
		ForestProjectService.getForestProjectsList(ForestProjectState.ACTIVE, 0).then((response) => {
			setProjects(response.data);
		});
	}, []);

	return (
		<>
			<div className="clr"></div>
			<div className="projects">
				<PageHeader user={user} parts={[{ name: "Active Projects" }]} />
				<div className="container">
					<div className="container-in">
						{projects.map((project, index) => (
							<div className="col-6 col-m-full fl" key={index}>
								<ProjectCardActive item={project} />
							</div>
						))}
						<div className="clr"></div>
					</div>
				</div>
			</div>
		</>
	);
}
