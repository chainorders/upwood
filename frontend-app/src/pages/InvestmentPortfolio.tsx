import { useEffect, useState } from "react";
import { ApiUser, ForestProjectAggApiModel, ForestProjectState } from "../apiClient";
import PageHeader from "../components/PageHeader";
import ProjectCard2 from "../components/ProjectCard2";
import PortfolioValueChart from "../components/PortfolioValueChart";
import { useOutletContext } from "react-router";

export default function InvestmentPortfolio() {
    const { user } = useOutletContext<{ user: ApiUser }>();
    const [projects, setData] = useState<ForestProjectAggApiModel[]>([
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
    ]);
    useEffect(() => {
     //   setData()
    }, []);

    const topvalues = {
        locked_token_value: "99 000 €",
        portfolio_value: "13 000 €",
        yearly_portfolio_growth_value: "+1000€",
        return_investment_value: "8.4%",
        carbon_tons_value: "5t"
    }
    return (
        <>
            <div className="clr"></div>
            <div className="investmentportfolio">
                <PageHeader userFullName={user.fullName} initials={user.initials} parts={[{ name: "Investment Portfolio" }]} />
                <div className="outerboxshadow">
                    <div className="container">
                        <div className="container-in">
                            <div className='col-20-percent fl investmentms col-m-full col-mr-bottom-30'>
                                <div className="tag">Locked token value</div>
                                <div className="value">{topvalues.locked_token_value}</div>
                            </div>
                            <div className='col-20-percent fl investmentms col-m-full col-mr-bottom-30'>
                                <div className="tag">Portfolio value</div>
                                <div className="value">{topvalues.portfolio_value}</div>
                            </div>
                            <div className='col-20-percent fl investmentms col-m-full col-mr-bottom-30'>
                                <div className="tag">Yearly portfolio growth</div>
                                <div className="value">{topvalues.yearly_portfolio_growth_value}</div>
                            </div>
                            <div className='col-20-percent fl investmentms col-m-full col-mr-bottom-30'>
                                <div className="tag">Return on investment</div>
                                <div className="value">{topvalues.return_investment_value}</div>
                            </div>
                            <div className='col-20-percent fl investmentms col-m-full'>
                                <div className="tag">Carbon tons offset</div>
                                <div className="value">{topvalues.carbon_tons_value}</div>
                            </div>
                            <div className="clr"></div>
                        </div>
                        <div className="space-30"></div>
                    </div>
                </div>
                <div className="space-30"></div>
                <div className="outerboxshadow">
                    <div className="container">
                        <div className="container-in">
                            <div className="col-8 col-m-full fl">
                                <div className="heading">Portfolio value</div>
                            </div>
                            <div className="col-4 fr">

                            </div>
                            <div className="clr"></div>
                        </div>
                    </div>
                    <div className="space-20"></div>
                    <div className="container">
                        <div className="container-in">
                            <div className="col-12">
                                <div className="chart">
                                    <div className="chart-in">
                                        {/* <PortfolioValueChart values={[]} /> */}
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div className="space-30"></div>
                </div>
                <div className="space-30"></div>
            </div>
            <div className="projects">
                <div className="container">
                    <div className="container-in">
                        {projects.map((project, index) => (
                            <div className="col-6 col-m-full fl" key={index}>
                                <ProjectCard2 item={project} />
                            </div>
                        ))}
                        <div className="clr"></div>
                    </div>
                </div>
            </div>
        </>
    );
}