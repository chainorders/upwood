import {
	Button,
	Divider,
	Grid,
	Stack,
	Step,
	StepLabel,
	Stepper,
	Typography,
} from "@mui/material";
import ErrorDisplay from "../common/ErrorDisplay";
import { useEffect, useState } from "react";
import { useVerifierApi } from "../VerifierApiProvider";
import {
	AccountAddress,
	ContractAddress,
	CredentialStatements,
} from "@concordium/web-sdk";
import { WalletApi } from "@concordium/browser-wallet-api-helpers";
import SendTransactionButton from "../common/SendTransactionButton";
import ContractAddressField from "../common/concordium/ContractAddressField";
import AccountAddressField from "../common/concordium/AccountAddressField";

export default function Registration(props: {
	wallet: WalletApi;
	currentAccount: AccountAddress.Type;
}) {
	const { wallet: wallet, currentAccount } = props;
	const [activeStep, setActiveStep] = useState(0);
	const [challenge, setChallenge] = useState("");
	const [statement, setStatement] = useState<CredentialStatements>([]);
	const [error, setError] = useState("");

	const { provider: api } = useVerifierApi();

	useEffect(() => {
		setError("");
		setStatement([]);
		setChallenge("");
		setActiveStep(0);
	}, [currentAccount]);

	const generateChallenge = () => {
		api.default
			.postVerifierGenerateChallenge({
				requestBody: {
					account: currentAccount!.address,
				},
			})
			.then((response) => {
				setChallenge(response.challenge);
				setStatement([
					{
						idQualifier: {
							type: "cred",
							issuers: response.identity_providers,
						},
						statement: response.id_statement,
					},
					{
						idQualifier: {
							type: "sci",
							issuers: response.issuers.map((i) =>
								ContractAddress.create(i.index, i.subindex),
							),
						},
						statement: response.cred_statement,
					},
				] as CredentialStatements);
				setActiveStep(1);
			})
			.catch((e) => {
				setError(e.message);
			});
	};

	const registerIdentity = (contract?: ContractAddress.Type) => {
		console.log("Registering identity", challenge, statement);
		return wallet
			.requestVerifiablePresentation(challenge, statement)
			.then((proof) =>
				api.default.postVerifierRegisterIdentity({
					requestBody: {
						account: currentAccount.address,
						proof,
						contract: contract && {
							index: Number(contract.index),
							subindex: Number(contract.subindex),
						},
					},
				}),
			)
			.then((res) => res.txn_hash);
	};

	const GenerateChallengeStep = () => {
		return (
			<>
				<Typography variant="h5">Generate Challenge</Typography>
				<Button onClick={() => generateChallenge()}>Generate Challenge</Button>
			</>
		);
	};

	const RegisterIdentityStep = () => {
		const [contract, setContract] = useState<ContractAddress.Type>();
		return (
			<Stack spacing={2}>
				<Typography variant="h5">Register Account Identity</Typography>
				<AccountAddressField value={currentAccount} disabled />
				<SendTransactionButton
					disabled={!challenge || !statement}
					onClick={registerIdentity}
					onDone={() => setActiveStep(0)}
				>
					Register Account Identity
				</SendTransactionButton>
				<Divider />
				<Typography variant="h5">Register Contract Identity</Typography>
				<ContractAddressField onChange={setContract} />
				<SendTransactionButton
					disabled={!challenge || !statement || !contract}
					onClick={registerIdentity}
					onDone={() => setActiveStep(0)}
				>
					Register Contract Identity
				</SendTransactionButton>
			</Stack>
		);
	};

	return (
		<Stack spacing={2} m={2}>
			<Stepper activeStep={activeStep}>
				<Step>
					<StepLabel>Generate Challenge</StepLabel>
				</Step>
				<Step>
					<StepLabel>Register Identity</StepLabel>
				</Step>
			</Stepper>
			<Stack spacing={2}>
				{
					{
						0: GenerateChallengeStep(),
						1: RegisterIdentityStep(),
					}[activeStep]
				}
				{error && <ErrorDisplay text={error} />}
			</Stack>
		</Stack>
	);
}
