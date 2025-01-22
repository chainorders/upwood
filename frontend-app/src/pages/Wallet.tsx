import { useEffect, useState } from "react";
import { ApiUser, ForestProjectAggApiModel, ForestProjectState } from "../apiClient";
import PageHeader from "../components/PageHeader";
import { useOutletContext } from "react-router";
import ProjectCard2 from "../components/ProjectCard2";
import ClaimPopup from "../components/ClaimPopup";
import { Link } from "react-router";
export default function Wallet() {
    const { user } = useOutletContext<{ user: ApiUser }>();
    const carbon_credits = {
        emissions: "15 Co2 TONS",
        value: "750"
    };
    const dividends_details = "150 EUROe";
    const etree_details = "1500"
    const [carbon_credits_popup, setCarbonCreditsPopup] = useState(false);
    const [dividends_details_popup, setDividendsPopup] = useState(false);
    const [etrees_popup, setEtreesPopup] = useState(false);
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

    const __carbon_credits_details = {
        heading: "Claim carbon credits",
        list: [
            {
                tag: "Offset your emissions",
                display: carbon_credits.emissions
            },
            {
                tag: "Carbon credit dividends",
                display: carbon_credits.value + ' EUROe'
            }
        ]
    }
    const __dividends_details = {
        heading: "Dividends",
        list: [
            {
                tag: "Claim all dividends",
                display: dividends_details
            }
        ]
    }
    const __etrees_details = {
        heading: "E-trees",
        list: [
            {
                tag: "Claim E-trees",
                display: etree_details
            }
        ]
    }
    const table_data = [
        {
            token_symbol: "UPW1",
            asset_name: "Oak tree house forest plantation",
            smart_contact: "tdgsbha37326dnsajkjd8",
            share_amount: "5",
            share_value: "500€",
            carbon_credits: "3",
            dividends_earned: "150 €"
        },
        {
            token_symbol: "UPW1",
            asset_name: "Oak tree house forest plantation",
            smart_contact: "tdgsbha37326dnsajkjd8",
            share_amount: "5",
            share_value: "500€",
            carbon_credits: "3",
            dividends_earned: "150 €"
        },
        {
            token_symbol: "UPW1",
            asset_name: "Oak tree house forest plantation",
            smart_contact: "tdgsbha37326dnsajkjd8",
            share_amount: "5",
            share_value: "500€",
            carbon_credits: "3",
            dividends_earned: "150 €"
        },
        {
            token_symbol: "UPW1",
            asset_name: "Oak tree house forest plantation",
            smart_contact: "tdgsbha37326dnsajkjd8",
            share_amount: "5",
            share_value: "500€",
            carbon_credits: "3",
            dividends_earned: "150 €"
        },
        {
            token_symbol: "UPW1",
            asset_name: "Oak tree house forest plantation",
            smart_contact: "tdgsbha37326dnsajkjd8",
            share_amount: "5",
            share_value: "500€",
            carbon_credits: "3",
            dividends_earned: "150 €"
        },
        {
            token_symbol: "UPW1",
            asset_name: "Oak tree house forest plantation",
            smart_contact: "tdgsbha37326dnsajkjd8",
            share_amount: "5",
            share_value: "500€",
            carbon_credits: "3",
            dividends_earned: "150 €"
        },
        {
            token_symbol: "UPW1",
            asset_name: "Oak tree house forest plantation",
            smart_contact: "tdgsbha37326dnsajkjd8",
            share_amount: "5",
            share_value: "500€",
            carbon_credits: "3",
            dividends_earned: "150 €"
        },
        {
            token_symbol: "UPW1",
            asset_name: "Oak tree house forest plantation",
            smart_contact: "tdgsbha37326dnsajkjd8",
            share_amount: "5",
            share_value: "500€",
            carbon_credits: "3",
            dividends_earned: "150 €"
        },
        {
            token_symbol: "UPW1",
            asset_name: "Oak tree house forest plantation",
            smart_contact: "tdgsbha37326dnsajkjd8",
            share_amount: "5",
            share_value: "500€",
            carbon_credits: "3",
            dividends_earned: "150 €"
        },
        {
            token_symbol: "UPW1",
            asset_name: "Oak tree house forest plantation",
            smart_contact: "tdgsbha37326dnsajkjd8",
            share_amount: "5",
            share_value: "500€",
            carbon_credits: "3",
            dividends_earned: "150 €"
        }
    ]

    return (
        <>
            <div className="clr"></div>
            <div className="walletmanagement">
                <PageHeader userFullName={user.fullName} initials={user.initials} parts={[
                    { name: "Wallet Management" },
                ]} />
                <div className="outerboxshadow">
                    <div className="container">
                        <div className="container-in">
                            <div className="col-8 fl">
                                <div className="heading">Balance</div>
                            </div>
                            <div className="col-4 text-align-right fr hideonmobile">
                                <Link type="text" to="/"
                                    className="guides"
                                    style={{ cursor: "pointer" }}
                                >
                                    WALLET MANAGEMENT GUIDES
                                </Link>
                            </div>
                            <div className="col-4 text-align-right fr showonmobile">
                                <Link type="text" to="/"
                                    className="guides"
                                    style={{ cursor: "pointer" }}
                                >
                                    GUIDES
                                </Link>
                            </div>

                            <div className="clr"></div>
                        </div>
                        <div className="space-20"></div>
                        <div className="container-in">
                            <div className='col-20-percent fl walletclms col-m-full col-mr-bottom-30'>
                                <div className="tag">Wallet</div>
                                <div className="value">{user.account_address || "NA"}</div>
                                <span>Change</span>
                            </div>
                            <div className='col-20-percent fl walletclms col-m-full col-mr-bottom-30'>
                                <div className="tag">Entity</div>
                                <div className="value">SIA UPWOOD</div>
                            </div>
                            <div className='col-20-percent fl walletclms col-m-full col-mr-bottom-30'>
                                <div className="tag">Carbon credits</div>
                                <div className="value">{carbon_credits.emissions} = {carbon_credits.value}€</div>
                                <button onClick={() => setCarbonCreditsPopup(true)}>Claim</button>
                            </div>
                            <div className='col-20-percent fl walletclms col-m-full col-mr-bottom-30'>
                                <div className="tag">Dividends</div>
                                <div className="value">{dividends_details}</div>
                                <button onClick={() => setDividendsPopup(true)}>Claim</button>
                            </div>
                            <div className='col-20-percent fl walletclms col-m-full'>
                                <div className="tag">E-trees</div>
                                <div className="value">{etree_details}</div>
                                <button onClick={() => setEtreesPopup(true)}>Claim</button>
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
                            <div className="col-6 col-m-full fl">
                                <div className="heading">Token list</div>
                            </div>
                            <div className="col-6 text-align-right fr hideonmobile">
                            <Link type="text" to="/" className="guides" style={{ cursor: "pointer" }}>
                                    Export transaction history
                                </Link>
                                <Link type="text" to="/" className="guides margin" style={{ cursor: "pointer" }}>
                                    Export token list
                                </Link>
                            </div>
                            <div className="clr"></div>
                        </div>
                    </div>
                    <div className="container">
                        <div className="container-in">
                            <div className="col-12">
                                <div className='table'>
                                    <table cellSpacing={0}>
                                        <thead>
                                            <tr>
                                                <th>Token symbol</th>
                                                <th>Asset name</th>
                                                <th>Smart contract address</th>
                                                <th>Share amount</th>
                                                <th>Share value</th>
                                                <th>Carbon credits</th>
                                                <th>Dividends earned</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {table_data.map((item, index) => (
                                                <tr key={index}>
                                                    <td>{item.token_symbol}</td>
                                                    <td>{item.asset_name}</td>
                                                    <td>{item.smart_contact}</td>
                                                    <td>{item.share_amount}</td>
                                                    <td>{item.share_value}</td>
                                                    <td>{item.carbon_credits}</td>
                                                    <td>{item.dividends_earned}</td>
                                                </tr>
                                            ))}
                                        </tbody>
                                    </table>
                                </div>
                                <div className='pagignation'>
                                    <ul>
                                        <li className='disabled'>{'<'}</li>
                                        <li className='active'>{'1'}</li>
                                        <li>{'2'}</li>
                                        <li>{'3'}</li>
                                        <li>{'4'}</li>
                                        <li>{'5'}</li>
                                        <li>{'>'}</li>
                                    </ul>
                                </div>
                                <div className="space-30"></div>
                            </div>
                            <div className='col-12 showonmobile text-align-center'>
                                <Link type="text" to="/" className="guides" style={{ cursor: "pointer" }}>
                                    Export transaction history
                                </Link>
                                <Link type="text" to="/" className="guides margin" style={{ cursor: "pointer" }}>
                                    Export token list
                                </Link>
                            </div>
                            <div className="space-20 showonmobile"></div>
                        </div>
                    </div>
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
            {carbon_credits_popup ? <ClaimPopup config={__carbon_credits_details} close={() => setCarbonCreditsPopup(false)} /> : null}
            {dividends_details_popup ? <ClaimPopup config={__dividends_details} close={() => setDividendsPopup(false)} /> : null}
            {etrees_popup ? <ClaimPopup config={__etrees_details} close={() => setEtreesPopup(false)} /> : null}
        </>
    );
}