import { useCallback, useEffect, useState } from "react";
import closeIcon from "../assets/close.svg";
import Button from "./Button";
import editRow from "../assets/editRow.svg";
import saveRow from "../assets/saveRow.svg";
import Avatar from "../assets/Avatar.svg";
import Remove from "../assets/remove.svg";
import OtpInput from "./OtpInput";
interface EditCompanyProps {
	close?: () => void;
}
interface Member {
	email: string;
	status: string;
}
export default function EditCompany({ close = () => {} }: EditCompanyProps) {
	const [companyName, setCompanyName] = useState("SIA Upwood");
	const [companyNameEdit, setCompanyNameEdit] = useState(false);
	const [email, setEmail] = useState("Jonh@gmail.com");
	const [emailEdit, setEmailEdit] = useState(false);
	const [emailEditOtpScreen, setEmailEditOtpScreen] = useState(false);
	const [membersScreen, setMembersScreen] = useState(false);
	const [memberemail, setMemberEmail] = useState("");
	const [members, setMembers] = useState<Member[]>([]);
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
	const handleAddMember = () => {
		setMembers((prevMembers) => [
			...prevMembers,
			{
				email: memberemail,
				status: "Sent",
			},
		]);
		setMemberEmail("");
		setMembersScreen(false);
	};
	return (
		<div className="popup-overlay" onClick={handleOverlayClick}>
			<div className="popup" onClick={(e) => e.stopPropagation()}>
				<>
					<img src={closeIcon} alt="Close icon" width={32} height={32} className="close" onClick={close} />
					<div className="heading">Edit company profile</div>
					<div className="cl-area edo">
						<div className="container">
							<div className="container-in">
								<div className="space-20"></div>
								<div className="col-12">
									<div className="text-align-center">
										<img src={Avatar} alt="" className="Avatar" />
									</div>
									<div className="space-15"></div>
									<div className="links">
										<span>CHANGE</span>
										<span className="danger">DELETE</span>
									</div>
								</div>
								<div className="space-20"></div>
								<div className="col-12">
									<div className="head">Company data</div>
								</div>
								<div className="clr"></div>
								<div className="col-4 fl col-m-full">
									<div className="boxl">Company name</div>
								</div>
								<div className="col-8 fr col-m-full">
									{companyNameEdit ? (
										<div>
											<input
												type="text"
												placeholder="Enter full name"
												value={companyName}
												className="boxt withedit fl"
												onChange={(e) => setCompanyName(e.target.value)}
											/>
											<img src={saveRow} alt="" className="saverow fr" onClick={() => setCompanyNameEdit(false)} />
											<div className="clr"></div>
										</div>
									) : (
										<div className="boxl lg">
											{companyName}{" "}
											<span className="fr">
												<img src={editRow} onClick={() => setCompanyNameEdit(true)} />
											</span>
										</div>
									)}
								</div>
								<div className="clr"></div>
								<div className="space-15"></div>
								<div className="clr"></div>
								<div className="col-4 fl col-m-full">
									<div className="boxl">Company email</div>
								</div>
								<div className="col-8 fr col-m-full">
									{emailEdit ? (
										<div>
											<input
												type="text"
												placeholder="Enter email"
												value={email}
												className="boxt withedit fl"
												onChange={(e) => setEmail(e.target.value)}
											/>
											<img
												src={saveRow}
												alt=""
												className="saverow fr"
												onClick={() => {
													setEmailEdit(false);
													setEmailEditOtpScreen(true);
												}}
											/>
											<div className="clr"></div>
										</div>
									) : (
										<div className="boxl lg">
											{email}{" "}
											<span className="fr">
												<img src={editRow} onClick={() => setEmailEdit(true)} />
											</span>
										</div>
									)}
								</div>
								<div className="clr"></div>
								<div className="space-15"></div>
								<div className="clr"></div>
								<div className="col-4 fl col-m-full">
									<div className="boxl">Registration №</div>
								</div>
								<div className="col-8 fr col-m-full">
									<div className="boxl lg">12343678</div>
								</div>
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
								{members.length === 0 && (
									<div className="col-12">
										<div className="twofactor">Members absent</div>
									</div>
								)}
								{members.length !== 0 && (
									<>
										{members.map((item, index) => (
											<div key={index}>
												<div className="col-9 fl">
													<div className="twofactor">{item.email}</div>
												</div>
												<div className="col-3 fr">
													<span className={`sty ${item.status}`}>{item.status}</span>
													<img src={Remove} alt="" className="styremove fr" />
												</div>
												<div className="clr"></div>
												<div className="space-15"></div>
											</div>
										))}
									</>
								)}

								<div className="col-12"></div>
								<div className="space-30"></div>
								<div className="clr"></div>
								<div className="col-5 fl col-m-full col-mr-bottom-20">
									<Button text="CLOSE" call={() => close()} />
								</div>
								<div className="col-5 fr col-m-full">
									<Button text="SAVE" active />
								</div>
								<div className="clr"></div>
							</div>
						</div>
					</div>
				</>
			</div>

			{emailEditOtpScreen && (
				<div className="popup" onClick={(e) => e.stopPropagation()}>
					<div className="heading">Change Email</div>
					<div className="cl-area edo">
						<div className="container">
							<div className="container-in">
								<div className="space-20"></div>
								<div className="col-12">
									<div className="head text-align-center">To change, you need to confirm your new email address</div>
									<div className="space-15"></div>
									<div className="text-align-center">We’ve sent a code to Jonh23@gmail.com</div>
									<div className="space-30"></div>
									<OtpInput />
								</div>
								<div className="clr"></div>
								<div className="col-5 fl col-m-full col-mr-bottom-20 ">
									<Button text="CANCEL" call={() => setEmailEditOtpScreen(false)} />
								</div>
								<div className="col-5 fr col-m-full">
									<Button text="CONFIRM" active />
								</div>
								<div className="clr"></div>
							</div>
						</div>
					</div>
				</div>
			)}

			{membersScreen && (
				<div className="popup" onClick={(e) => e.stopPropagation()}>
					<div className="heading">Add members</div>
					<div className="cl-area edo">
						<div className="container">
							<div className="container-in">
								<div className="space-20"></div>
								<div className="col-12">
									<div className="text-align-center">Specify the member's email address</div>
									<div className="space-15"></div>
									<div className="field">
										<input
											className="textField center"
											placeholder="Enter email address"
											value={memberemail}
											onChange={(e) => setMemberEmail(e.target.value)}
										/>
									</div>
									<div className="space-30"></div>
								</div>
								<div className="clr"></div>
								<div className="col-5 fl col-m-full col-mr-bottom-20">
									<Button text="CANCEL" call={() => setMembersScreen(false)} />
								</div>
								<div className="col-5 fr col-m-full">
									<Button text="SEND" active call={() => handleAddMember()} />
								</div>
								<div className="clr"></div>
							</div>
						</div>
					</div>
				</div>
			)}
		</div>
	);
}
