import { Link } from "react-router";

interface ButtonProps {
    text: string,
    link: string,
    active: boolean,
    style?: string,
    disabled?: boolean,
    icon?: string,
    call?: () => void;
}
export default function Button({ text, link, active, style, disabled, icon, call }: ButtonProps) {
    return (
        <>
            {link === '' ?
                <span className={`button ${style} ${active ? 'active' : ''} ${disabled ? 'disabled' : ''}`} onClick={call}>{icon && icon !== '' ?
                    <img
                        src={icon}
                        width={14}
                        height={14}
                    />
                    : null}{text}</span>
                :
                <Link to={link} className={`button ${style} ${active ? 'active' : ''} ${disabled ? 'disabled' : ''}`}>
                    {icon && icon !== '' ?
                        <img
                            src={icon}
                            width={14}
                            height={14}
                        /> : <></>
                    }
                    {text}
                </Link>
            }
        </>
    );
}
