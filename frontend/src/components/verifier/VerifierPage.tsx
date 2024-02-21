import VerifierApiProvider from "../VerifierApiProvider";
import { useWallet } from "../WalletProvider";
import Registration from "./Registration";

export default function VerifierPage() {
	const { provider: wallet, currentAccount } = useWallet();

	return (
		<VerifierApiProvider>
			<Registration wallet={wallet!} currentAccount={currentAccount!} />
		</VerifierApiProvider>
	);
}
