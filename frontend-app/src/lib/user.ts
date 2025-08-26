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
	mfaEnabled = false;
	picture: string;
	companyId: string;

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
			picture,
			companyId,
		} = session.getIdToken().payload;

		this.firstName = given_name;
		this.lastName = family_name;
		this.fullName = `${given_name} ${family_name}`;
		this.initialis = `${given_name[0]} ${family_name[0] || ""}`;
		this.email = email;
		this.concordiumAccountAddress = concordium_account_address;
		this.idToken = session.getIdToken().getJwtToken();
		this.isAdmin = groups?.includes("admin");
		this.emailVerified = email_verified === "true";
		this.picture = picture;
		this.companyId = companyId;
	}

	forceRefresh() {
		return new Promise<User>((resolve, reject) => {
			this.cognitoUser.refreshSession(
				this.session.getRefreshToken(),
				(err: Error | null, session: CognitoUserSession | null) => {
					if (err) {
						reject(err);
						return;
					}
					this.session = session!;
					this.idToken = session!.getIdToken().getJwtToken();

					const {
						given_name,
						family_name,
						email,
						email_verified,
						"custom:con_accnt": concordium_account_address,
						"cognito:groups": groups,
						picture,
						companyId,
					} = session!.getIdToken().payload;

					this.firstName = given_name;
					this.lastName = family_name;
					this.fullName = `${given_name} ${family_name}`;
					this.initialis = `${given_name[0]} ${family_name[0] || ""}`;
					this.email = email;
					this.concordiumAccountAddress = concordium_account_address;
					this.isAdmin = groups?.includes("admin");
					this.emailVerified = email_verified === "true";
					this.picture = picture;
					this.companyId = companyId;
					resolve(this);
				},
			);
		});
	}

	refresh() {
		return new Promise<User>((resolve, reject) => {
			this.cognitoUser.getSession((err: Error | null, session: CognitoUserSession | null) => {
				if (err) {
					console.error("Error getting session", err);
					reject(err);
					return;
				}

				if (session && !session.isValid()) {
					this.forceRefresh().then(resolve).catch(reject);
					return;
				}

				this.session = session!;
				this.idToken = session!.getIdToken().getJwtToken();
				resolve(this);
			});
		});
	}
}
