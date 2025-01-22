import { useParams } from "react-router";

export default function NewsDetails() {
    const { id } = useParams<{ id: string }>();

    const data = {
        id: "1516",
        image: "/Photo2.jpg",
        image_label: "ABOUT UPWOOD",
        title: "Upwood signs agreement with Airflygreen",
        large: "<p>AirFlyGreen joins investor cummunity with commitment to compensate 6000 tons Co2 of flight emissions from London to Dubai. On 27th June Upwood announced official partnership si... </p><p>Contrary to popular belief, Lorem Ipsum is not simply random text. It has roots in a piece of classical Latin literature from 45 BC, making it over 2000 years old. Richard McClintock, a Latin professor at Hampden-Sydney College in Virginia, looked up one of the more obscure Latin words, consectetur, from a Lorem Ipsum passage, and going through the cites of the word in classical literature, discovered the undoubtable source. Lorem Ipsum comes from sections 1.10.32 and 1.10.33 of 'de Finibus Bonorum et Malorum' (The Extremes of Good and Evil) by Cicero, written in 45 BC. This book is a treatise on the theory of ethics, very popular during the Renaissance. The first line of Lorem Ipsum, 'Lorem ipsum dolor sit amet..', comes from a line in section 1.10.32.</p><p>The standard chunk of Lorem Ipsum used since the 1500s is reproduced below for those interested. Sections 1.10.32 and 1.10.33 from 'de Finibus Bonorum et Malorum' by Cicero are also reproduced in their exact original form, accompanied by English versions from the 1914 translation by H. Rackham.</p>"
    };

    const breadcrumb = [
        {
            name: "News",
            link: '/news-updates'
        },
        {
            name: "Upwood signs agreement with Airflygreen",
            link: ''
        }
    ];

    return (
        <>
            <div className="news-detail">
                <div className="fl"><Breadcrumb data={breadcrumb} /></div>
                <Username />
                <div className="clr"></div>
                <div className="space-30"></div>
                <div className="image">
                    <Image
                        src={data.image}
                        alt="Description of the image"
                        layout="responsive"
                        width={100}
                        height={100}
                    />
                    <div className="caption">{data.image_label}</div>
                </div>
                <div className="space-30"></div>
                <div className="container">
                    <div className="container-in">
                        <div className="col-12 col-m-full col-mr-bottom-20 fl">
                            <div className="news-name">{data.title}</div>
                        </div>
                        <div className="clr"></div>
                    </div>
                </div>
                <div className="container">
                    <div className="container-in">
                        <div className="col-12">
                            <div className="news-description" dangerouslySetInnerHTML={{ __html: data.large }}>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </>
    );
}