import { ContractAddress } from "@concordium/web-sdk";
import CCDCis2TokenLink from "./concordium/CCDCis2TokenLink";
import { Typography } from "@mui/material";
import { Cis2ExchangeRate, ExchangeRate } from "../market/types";

export default function ExchangeRateDisplay(props: {
	exchangeRate: ExchangeRate;
	token: { tokenId: string; contract: ContractAddress.Type };
}) {
	const { exchangeRate, token } = props;
	return (
		<>
			<Typography variant="body1">
				{exchangeRate.rate.numerator.toString()}{" "}
				<CCDCis2TokenLink tokenId={token.tokenId} contract={token.contract} />
				Tokens for&nbsp;
				{
					{
						Ccd: <>{exchangeRate.rate.denominator.toString()}&nbsp;CCD(s)</>,
						Cis2: (
							<>
								{exchangeRate.rate.denominator.toString()}&nbsp;
								<CCDCis2TokenLink
									tokenId={(exchangeRate as Cis2ExchangeRate).tokenId}
									contract={(exchangeRate as Cis2ExchangeRate).tokenContract}
								/>{" "}
								Tokens
							</>
						),
					}[exchangeRate.type]
				}
			</Typography>
		</>
	);
}
