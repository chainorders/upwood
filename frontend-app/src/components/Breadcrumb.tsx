import { Link } from "react-router";

interface Item {
    name: string,
    link: string,
}

interface BreadcrumbProps {
    data: Item[]
}

export default function Breadcrumb({ data }: BreadcrumbProps) {
    return (
        <div className="breadcrumb">
            <ul>
                {data.map((item, index) => (
                    <li key={index}>
                        {item.link !== '' ?
                            <Link to={item.link}>{item.name}</Link>
                            :
                            <span>{item.name}</span>
                        }
                    </li>
                ))}
            </ul>
            <div className="clr"></div>
        </div>
    );
}
