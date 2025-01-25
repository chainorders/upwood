import { useCallback, useEffect, useState } from "react";
import closeIcon from "../assets/close.svg";
import Button from "./Button";
import { Link } from "react-router";
import AccountCross from "../assets/account-not-protected.svg";
import AccountProtected from "../assets/account-protected.svg";
import editRow from "../assets/editRow.svg";
import saveRow from "../assets/saveRow.svg";
import Avatar from "../assets/Avatar.svg";
import OtpInput from "./OtpInput";
interface PopupProps {
    close?: () => void;
}
export default function EditProfile({ close = () => {} }: PopupProps) {
    const [full_name, setFullName] = useState("John Carter");
    const [full_name_edit, setFullNameEdit] = useState(false);
    const [email, setEmail] = useState("Jonh@gmail.com");
    const [email_edit, setEmailEdit] = useState(false);
    const [email_edit_otp_screen, setEmailEditOtpScreen] = useState(false);
    const [forgot_screen, setForgotScreen] = useState(false);
    const [enable_2fa_screen, set2FAScreen] = useState(false);
    const [disable_2fa_screen, set2FAScreenDisable] = useState(false);
    const handleKeyDown = useCallback((e: KeyboardEvent) => {
        if (e.key === "Escape" && close) {
            close();
        }
    }, [close]);
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
    return (
        <div className="popup-overlay" onClick={handleOverlayClick}>
            
                



                



                <div className="popup" onClick={(e) => e.stopPropagation()}>
                <>
                <img
                    src={closeIcon}
                    alt="Close icon"
                    width={32}
                    height={32}
                    className="close"
                    onClick={close}
                />
                <div className="heading">Edit profile</div>
                <div className="cl-area edo">
                    <div className="container">
                        <div className="container-in">
                            <div className="space-20"></div>
                            {/* <div className="col-12">
                                <div className="letter">J</div>
                                <div className="links">
                                    <span>Download pictures</span>
                                </div>
                            </div> */}
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
                                <div className="head">Personal data</div>
                            </div>
                            <div className="clr"></div>
                            <div className="col-4 fl col-m-full">
                                <div className="boxl">Full name</div>
                            </div>
                            <div className="col-8 fr col-m-full">
                                {full_name_edit ? 
                                <div>
                                    <input type="text" placeholder="Enter full name" value={full_name} className="boxt withedit fl" onChange={(e)=> setFullName(e.target.value)} />
                                    <img src={saveRow} alt="" className="saverow fr" onClick={()=> setFullNameEdit(false)} />
                                    <div className="clr"></div>
                                </div>
                                :
                                <div className="boxl lg">{full_name} <span className="fr"><img src={editRow} onClick={()=> setFullNameEdit(true)} /></span></div>
                                }
                            </div>
                            <div className="clr"></div>
                            <div className="space-15"></div>
                            <div className="clr"></div>
                            <div className="col-4 fl col-m-full">
                                <div className="boxl">Email</div>
                            </div>
                            <div className="col-8 fr col-m-full">
                                {email_edit ? 
                                <div>
                                    <input type="text" placeholder="Enter email" value={email} className="boxt withedit fl" onChange={(e)=> setEmail(e.target.value)} />
                                    <img src={saveRow} alt="" className="saverow fr" onClick={()=> {setEmailEdit(false); setEmailEditOtpScreen(true);}} />
                                    <div className="clr"></div>
                                </div>
                                :
                                <div className="boxl lg">{email} <span className="fr"><img src={editRow} onClick={()=> setEmailEdit(true)} /></span></div>
                                }
                            </div>
                            <div className="clr"></div>
                            <div className="space-30"></div>
                            <div className="col-12">
                                <div className="head">Change password <span className="headlink fr" onClick={()=> setForgotScreen(true)}>Forget Password</span></div>
                            </div>
                            <div className="clr"></div>
                            <div className="col-4 fl col-m-full col-mr-bottom-10">
                                <div className="boxl">Current password</div>
                            </div>
                            <div className="col-8 fr col-m-full">
                                <input type="password" placeholder="Enter current password" className="boxt error" />
                            </div>
                            <div className="clr"></div>
                            <div className="space-15"></div>
                            <div className="col-4 fl col-m-full col-mr-bottom-10">
                                <div className="boxl">New password</div>
                            </div>
                            <div className="col-8 fr col-m-full">
                                <input type="password" placeholder="Enter new password" className="boxt" />
                            </div>
                            <div className="clr"></div>
                            <div className="space-15"></div>
                            <div className="col-4 fl col-m-full col-mr-bottom-10">
                                <div className="boxl">Confirm password</div>
                            </div>
                            <div className="col-8 fr col-m-full">
                                <input type="password" placeholder="Confirm new password" className="boxt" />
                            </div>
                            <div className="clr"></div>
                            <div className="space-20"></div>
                            <div className="col-12">
                                <div className="head">Security 
                                    <span className="headlink fr showonmobile" onClick={()=>set2FAScreen(true)}>ENABLE</span>
                                    {/* <span className="headlink danger fr showonmobile" onClick={()=> set2FAScreenDisable(true)}>DISABLE</span> */}
                                </div>
                            </div>
                            <div className="space-15"></div>
                            <div className="col-12">
                            <span className="twofactor text-align-center-mob">
                                <img
                                    src={AccountCross}
                                    alt=""
                                />Account is not protected by 2FA</span> <span className="headlink fr hideonmobile" onClick={()=>set2FAScreen(true)}>ENABLE</span>
                            </div>
                            {/* <div className="col-12">
                            <span className="twofactor enabled text-align-center-mob">
                                <img
                                    src={AccountProtected}
                                    alt=""
                                    height={13}
                                />Account secured with 2FA</span> <span className="headlink danger fr hideonmobile" onClick={()=> set2FAScreenDisable(true)}>DISABLE</span>
                            </div> */}
                            <div className="clr"></div>
                            <div className="space-30"></div>
                            <div className="clr"></div>
                            <div className="col-5 fl col-m-full col-mr-bottom-20">
                                <Button text={'CLOSE'} link={''} active={false} call={()=> close()} />
                            </div>
                            <div className="col-5 fr col-m-full">
                                <Button text={'SAVE'} link={''} active={true} />
                            </div>
                            <div className="clr"></div>
                        </div>
                    </div>
                </div>
                </>
            </div>



            {email_edit_otp_screen && ( 
                <div className="popup" onClick={(e) => e.stopPropagation()}>
                <div className="heading">Change Email</div>
                <div className="cl-area edo">
                    <div className="container">
                        <div className="container-in">
                            <div className="space-20"></div>
                            <div className="col-12">
                                <div className="head text-align-center">
                                To change, you need to confirm your new email address
                                </div>
                                <div className="space-15"></div>
                                <div className="text-align-center">We’ve sent a code to Jonh23@gmail.com</div>
                                <div className="space-30"></div>
                                <OtpInput />
                            </div>
                            <div className="clr"></div>
                            <div className="col-5 fl col-m-full col-mr-bottom-20">
                                <Button text={'CANCEL'} link={''} active={false} call={()=> setEmailEditOtpScreen(false)} />
                            </div>
                            <div className="col-5 fr col-m-full">
                                <Button text={'CONFIRM'} link={''} active={true} />
                            </div>
                            <div className="clr"></div>
                        </div>
                    </div>
                </div>
                </div>  
                )}

                {forgot_screen && (
                <div className="popup" onClick={(e) => e.stopPropagation()}>
                <div className="heading">Forget Password</div>
                <div className="cl-area edo">
                    <div className="container">
                        <div className="container-in">
                            <div className="space-20"></div>
                            <div className="col-12">
                                <div className="head text-align-center">
                                Please check your email for create 
                                </div>
                                <div className="head text-align-center">
                                a new password
                                </div>
                                <div className="space-30"></div>
                            </div>
                        </div>
                    </div>
                </div>
                </div>
                )}

                {enable_2fa_screen && (
                <div className="popup" onClick={(e) => e.stopPropagation()}>
                <div className="heading">Security 2FA</div>
                <div className="cl-area edo">
                    <div className="container">
                        <div className="container-in">
                            <div className="space-20"></div>
                            <div className="col-12">
                                <div className="head text-align-center">
                                To activate 2FA security, you need to confirm via your email
                                </div>
                                <div className="space-15"></div>
                                <div className="text-align-center">We’ve sent a code to Jonh23@gmail.com</div>
                                <div className="space-30"></div>
                                <OtpInput />
                            </div>
                            <div className="clr"></div>
                            <div className="col-5 fl col-m-full col-mr-bottom-20">
                                <Button text={'CANCEL'} link={''} active={false} call={()=> set2FAScreen(false)} />
                            </div>
                            <div className="col-5 fr col-m-full">
                                <Button text={'ENABLE 2FA'} link={''} active={true} />
                            </div>
                            <div className="clr"></div>
                        </div>
                    </div>
                </div>
                </div>
                )}
                {disable_2fa_screen && (
                <div className="popup" onClick={(e) => e.stopPropagation()}>
                <div className="heading">Security 2FA</div>
                <div className="cl-area edo">
                    <div className="container">
                        <div className="container-in">
                            <div className="space-20"></div>
                            <div className="col-12">
                                <div className="head text-align-center">
                                To disable 2FA security, you need to confirm it via email
                                </div>
                                <div className="space-15"></div>
                                <div className="text-align-center">We’ve sent a code to Jonh23@gmail.com</div>
                                <div className="space-30"></div>
                                <OtpInput />
                            </div>
                            <div className="clr"></div>
                            <div className="col-5 fl col-m-full col-mr-bottom-20">
                                <Button text={'CANCEL'} link={''} active={false} call={()=> set2FAScreenDisable(false)} />
                            </div>
                            <div className="col-5 fr col-m-full">
                                <Button text={'DISABLE'} link={''} active={true} danger={true} />
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
