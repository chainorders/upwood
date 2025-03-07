import { Link } from "react-router";

interface ButtonProps {
	text: string;
	link?: string;
	active?: boolean;
	style?: string;
	disabled?: boolean;
	icon?: string;
	icononright?: boolean;
	radius16?: boolean;
	danger?: boolean;
	call?: () => void;
	linkState?: unknown;
	loading?: boolean;
	type?: "button" | "submit";
	linkTarget?: string;
	form?: string;
}
export default function Button({
	text,
	link,
	active,
	style,
	disabled,
	icon,
	icononright,
	radius16,
	danger,
	call,
	linkState,
	linkTarget,
	loading,
	type,
	form,
}: ButtonProps) {
	return (
		<>
			{!link ? (
				<button
					form={form}
					type={type || "button"}
					disabled={disabled || loading}
					className={`button ${radius16 ? "radius16" : ""} ${style || ""} ${active ? "active" : ""} ${danger ? "danger" : ""} ${disabled ? "disabled" : ""} ${loading ? "button__loader" : ""}`}
					onClick={call}
				>
					{icon && icon !== "" && !icononright && !loading && <img src={icon} width="14" height="14" className="left" />}
					<span className="button__text">{text}</span>
					{icon && icon !== "" && icononright && !loading && <img src={icon} width="14" height="14" className="right" />}
				</button>
			) : (
				<Link
					to={link}
					state={linkState}
					target={linkTarget ? "_blank" : "_self"}
					rel="noreferrer"
					className={`button ${radius16 ? "radius16" : ""} ${style || ""} ${active ? "active" : ""} ${danger ? "danger" : ""} ${disabled ? "disabled" : ""} ${loading ? "button__loader" : ""}`}
				>
					{icon && icon !== "" && !icononright && !loading && <img src={icon} width="14" height="14" className="left" />}
					<span className="button__text">{text}</span>
					{icon && icon !== "" && icononright && !loading && <img src={icon} width="14" height="14" className="right" />}
				</Link>
			)}
		</>
	);
}
