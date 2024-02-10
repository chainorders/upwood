import { TransactionHash } from "@concordium/web-sdk";

export default function CCDScanTransactionLink(props: {
	transactionHash: TransactionHash.Type;
}) {
	const ccdScanWebsite = import.meta.env.VITE_CCDSCAN_TRANSACTION_LINK!.replace(
		"<transactionHash>",
		TransactionHash.toHexString(props.transactionHash),
	);
	return (
		<a href={ccdScanWebsite} target="_blank" rel="noreferrer">
			{TransactionHash.toHexString(props.transactionHash).substring(0, 10)}...
		</a>
	);
}
