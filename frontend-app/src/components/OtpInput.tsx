import React, { useState, useRef } from 'react';

const OtpInput: React.FC = () => {
    // Initialize OTP state with an array of strings, each initialized to an empty string
    const [otp, setOtp] = useState<string[]>(new Array(4).fill(""));
    // Use useRef to hold a reference to the input elements
    const inputs = useRef<HTMLInputElement[]>([]);

    // Function to handle changes to each input
    const handleChange = (element: HTMLInputElement, index: number): void => {
        if (isNaN(Number(element.value))) return; // Ensure that the input is a number
        const newOtp = [...otp];
        newOtp[index] = element.value;
        setOtp(newOtp);

        // Move focus to next input if the value is not empty and there is a next input element
        if (element.value && index < otp.length - 1) {
            inputs.current[index + 1].focus();
        }
    };

    return (
        <div style={{ display: 'flex', justifyContent: 'space-between', maxWidth: '296px', margin:'auto', marginBottom:'50px' }}>
            {otp.map((data, index) => (
                <input
                    className='inputotp error'
                    key={index}
                    type="text"
                    value={data}
                    maxLength={1}
                    style={{ width: '64px', height: '64px', textAlign: 'center' }}
                    onChange={e => handleChange(e.target, index)}
                    onFocus={e => e.target.select()}
                    ref={ref => inputs.current[index] = ref!} // Assert that ref is not null
                />
            ))}
        </div>
    );
};

export default OtpInput;
