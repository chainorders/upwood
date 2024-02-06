import { Button, Stack, Step, StepLabel, Stepper, Typography } from "@mui/material";
import { useState } from "react";
import { useVerifierApi } from "../VerifierApiProvider";
import { useWallet } from "../WalletProvider";
import { IdStatement } from "@concordium/web-sdk";
import SendTransactionButton from "../common/SendTransactionButton";
import ErrorDisplay from "../common/ErrorDisplay";

export default function VerifierPage() {
	const [activeStep, setActiveStep] = useState(0);
	const [challenge, setChallenge] = useState("");
	const [statement, setStatement] = useState<IdStatement>([]);
	const [error, setError] = useState("");

	const { provider: wallet, currentAccount } = useWallet();
	const { provider: api } = useVerifierApi();

	const generateChallenge = () => {
		api.default
			.postVerifierGenerateChallenge({
				requestBody: {
					account: currentAccount!.address,
				},
			})
			.then((response) => {
				setChallenge(response.challenge);
				setStatement(response.statement);
				setActiveStep(1);
			})
			.catch((e) => {
				setError(e.message);
			});
	};

	const registerIdentity = () => {
		return wallet!
			.requestIdProof(currentAccount!.address, statement, challenge)
			.then((proof) =>
				api.default.postVerifierRegisterIdentity({
					requestBody: {
						account: currentAccount!.address,
						proof: {
							credential: proof?.credential,
							proof: JSON.stringify(proof?.proof),
						},
					},
				})
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
		return (
			<>
				<Typography variant="h5">Register Identity</Typography>
				<SendTransactionButton
					disabled={!challenge || !statement}
					onClick={registerIdentity}
					onDone={() => setActiveStep(0)}>
					Register Identity
				</SendTransactionButton>
			</>
		);
	};

	return (
		<Stack spacing={2}>
			<Stepper activeStep={activeStep}>
				<Step>
					<StepLabel>Generate Challenge</StepLabel>
				</Step>
				<Step>
					<StepLabel>Register Identity</StepLabel>
				</Step>
			</Stepper>
			<Stack>
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
