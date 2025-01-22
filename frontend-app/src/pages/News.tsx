import { useState } from "react";
import { Link, useOutletContext } from "react-router";
import { ApiUser } from "../apiClient";
import PageHeader from "../components/PageHeader";
import NewsCard from "../components/NewsCard";

export default function News() {
    const { user } = useOutletContext<{ user: ApiUser }>();
    const [showAll, setShowAll] = useState(false);

    const links = [
        { title: "Update: 1.13", description: "Proactively incubate innovative processes for high-payoff architectures. Globally benchmark flexible.", link: "" },
        { title: "Update: 1.13", description: "Proactively incubate innovative processes for high-payoff architectures. Globally benchmark flexible.", link: "" },
        { title: "Update: 1.13", description: "Proactively incubate innovative processes for high-payoff architectures. Globally benchmark flexible.", link: "" },
        { title: "Update: 1.13", description: "Proactively incubate innovative processes for high-payoff architectures. Globally benchmark flexible.", link: "" },
        { title: "Update: 1.13", description: "Proactively incubate innovative processes for high-payoff architectures. Globally benchmark flexible.", link: "" },
        { title: "Update: 1.13", description: "Proactively incubate innovative processes for high-payoff architectures. Globally benchmark flexible.", link: "" },
        { title: "Update: 1.13", description: "Proactively incubate innovative processes for high-payoff architectures. Globally benchmark flexible.", link: "" },
        { title: "Update: 1.13", description: "Proactively incubate innovative processes for high-payoff architectures. Globally benchmark flexible.", link: "" },
        { title: "Update: 1.13", description: "Proactively incubate innovative processes for high-payoff architectures. Globally benchmark flexible.", link: "" },
        { title: "Update: 1.13", description: "Proactively incubate innovative processes for high-payoff architectures. Globally benchmark flexible.", link: "" },
        { title: "Update: 1.13", description: "Proactively incubate innovative processes for high-payoff architectures. Globally benchmark flexible.", link: "" }
    ];

    const data = [
        {
            id: "1256",
            image: "/Photo2.jpg",
            image_label: "ABOUT UPWOOD",
            title: "Upwood signs agreement with Airflygreen",
            short: "AirFlyGreen joins investor cummunity with commitment to compensate 6000 tons Co2 of flight emissions from London to Dubai. On 27th June Upwood announced official partnership si..."
        },
        {
            id: "1516",
            image: "/Photo2.jpg",
            image_label: "LETTER FROM CEO",
            title: "Forest market experience positive dynamics",
            short: "Following recent changes in reduction of Euribor % by European Central Bank construction industry experience upward trend which leads to increased demand for timber. Current, trajectory..."
        }
    ];

    const planned_maintainence = {
        date: "22.10.24",
        text: "Next planned platform maintenance is going to happen at 22.07.2024 22 : 00 (CET), please be aware that there may be short time frame when platform may show service maintenance. We are constantly working on platform improvements, updates and new features to provide better investment experience. Your funds are in your digital wallet, thereby, always safe!"
    };

    const displayedLinks = showAll ? links : links.slice(0, 3);
    return (
        <>
            <div className="clr"></div>
            <div className="news">
                <PageHeader userFullName={user.fullName} initials={user.initials} parts={[{ name: "News & Updates" }]} />
                <div className="outerboxshadow">
                    <div className="container">
                        <div className="container-in">
                            <div className="col-4 fr text-align-right col-m-full col-mr-bottom-10 text-align-left-mobile">
                                {planned_maintainence.date}
                            </div>
                            <div className="col-8 fl col-m-full">
                                <div className="heading">Planned platform maintenance</div>
                            </div>
                            <div className="clr"></div>
                        </div>
                        <div className="container-in">
                            <div className="col-12">
                                <div className="content">{planned_maintainence.text}</div>
                            </div>
                        </div>
                    </div>
                </div>
                <div className="space-30"></div>
                <div className="outerboxshadow">
                    <div className="container">
                        <div className="container-in">
                            <div className="col-8 col-m-full fl">
                                <div className="heading">Platform updates</div>
                            </div>
                            <div className="col-4 text-align-right fr hideonmobile">
                                <span
                                    className="seeall"
                                    onClick={() => setShowAll(!showAll)}
                                    style={{ cursor: "pointer" }}
                                >
                                    {showAll ? "SEE LESS" : "SEE ALL"}
                                </span>
                            </div>

                            <div className="clr"></div>
                            {displayedLinks.map((item, index) => (
                                <div className="col-4 col-m-full fl" key={index}>
                                    <div className="linkbox">
                                        <Link to={item.link}>
                                            <div className="title">{item.title}</div>
                                            <div className="description">{item.description}</div>
                                        </Link>
                                    </div>
                                </div>
                            ))}
                            <div className="clr"></div>
                            <div className='col-12 showonmobile text-align-center'>
                                <span
                                    className="seeall"
                                    onClick={() => setShowAll(!showAll)}
                                    style={{ cursor: "pointer" }}
                                >
                                    {showAll ? "SEE LESS" : "SEE ALL"}
                                </span>
                            </div>
                            <div className="space-20 showonmobile"></div>
                        </div>
                    </div>
                </div>
                <div className="space-30"></div>
                <div className="container">
                    <div className="container-in">
                        {data.map((item, index) => (
                            <div className="col-6 col-m-full fl" key={index}>
                                <NewsCard item={item} />
                            </div>
                        ))}
                        <div className="clr"></div>
                    </div>
                </div>
            </div>
        </>
    );
}