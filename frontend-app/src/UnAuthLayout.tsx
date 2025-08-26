import { Outlet } from "react-router";
import AuthTextSlider from "./components/AuthTextSlider";
import { SliderData } from "./pages/Login";
import logo from "./assets/logo.svg";

export default function UnAuthLayout() {
	const sliderData: SliderData[] = [
		{
			title: "Grow your wealth<br/>with sustainable<br/>investments",
			description:
				"On our platform you can becomea co-owner<br/>of real world forests by buying forest<br/>backed shares or invest in bonds backed<br/>up by forest plantations and carbon credits.",
		},
		{
			title: "Earn dividends<br/>or offset your<br/>emissions",
			description:
				"During the investment period you can earn<br/>dividends from forestry related income,<br/>carbon credit sales or choose to claim carbon<br/>credits to offset your emissions.",
		},
		{
			title: "Earn NFTs",
			description:
				"Every investor gets unique NFT<br/>collectibles representing forests planted.<br/>Every NFT collectible is unique and can<br/>be traded or held for additional benefits.",
		},
	];

	return (
		<>
			<AuthTextSlider data={sliderData} />
			<div className="auth-work">
				<div className="logo">
					<img src={logo} alt="" width={176} height={38} />
				</div>
				<div className="divider"></div>
				<Outlet />
			</div>
		</>
	);
}
