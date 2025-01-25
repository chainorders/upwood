import { Link } from "react-router";

interface ButtonProps {
    text: string,
    link: string,
    active: boolean,
    style?: string,
    disabled?: boolean,
    icon?: string,
    icononright? : boolean,
    radius16? : boolean,
    danger? : boolean,
    call?: () => void;
}
export default function Button({ text, link, active, style, disabled, icon, icononright, radius16, danger, call }: ButtonProps) {
    return (
        <>
            {link === '' ?
                <button className={`button ${radius16?'radius16':''} ${style} ${active ? 'active' : ''} ${danger?'danger':''} ${disabled ? 'disabled' : ''}`} onClick={call}>
                    
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
                <Link to={link} className={`button ${radius16?'radius16':''} ${style} ${active ? 'active' : ''} ${danger?'danger':''} ${disabled ? 'disabled' : ''}`}>
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
