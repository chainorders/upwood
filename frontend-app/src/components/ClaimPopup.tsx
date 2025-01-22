import { useState, useEffect, useCallback } from "react";
import Button from "./Button";
import closeIcon from "../assets/close.svg";
import greenTickIcon from "../assets/green-tick.svg";

interface KeyItem {
    tag: string;
    display: string;
}

interface Item {
    heading: string;
    list: KeyItem[];
}

interface ClaimPopupProps {
    config: Item;
    close?: () => void;
}

export default function ClaimPopup({ config, close }: ClaimPopupProps) {
    const [thankyou, setThankYou] = useState(false);
    const handleKeyDown = useCallback((e: KeyboardEvent) => {
        if (e.key === "Escape" && close) {
            close();
        }
    }, [close]);
    const showThankyou = () => {
        setThankYou(true);
    };
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
            {thankyou ? (
                <div className="popup" onClick={(e) => e.stopPropagation()}>
                    <img
                        src={closeIcon}
                        alt="Close icon"
                        // layout="intrinsic"
                        width={32}
                        height={32}
                        className="close"
                        onClick={close}
                    />
                    <div className="heading">Congratulations!</div>
                    <div className="message">
                        <img
                            src={greenTickIcon}
                            //   layout="intrinsic"
                            width={100}
                            height={100}
                        />
                        You have successfully invested in <span>5 shares</span> of the &quot;Oaktree House&quot; forest plantation.
                    </div>
                    <div className="space-30"></div>
                    <div className="container">
                        <div className="container-in">
                            <div className="col-12">
                                <Button text={'GO TO INVESTMENT PORTFOLIO'} link={''} active={true} />
                            </div>
                        </div>
                    </div>
                </div>
            ) : (
                <div className="popup" onClick={(e) => e.stopPropagation()}>
                    <img
                        src={closeIcon}
                        alt="Close icon"
                        // layout="intrinsic"
                        width={32}
                        height={32}
                        className="close"
                        onClick={close}
                    />
                    <div className="heading">{config.heading}</div>
                    <div className="cl-area">
                        <div className="container">
                            <div className="container-in">
                                {config.list.map((item, index) => (
                                    <div className={`col-6 fl ${config.list.length === 1 ? 'auto' : 'fl'} col-m-full`} key={index}>
                                        <div className="cl-in">
                                            <div className="tag">{item.tag}</div>
                                            <div className="display">{item.display}</div>
                                            <Button text={'CLAIM'} link={''} call={showThankyou} active={true} />
                                        </div>
                                    </div>
                                ))}
                                <div className="clr"></div>
                            </div>
                        </div>
                    </div>
                </div>
            )}
        </div>
    );
}
