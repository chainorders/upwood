import "../styles/NotFound.css";
import { FaExclamationTriangle } from "react-icons/fa";

export default function NotFound() {
	return (
		<>
			<div className="not-found">
				<FaExclamationTriangle className="not-found-icon" />
				<h1>404 - Page Not Found</h1>
				<p>The page you are looking for does not exist.</p>
			</div>
		</>
	);
}
