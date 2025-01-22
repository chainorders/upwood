import { useState } from "react";
import Button from "../components/Button";
import { Link, useLocation, useNavigate } from "react-router";
import AuthTextSlider from "../components/AuthTextSlider";
import logo from '../assets/logo.svg';
import { useTitle } from "../components/useTitle";
import { ApiUser } from "../apiClient";

export default function Login(props: { setUser: (user: ApiUser) => void }) {
    useTitle("Login");
    const navigate = useNavigate();
    const location = useLocation();
    const [inviteSuccess, setinviteSuccess] = useState(false);
    const whenLoginButtonHit = () => {
        // todo: remove this line
        props.setUser({ cognito_user_id: '123' } as ApiUser);
        navigate(location.state?.from ? location.state.from : '/projects/active');
    }

    const whenRequestInvitationButtonHit = () => { }
    const backToLoginButtonHit = () => {
        setinviteSuccess(false)
    }

    const sliderdata = [
        {
            title: "Grow your wealth<br/>with sustainable<br/>investments",
            description: "On our platform you can becomea co-owner<br/>of real world forests by buying forest<br/>backed shares or invest in bonds backed<br/>up by forest plantations and carbon credits."
        },
        {
            title: "Earn dividends<br/>or offset your<br/>emissions",
            description: "During the investment period you can earn<br/>dividends from forestry related income,<br/>carbon credit sales or choose to claim carbon<br/>credits to offset your emissions."
        },
        {
            title: "Earn NFTs",
            description: "Every investor gets unique NFT<br/>collectibles representing forests planted.<br/>Every NFT collectible is unique and can<br/>be traded or held for additional benefits."
        }
    ];

    return (
        <>
            <AuthTextSlider data={sliderdata} />
            <div className="auth-work">
                <div className='logo'>
                    <img
                        src={logo}
                        alt=""
                        width={176}
                        height={38}
                    // priority
                    />
                </div>
                <div className='divider'></div>

                {inviteSuccess ?
                    <div>
                        <div className="image">
                            <img
                                src="/Group-1000003068.svg"
                                alt="Description of the image"
                                //   layout="responsive"
                                width={100}
                                height={100}
                            />
                        </div>
                        <div className="center-text biger bold">Success</div>
                        <div className="space-30"></div>
                        <div className="center-text">We have recieved your<br />
                            invitation request!</div>
                        <div className="space-30"></div>
                        <div className="center-text big">Please check your e-mail</div>
                        <div className="space-30"></div>
                        <div>
                            <Button icon={'/Vector.svg'} text={'BACK TO LOGIN'} link={''} active={false} call={backToLoginButtonHit} />
                        </div>
                        <div className='divider'></div>
                    </div>
                    :
                    <div>
                        <div className='heading'>User login</div>
                        <div className="field mrbottom">
                            <input type="text" placeholder="Enter email address" className="textField style2" />
                        </div>
                        <div className="field mrbottom">
                            <p className="text-align-right error">Your wallet balance is not sufficient to buy shares. Please add funds to your wallet.</p>
                            <input type="text" placeholder="Enter your password" className="textField style2 error" />
                        </div>
                        <div>
                            <Button style={'style2'} text={'LOG IN'} link={''} active={true} call={whenLoginButtonHit} />
                        </div>
                        <div className="forgotlink"><Link to="/forgot-password">Forgot Password</Link></div>
                        <div className='divider'></div>
                        <div className='heading'>Not a user? Request invitation</div>
                        <div className="field mrbottom">
                            <input type="text" placeholder="Enter email address" className="textField style2" />
                        </div>
                        <div className="field mrbottom">
                            <div className="checkbox">
                                <input type="checkbox" id='terms' />
                                <label htmlFor="terms" className="no-center-mobile"><span>By entering your email you agree to receive marketing communications from SIA Upwood. You can unsubscribe at any time. For more information, see our <a href=''>Privacy Policy</a>.</span></label>
                            </div>
                        </div>
                        <div>
                            <Button style={'style2'} text={'REQUEST INVITATION'} link={''} active={true} disabled={true} call={whenRequestInvitationButtonHit} />
                        </div>
                    </div>
                }
            </div>
        </>
    );
}