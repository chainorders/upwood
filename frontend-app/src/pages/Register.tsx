"use client"
import { useState } from "react";
import AuthTextSlider from "../components/AuthTextSlider";
import Button from "../components/Button";
import logo from '../assets/logo.svg';
import greenTickIcon from "../assets/green-tick.svg";
export default function Register() {
    const [registerSuccess, setregisterSuccess] = useState(false);
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

        {registerSuccess ?
        <div>
          <div className="image">
          <img
                src={greenTickIcon}
                alt=""
            />
          </div>
          <div className="center-text biger bold">Let’s verify your identity</div>
          <div className="space-30"></div>
          <div className="center-text big">To finish your registration you need to create a digital wallet<br/>and verify your identity in few simple steps.</div>
          <div className="space-30"></div>
          <div className="center-text">Concordium wallet extension will be downloaded to your<br/>device or browser</div>
          <div className="space-30"></div>
          <div className="center-text big">Once you’re done you will be logged in Upwood platform<br/><a href="">I understand, let’s continue</a></div>
          <div className='divider'></div>
        </div>
        : 
        <div>
          <div className='heading'>User Registration</div>
          <div className="field mrbottom">
              <input type="text" placeholder="Enter your email" className="textField style2"  />
          </div>
          <div className="field mrbottom">
              <p className="text-align-right error">error will display here.</p>
              <input type="text" placeholder="Enter your password" className="textField style2 error"  />
          </div>
          <div className="field mrbottom">
              <input type="text" placeholder="Repeat your password" className="textField style2"  />
          </div>
          <div className="field mrbottom">
              <div className="lab">Desired investment amount € (optional)</div>  
              <input type="text" placeholder="Enter amount €" className="textField style2"  />
          </div>
          <div className="field mrbottom">
            <div className="checkbox">
              <input type="checkbox" id='terms' />
              <label htmlFor="terms" className="no-center-mobile"><span>I accept Terms & condition....</span></label>
            </div>
          </div>
          <div className="field mrbottom">
              <Button style={'style3'} text ={'CONTINUE WITH DIGITAL WALLET CREATION'} link={''} active={false} call={() => setregisterSuccess(true)}   />
          </div>
          <div className="or"><span>OR</span></div>
          <div className="field mrbottom">
              <Button style={'style3'} text ={'CONNECT IF YOU HAVE CONCORDIUM WALLET'} link={''} active={false}   />
          </div>
          <div className='divider'></div>
        </div>
        }
      </div>
    </>
  );
}