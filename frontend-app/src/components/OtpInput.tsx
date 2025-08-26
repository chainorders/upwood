import { useRef } from "react";
import { useController, Control, FieldError } from "react-hook-form";

interface OtpFormInputs {
	otp: string;
}

interface OtpInputProps {
	control: Control<OtpFormInputs>;
	name: "otp";
	length: number;
	error: FieldError | undefined;
}

export default function OtpInput({ control, name, length, error }: OtpInputProps) {
	const { field, fieldState } = useController({
		name,
		control,
		defaultValue: "",
	});
	const inputs = useRef<HTMLInputElement[]>([]);

	const handleChange = (element: HTMLInputElement, index: number): void => {
		if (isNaN(Number(element.value))) return;
		const newOtp = field.value.trim().split("");
		newOtp[index] = element.value;
		field.onChange(newOtp.join(""));

		if (element.value && index < 5) {
			inputs.current[index + 1].focus();
		}
	};

	return (
		<>
			<div
				id="input-divs"
				style={{
					display: "flex",
					flexDirection: "column",
					alignItems: "center",
					maxWidth: "296px",
					margin: "auto",
					marginBottom: "50px",
				}}
			>
				<div style={{ display: "flex", justifyContent: "center" }}>
					{new Array(length).fill("").map((_, index) => (
						<input
							className={`inputotp ${fieldState.invalid ? "error" : ""}`}
							key={index}
							type="text"
							value={field.value[index] || ""}
							maxLength={1}
							style={{ width: "50px", height: "50px", textAlign: "center" }}
							onChange={(e) => handleChange(e.target, index)}
							onFocus={(e) => e.target.select()}
							ref={(ref) => (inputs.current[index] = ref!)}
						/>
					))}
				</div>
				{error && (
					<p className="error" style={{ textAlign: "center", marginTop: "10px" }}>
						{error.message?.toString()}
					</p>
				)}
			</div>
		</>
	);
}
