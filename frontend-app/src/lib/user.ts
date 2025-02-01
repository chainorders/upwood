import { CognitoUser, CognitoUserSession } from "amazon-cognito-identity-js";

export class User {
	firstName: string;
	lastName: string;
	fullName: string;
	initialis: string;
	email: string;
	concordiumAccountAddress: string;
	idToken: string;
	isAdmin: boolean;
	emailVerified: boolean;

	constructor(
		public session: CognitoUserSession,
		public cognitoUser: CognitoUser,
	) {
		const {
			given_name,
			family_name,
			email,
			email_verified,
			"custom:con_accnt": concordium_account_address,
			"cognito:groups": groups,
		} = session.getIdToken().payload;

		this.firstName = given_name;
		this.lastName = family_name;
		this.fullName = `${given_name} ${family_name}`;
		this.initialis = `${given_name[0]} ${family_name[0]}`;
		this.email = email;
		this.concordiumAccountAddress = concordium_account_address;
		this.idToken = session.getIdToken().getJwtToken();
		this.isAdmin = groups.includes("admin");
		this.emailVerified = email_verified === "true";
	}
}
