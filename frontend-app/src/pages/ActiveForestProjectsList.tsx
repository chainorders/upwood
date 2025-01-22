import { useEffect, useState } from "react";
import { ForestProjectAggApiModel } from "../apiClient/models/ForestProjectAggApiModel";
import { ApiUser, ForestProjectState } from "../apiClient";
import ProjectCard from "../components/ProjectCard";
import PageHeader from "../components/PageHeader";
import { useOutletContext } from "react-router";

export default function ActiveForestProjectsList() {
    const { user } = useOutletContext<{ user: ApiUser }>();
    const [projects, setProjects] = useState<ForestProjectAggApiModel[]>([]);
    useEffect(() => {
        setProjects([
            {
                forest_project: {
                    area: "100",
                    carbon_credits: 100,
                    created_at: "2021-09-07T12:00:00Z",
                    desc_long: "This is a long description",
                    desc_short: "This is a short description",
                    id: "forest_project_id_1",
                    image_small_url: "https://picsum.photos/id/237/400/200",
                    image_large_url: "https://picsum.photos/id/237/800/600",
                    label: "Grow",
                    latest_price: "100",
                    name: "Project Name",
                    property_media_footer: "Property Media Footer",
                    property_media_header: "Property Media Header",
                    roi_percent: 100,
                    shares_available: 100,
                    state: ForestProjectState.ACTIVE,
                    updated_at: "2021-09-07T12:00:00Z",
                    geo_spatial_url: "https://via.placeholder.com/150",
                    offering_doc_link: "https://via.placeholder.com/150",
                },
                supply: "100",
                user_balance: "100",
            },
            {
                forest_project: {
                    area: "100",
                    carbon_credits: 100,
                    created_at: "2021-09-07T12:00:00Z",
                    desc_long: "This is a long description",
                    desc_short: "This is a short description",
                    id: "forest_project_id_2",
                    image_small_url: "https://picsum.photos/id/237/400/200",
                    image_large_url: "https://picsum.photos/id/237/800/600",
                    label: "Grow",
                    latest_price: "100",
                    name: "Project Name 2",
                    property_media_footer: "Property Media Footer",
                    property_media_header: "Property Media Header",
                    roi_percent: 100,
                    shares_available: 100,
                    state: ForestProjectState.ACTIVE,
                    updated_at: "2021-09-07T12:00:00Z",
                    geo_spatial_url: "https://via.placeholder.com/150",
                    offering_doc_link: "https://via.placeholder.com/150",
                },
                supply: "100",
                user_balance: "100",
            }
        ])
    }, []);

    return (
        <>
            <div className="clr"></div>
            <div className="projects">
                <PageHeader userFullName={user.fullName} initials={user.initials} parts={[
                    { name: "Active Projects" },
                ]} />
                <div className="container">
                    <div className="container-in">
                        {projects.map((project, index) => (
                            <div className="col-6 col-m-full fl" key={index}>
                                <ProjectCard item={project} />
                            </div>
                        ))}
                        <div className="clr"></div>
                    </div>
                </div>
            </div>
        </>
    );
}