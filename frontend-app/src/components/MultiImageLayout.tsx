interface Item {
    title: string;
    short: string;
    large: string;
    images: string[];
}
interface MultiImageProps {
    data: Item
}
export default function MultiImageLayout({ data }: MultiImageProps) {
    return (
        <div className="multiimage">
            <div className="title">{data.title}</div>
            <div className="description">{data.short}</div>
            <div className="images">
                <div className="container">
                    <div className="container-in">
                        {data.images.map((im, index) => (
                            <div className="col-3 col-m-full col-mr-bottom-10 fl" key={index}>
                                <div className="im">
                                    <img
                                        src={im}
                                        //   layout="responsive"
                                        width={100}
                                        height={100}
                                    />
                                </div>
                            </div>
                        ))}
                        <div className="clr"></div>
                    </div>
                </div>
            </div>
            <div className="description">{data.large}</div>
        </div>
    );
}
