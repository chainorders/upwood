import { useCallback, useEffect } from "react";
import { SubmitHandler } from "react-hook-form";
import closeIcon from "../assets/close.svg";
import Button from "./Button";
import { Company, UserCompanyCreateUpdateReq, UserService } from "../apiClient";
import CompanyForm from "./CompanyForm";
interface PopupProps {
	onCreated: (company: Company) => Promise<unknown>;
	close?: () => void;
	filesBaseUrl: string;
}

export default function CreateCompany({ close = () => {}, filesBaseUrl, onCreated }: PopupProps) {
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

	const onSubmit: SubmitHandler<UserCompanyCreateUpdateReq> = (data) => {
		UserService.postCompany(data)
			.then(onCreated)
			.then(close)
			.catch((err) => {
				console.error(err);
				alert("Error creating company");
			});
	};

	return (
		<div className="popup-overlay" onClick={handleOverlayClick}>
			<div className="popup" onClick={(e) => e.stopPropagation()}>
				<>
					<img src={closeIcon} alt="Close icon" width={32} height={32} className="close" onClick={close} />
					<div className="heading">Create Company</div>
					<div className="cl-area edo">
						<div className="container">
							<div className="container-in">
								<CompanyForm
									id="company-form"
									filesBaseUrl={filesBaseUrl}
									onSubmit={onSubmit}
									profilePictureDefaultState="upload"
								/>
								<div className="clr"></div>
								<div className="space-30"></div>
								<div className="clr"></div>
								<div className="col-5 fl col-m-full col-mr-bottom-20">
									<Button text="CLOSE" call={() => close()} />
								</div>
								<div className="col-5 fr col-m-full">
									<Button type="submit" text="SAVE" form="company-form" active />
								</div>
								<div className="clr"></div>
							</div>
						</div>
					</div>
				</>
			</div>
		</div>
	);
}
