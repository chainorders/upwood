"use client"
import { useState } from "react";
import AuthTextSlider from "../components/AuthTextSlider";
import Button from "../components/Button";
import logo from '../assets/logo.svg';
import Vector from '../assets/Vector.svg';
import greenTickIcon from "../assets/green-tick.svg";
export default function ForgotPassword() {
    const [forgotSuccess, setforgotSuccess] = useState(false);
  const whenconfirmbuttonhit = () => {
    setforgotSuccess(true)
  }
  const sliderdata = [
      {
          title : "Grow your wealth<br/>with sustainable<br/>investments",
          description : "On our platform you can becomea co-owner<br/>of real world forests by buying forest<br/>backed shares or invest in bonds backed<br/>up by forest plantations and carbon credits."
      },
      {
          title : "Earn dividends<br/>or offset your<br/>emissions",
          description : "During the investment period you can earn<br/>dividends from forestry related income,<br/>carbon credit sales or choose to claim carbon<br/>credits to offset your emissions."
      },
      {
          title : "Earn NFTs",
          description : "Every investor gets unique NFT<br/>collectibles representing forests planted.<br/>Every NFT collectible is unique and can<br/>be traded or held for additional benefits."
      }
  ]
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

        {forgotSuccess ?
        <div>
          <div className="image">
          <img
                src={greenTickIcon}
                alt=""
            />
          </div>
          <div className="center-text biger bold">Success</div>
          <div className="space-30"></div>
          <div className="center-text">Please check your email<br />
          for create a new password</div>
          <div className="space-30"></div>
          <div className="center-text big">Can&apos;t get email? <a href="">Resubmit</a></div>
          <div className="space-30"></div>
          <div>
              <Button icon={Vector} text ={'BACK TO LOGIN'} link={'/login'} active={false}  />
          </div>
          <div className='divider'></div>
        </div>
        : 
        <div>
          <div className='heading'>Forgot your password?</div>
          <div className="field mrbottom">
              <p className="text-align-right error">Email is not recognized in our system, please contact support!</p>
              <input type="text" placeholder="Enter email address" className="textField style2 error"  />
          </div>
          <div className="field mrbottom">
              <Button style={'style2'} text ={'CONFIRM'} link={''} active={true} call={whenconfirmbuttonhit}  />
          </div>
          <div className="left-text">
          If your email will be recognized in the system, you will recieve further instructions to reset your password in the email. If you don&apos;t see an email from Upwood, please check your spam folder. If you haven&apos;t recieved e-mail or forgot your e-mail address please contact Upwood support. 
          </div>
          <div className="space-30"></div>
          <div className="container">
              <div className="container-in">
                <div className="col-6 fl col-m-full col-mr-bottom-20">
                    <Button icon={Vector} text ={'BACK TO LOGIN'} link={'/login'} active={false} />
                </div>
                <div className="col-6 fr col-m-full">
                    <Button style={'style3'} text ={'CONTACT SUPPORT'} link={''} active={false} />
                </div>
                <div className="clr"></div>
              </div>  
              
          </div>
          <div className='divider'></div>
        </div>
        }
      </div>
    </>
  );
}