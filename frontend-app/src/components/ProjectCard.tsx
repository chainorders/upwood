import { useState } from "react";
import Button from "./Button";
import BuyShare from "./BuyShare";
import { ForestProjectAggApiModel, ForestProjectState } from "../apiClient";

interface ItemProps {
    item: ForestProjectAggApiModel
}

export default function ProjectCard({ item }: ItemProps) {
    const [buyShare, setBuyShare] = useState(false);
    const openBuyShare = () => {
        setBuyShare(true);
    };
    const closeBuyShare = () => {
        setBuyShare(false);
    };
    const buyShareConfig = {
        heading: "Buy shares",
        title: item.forest_project.name,
        share_price: BigInt(item.forest_project.latest_price),
        share_available: BigInt(item.forest_project.shares_available),
    };

    const comingSoon = item.forest_project.state === ForestProjectState.DRAFT;
    return (
        <>
            <div className="project-card">
                <div className="container">
                    <div className="container-in">
                        <div className="col-12">
                            <div className="image">
                                <img
                                    src={item.forest_project.image_small_url}
                                />
                                <div className="caption">{comingSoon ? 'coming soon' : item.forest_project.label}</div>
                            </div>
                        </div>
                    </div>
                </div>
                <div className={`container ${comingSoon ? 'disable-overlay' : ''}`}>
                    <div className="container-in">
                        <div className="col-12">
                            <div className="project-name">{comingSoon ? 'To be announced' : item.forest_project.name}</div>
                            <div className="project-description">{comingSoon ? 'Description coming soon, please wait' : item.forest_project.desc_short}</div>
                        </div>
                    </div>
                    <div className="container-in">
                        <div className="col-4 col-m-padding-right-0 fl">
                            <span className="colb">AREA</span>
                            <span className="colc">{comingSoon ? 'TBA' : item.forest_project.area}</span>
                        </div>
                        <div className="col-4 col-m-padding-right-0 fl">
                            <span className="colb">ROI</span>
                            <span className="colc">{comingSoon ? 'TBA' : item.forest_project.roi_percent}%</span>
                        </div>
                        <div className="col-4 col-m-padding-right-0 fl">
                            <span className="colb">CARBON CREDITS</span>
                            <span className="colc">{comingSoon ? 'TBA' : item.forest_project.carbon_credits}%</span>
                        </div>
                        <div className="clr"></div>
                    </div>
                    <div className="container-in">
                        <div className="col-8 col-m-full col-mr-bottom-20 fl">
                            <Button text={'VIEW DETAILS'} link={`/projects/active/${item.forest_project.id}`} active={false} />
                        </div>
                        <div className="col-4 col-m-full fl">
                            <Button text={'INVEST'} link={''} active={true} call={openBuyShare} />
                        </div>
                        <div className="clr"></div>
                    </div>
                </div>
            </div>
            {buyShare ? <BuyShare config={buyShareConfig} close={closeBuyShare} /> : null}
        </>
    );
}
