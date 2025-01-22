import { Link } from "react-router";

export default function PageHeader(props: { parts: { name: string, link?: string }[], userFullName: string, initials: string }) {
    return <>
        <div className="container">
            <div className="container-in">
                <div className="col-12">
                    <h1 className="breadcrumb">
                        {
                            props.parts.map((part, index) => {
                                return <span>
                                    {part.link ?
                                        <Link type="text" to={part.link} key={index}>{part.name}</Link>
                                        :
                                        <span>{part.name}</span>
                                    }
                                    {index < props.parts.length - 1 && <span> &gt; </span>}
                                </span>
                            })
                        }
                        <div className="username fr" style={{ marginTop: '-0.5em' }}>
                            <span>{props.initials}</span>
                            {props.userFullName}
                        </div>
                    </h1>
                </div>
            </div>
        </div>
        <div className="space-30"></div>
    </>
}