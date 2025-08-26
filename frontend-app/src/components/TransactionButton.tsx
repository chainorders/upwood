import { Button, CircularProgress, ButtonProps } from "@mui/material";
import React from "react";

interface SubmitButtonProps extends ButtonProps {
	txnStatus: "none" | "sending" | "waiting" | "success" | "error";
	defaultText: string;
	loadingText: string;
}

const TransactionButton: React.FC<SubmitButtonProps> = ({ txnStatus, defaultText, loadingText, ...props }) => {
	return (
		<Button
			disabled={txnStatus === "sending" || txnStatus === "waiting"}
			startIcon={(txnStatus === "sending" || txnStatus === "waiting") && <CircularProgress size={20} />}
			color={txnStatus === "error" ? "error" : txnStatus === "success" ? "success" : "primary"}
			{...props}
		>
			{txnStatus === "sending" || txnStatus === "waiting" ? loadingText : defaultText}
		</Button>
	);
};

export default TransactionButton;
