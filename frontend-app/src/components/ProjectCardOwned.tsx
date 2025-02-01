import { ForestProjectAggApiModel } from "../apiClient";
import Button from "./Button";

interface ItemProps {
	item: ForestProjectAggApiModel;
}

export default function ProjectCardOwned({ item }: ItemProps) {
	return (
		<>
			<div className="project-card">
				<div className="container">
					<div className="container-in">
						<div className="col-12">
							<div className="image">
								<img src={item.forest_project.image_small_url} alt="Description of the image" />
								<div className="caption">{item.forest_project.label}</div>
							</div>
						</div>
					</div>
				</div>
				<div className={`container`}>
					<div className="container-in">
						<div className="col-12">
							<div className="project-name">{item.forest_project.name}</div>
							<div className="project-description">{item.forest_project.desc_short}</div>
						</div>
					</div>
					<div className="container-in">
						<div className="col-4 col-m-padding-right-0 fl">
							<span className="colb">AREA</span>
							<span className="colc">{item.forest_project.area}</span>
						</div>
						<div className="col-4 col-m-padding-right-0 fl">
							<span className="colb">ROI</span>
							<span className="colc">{item.forest_project.roi_percent}%</span>
						</div>
						<div className="col-4 col-m-padding-right-0 fl">
							<span className="colb">CARBON CREDITS</span>
							<span className="colc">{item.forest_project.carbon_credits}%</span>
						</div>
						<div className="clr"></div>
					</div>
					<div className="container-in">
						<div className="col-5 col-m-full col-mr-bottom-20 fl">
							<Button text={"SELL SHARES"} link={""} active={false} />
						</div>
						<div className="col-5 col-m-full fr">
							<Button text={"INVEST MORE"} link={""} active={true} />
						</div>
						<div className="clr"></div>
					</div>
				</div>
			</div>
		</>
	);
}
