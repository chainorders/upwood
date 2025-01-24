import { Link } from "react-router";

interface ButtonProps {
    text: string,
    link: string,
    active: boolean,
    style?: string,
    disabled?: boolean,
    icon?: string,
    icononright? : boolean,
    call?: () => void;
}
export default function Button({ text, link, active, style, disabled, icon, icononright, call }: ButtonProps) {
    return (
        <>
            {link === '' ?
                <button className={`button ${style} ${active ? 'active' : ''} ${disabled ? 'disabled' : ''}`} onClick={call}>
                    
                    {icon && icon !== '' && !icononright &&
                    <img
                        src={icon}
                        width="14"
                        height="14"
                        className="left"
                    />
                    }
                    {text}
                    {icon && icon !== '' && icononright &&
                    <img
                        src={icon}
                        width="14"
                        height="14"
                        className="right"
                    />
                    }
                    </button>
                :
                <Link to={link} className={`button ${style} ${active ? 'active' : ''} ${disabled ? 'disabled' : ''}`}>
                    {icon && icon !== '' && !icononright &&
                    <img
                        src={icon}
                        width="14"
                        height="14"
                        className="left"
                    />
                    }
                    {text}
                    {icon && icon !== '' && icononright &&
                    <img
                        src={icon}
                        width="14"
                        height="14"
                        className="right"
                    />
                    }
                </Link>
            }
        </>
    );
}
