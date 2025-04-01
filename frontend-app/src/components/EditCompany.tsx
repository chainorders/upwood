import { useCallback, useEffect, useState } from "react";
import closeIcon from "../assets/close.svg";
import Button from "./Button";
import Remove from "../assets/remove.svg";
import {
	Company,
	PagedResponse_CompanyInvitation,
	PagedResponse_UserKYCModel,
	UserCompanyCreateUpdateReq,
	UserKYCModel,
	UserService,
} from "../apiClient";
import CompanyForm from "./CompanyForm";
import { useForm, SubmitHandler } from "react-hook-form";
import { CompanyInvitation } from "../apiClient/models/CompanyInvitation";
import { UserCompanyInvitationCreateReq } from "../apiClient/models/UserCompanyInvitationCreateReq";

interface EditCompanyProps {
	close?: () => void;
	filesBaseUrl: string;
	company: Company;
	onUpdated: (company: Company) => Promise<unknown>;
}

export default function EditCompany({ close = () => {}, filesBaseUrl, company, onUpdated }: EditCompanyProps) {
	const handleKeyDown = useCallback(
		(e: KeyboardEvent) => {
			if (e.key === "Escape" && close) {
				close();
			}
		},
		[close],
	);
	const handleOverlayClick = (e: React.MouseEvent<HTMLDivElement, MouseEvent>) => {
		if (e.target === e.currentTarget && close) {
			close();
		}
	};
	useEffect(() => {
		document.addEventListener("keydown", handleKeyDown);
		return () => {
			document.removeEventListener("keydown", handleKeyDown);
		};
	}, [handleKeyDown]);

	// const [members, setMembers] = useState<Member[]>([]);
	const [refreshCounter, setRefreshCounter] = useState(0);
	const [members, setMembers] = useState<PagedResponse_UserKYCModel>();
	const [invitations, setInvitations] = useState<PagedResponse_CompanyInvitation>();
	useEffect(() => {
		UserService.getCompanyMembersList().then(setMembers);
		UserService.getCompanyInvitationList().then(setInvitations);
	}, [company, refreshCounter]);

	const [membersScreen, setMembersScreen] = useState(false);

	const {
		register,
		handleSubmit,
		formState: { errors, isValid },
		reset,
	} = useForm<UserCompanyInvitationCreateReq>({});
	const onInviteMemberSubmit: SubmitHandler<UserCompanyInvitationCreateReq> = async (invitation) => {
		console.log("Inviting member", invitation);
		try {
			await UserService.postCompanyInvitation(invitation);
			// Handle member invitation logic here using the invitation object
			setRefreshCounter((prev) => prev + 1);
			reset();
		} catch {
			alert("Error inviting member");
		}
		setMembersScreen(false);
	};

	const onSubmit: SubmitHandler<UserCompanyCreateUpdateReq> = (data) => {
		UserService.putCompany(data)
			.then(onUpdated)
			.then(close)
			.catch((err) => {
				console.error(err);
				alert("Error updating company");
			});
	};

	const [isRemovingMember, setIsRemovingMember] = useState(false);
	const onRemoveMember = (user: UserKYCModel) => {
		if (isRemovingMember) {
			return;
		}
		setIsRemovingMember(true);
		UserService.deleteCompanyMembers(user.cognito_user_id)
			.then(() => {
				setRefreshCounter((prev) => prev + 1);
			})
			.catch((err) => {
				console.error(err);
				alert("Error removing member");
			})
			.finally(() => {
				setIsRemovingMember(false);
			});
	};

	const [isRemovingInvitation, setIsRemovingInvitation] = useState(false);
	const onRemoveInvitation = (invitation: CompanyInvitation) => {
		if (isRemovingInvitation) {
			return;
		}
		setIsRemovingInvitation(true);
		UserService.deleteCompanyInvitation(invitation.id)
			.then(() => {
				setRefreshCounter((prev) => prev + 1);
			})
			.catch((err) => {
				console.error(err);
				alert("Error removing invitation");
			})
			.finally(() => {
				setIsRemovingInvitation(false);
			});
	};

	return (
		<div className="popup-overlay" onClick={handleOverlayClick}>
			<div className="popup" onClick={(e) => e.stopPropagation()}>
				<>
					<img src={closeIcon} alt="Close icon" width={32} height={32} className="close" onClick={close} />
					<div className="heading">Edit Company</div>
					<div className="cl-area edo">
						<div className="container">
							<div className="container-in">
								<div className="space-20"></div>
								<CompanyForm id="company-form" company={company} filesBaseUrl={filesBaseUrl} onSubmit={onSubmit} />
								<div className="clr"></div>
								<div className="space-30"></div>
								<div className="col-12">
									<div className="head">
										Entity members{" "}
										<span className="headlink fr" onClick={() => setMembersScreen(true)}>
											Add members
										</span>
									</div>
								</div>
								<div className="space-15"></div>
								{members?.data.length === 0 && (
									<div className="col-12">
										<div className="twofactor">No Members</div>
									</div>
								)}
								{invitations?.data.map((invitation, index) => (
									<div key={index}>
										<div className="col-9 fl">
											<div className="twofactor">{invitation.email}</div>
										</div>
										<div className="col-3 fr">
											<span className="sty Sent">Sent</span>
											<button
												className="reset-button styremove fr"
												onClick={() => onRemoveInvitation(invitation)}
												disabled={isRemovingInvitation}
											>
												<img src={Remove} alt="" width="100%" height="100%" />
											</button>
										</div>
										<div className="clr"></div>
										<div className="space-15"></div>
									</div>
								))}
								{members?.data.map((member) => (
									<div key={member.cognito_user_id}>
										<div className="col-9 fl">
											<div className="twofactor">{member.email}</div>
										</div>
										<div className="col-3 fr">
											<span className="sty Accepted">Accept</span>
											<button
												className="reset-button styremove fr"
												onClick={() => onRemoveMember(member)}
												disabled={isRemovingMember}
											>
												<img src={Remove} alt="" width="100%" height="100%" />
											</button>
										</div>
										<div className="clr"></div>
										<div className="space-15"></div>
									</div>
								))}

								<div className="col-12"></div>
								<div className="space-30"></div>
								<div className="clr"></div>
								<div className="col-5 fl col-m-full col-mr-bottom-20">
									<Button text="CLOSE" call={() => close()} />
								</div>
								<div className="col-5 fr col-m-full">
									<Button text="SAVE" active type="submit" form="company-form" />
								</div>
								<div className="clr"></div>
							</div>
						</div>
					</div>
				</>
			</div>

			{membersScreen && (
				<div className="popup" onClick={(e) => e.stopPropagation()} id="add-member-popup">
					<div className="heading">Add members</div>
					<div className="cl-area edo">
						<div className="container">
							<div className="container-in">
								<div className="space-20"></div>
								<form onSubmit={handleSubmit(onInviteMemberSubmit)}>
									<div className="col-12">
										<div className="text-align-center">Specify the member's email address</div>
										<div className="space-15"></div>
										<div className="field">
											<input
												className={`textField center ${errors.email ? "error" : ""}`}
												placeholder="Enter email address"
												{...register("email", {
													required: true,
													pattern: /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/,
												})}
											/>
										</div>
										<div className="space-30"></div>
									</div>
									<div className="clr"></div>
									<div className="col-5 fl col-m-full col-mr-bottom-20">
										<Button text="CANCEL" call={() => setMembersScreen(false)} />
									</div>
									<div className="col-5 fr col-m-full">
										<Button text="SEND" active type="submit" disabled={!isValid} />
									</div>
									<div className="clr"></div>
								</form>
							</div>
						</div>
					</div>
				</div>
			)}
		</div>
	);
}
