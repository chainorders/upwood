import { useEffect, useState } from "react";
import Button from "../components/Button";
import PageHeader from "../components/PageHeader";
import { User } from "../lib/user";
import { PagedResponse_Guide, UserCommunicationService } from "../apiClient";
import { useForm } from "react-hook-form";

interface SupportProps {
	user: User;
}

export default function Support({ user }: SupportProps) {
	const [showAll, setShowAll] = useState(false);
	const [guides, setGuides] = useState<PagedResponse_Guide>();
	useEffect(() => {
		UserCommunicationService.getGuidesList(0, 20).then(setGuides);
	}, [user]);

	const displayGuides = showAll ? guides?.data : guides?.data.slice(0, 6);
	const showAllDisabled = (guides?.data.length || 0) <= 6;
	const {
		register,
		handleSubmit,
		reset,
		formState: { isValid, isSubmitting },
	} = useForm<{ message: string }>();
	const onSubmit = async (data: { message: string }) => {
		await UserCommunicationService.postSupportQuestions(data.message);
		reset();
	};

	return (
		<>
			<div className="clr"></div>
			<div className="support">
				<PageHeader user={user} parts={[{ name: "Active Projects" }]} />
				<div className="outerboxshadow">
					<div className="container">
						<div className="container-in">
							<div className="col-8 col-m-full fl">
								<div className="heading">Platform how to guides</div>
							</div>
							<div className="col-4 text-align-right fr hideonmobile">
								<span
									className={`seeall ${showAllDisabled && "disabled"}`}
									onClick={() => !showAllDisabled && setShowAll(!showAll)}
								>
									{showAll ? "SEE LESS" : "SEE ALL"}
								</span>
							</div>

							<div className="clr"></div>
							{displayGuides?.map((guide, index) => (
								<div className="col-4 col-m-full fl" key={index}>
									<div className="linkbox">
										<a href={guide.guide_url} target="_blank" rel="noreferrer">
											<div className="title">{guide.title}</div>
											<div className="description">{guide.label}</div>
										</a>
									</div>
								</div>
							))}
							<div className="clr"></div>
							<div className="col-12 showonmobile text-align-center">
								<span
									className={`seeall ${showAllDisabled && "disabled"}`}
									onClick={() => !showAllDisabled && setShowAll(!showAll)}
								>
									{showAll ? "SEE LESS" : "SEE ALL"}
								</span>
							</div>
							<div className="space-20 showonmobile"></div>
						</div>
					</div>
				</div>
				<div className="space-30"></div>
				<div className="outerboxshadow" id="supportQuerySection">
					<div className="container">
						<div className="container-in">
							<form onSubmit={handleSubmit(onSubmit)}>
								<div className="col-12">
									<div className="heading">Write to support</div>
									<div className="sub">
										Our support hours are 10:00 to 16:00 (UTC +2) Mon to Fri. Please expect an answer during those times.
									</div>
								</div>
								<div className="space-20"></div>
								<div className="col-12">
									<div className="field">
										<textarea
											id="supportQueryInput"
											placeholder="Type your message here..."
											rows={10}
											className="textareaField style2"
											{...register("message", { required: true })}
										></textarea>
									</div>
								</div>
								<div className="clr"></div>
								<div className="space-20"></div>
								<div className="col-3 col-m-full fr">
									<Button text="SUBMIT" active type="submit" disabled={!isValid} loading={isSubmitting} />
								</div>
								<div className="clr"></div>
								<div className="space-20"></div>
							</form>
						</div>
					</div>
				</div>
			</div>
		</>
	);
}
