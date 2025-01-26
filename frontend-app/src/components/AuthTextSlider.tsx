import { useState, useEffect, useCallback, JSX } from "react";

interface KeyItem {
	title: string;
	description: string;
}

interface DataProp {
	data: KeyItem[];
}

export default function AuthTextSlider({ data }: DataProp): JSX.Element {
	// State to keep track of the current image index
	const [currentIndex, setCurrentIndex] = useState<number>(0);

	// State to determine if the image is being hovered over
	const [isHovered, setIsHovered] = useState<boolean>(false);

	// useCallback to prevent the recreation of the nextSlide function on every render
	const nextSlide = useCallback((): void => {
		setCurrentIndex((prevIndex) => (prevIndex + 1) % data.length);
	}, [data.length]);

	// Function to show the previous slide
	const prevSlide = (): void => {
		setCurrentIndex((prevIndex) => (prevIndex - 1 + data.length) % data.length);
	};

	// useEffect hook to handle automatic slide transition
	useEffect(() => {
		// Start interval for automatic slide change if not hovered
		if (!isHovered) {
			const interval = setInterval(() => {
				nextSlide();
			}, 5000);

			// Cleanup the interval on component unmount
			return () => {
				clearInterval(interval);
			};
		}
	}, [isHovered, nextSlide]); // Add nextSlide to dependencies

	// Handle mouse over event
	const handleMouseOver = (): void => {
		setIsHovered(true);
	};

	// Handle mouse leave event
	const handleMouseLeave = (): void => {
		setIsHovered(false);
	};

	return (
		<div className="authtextslider">
			<div className="dividerwhite"></div>
			<div onMouseOver={handleMouseOver} onMouseLeave={handleMouseLeave}>
				<div
					className="title"
					dangerouslySetInnerHTML={{
						__html: data[currentIndex].title,
					}}
				></div>
				<div
					className="description"
					dangerouslySetInnerHTML={{
						__html: data[currentIndex].description,
					}}
				></div>
			</div>
			<div className="bullets">
				<button onClick={prevSlide} className="orv left"></button>
				{data.map((_, index) => (
					<button key={index} className={`bullet ${index === currentIndex ? "active" : ""}`}></button>
				))}
				<button onClick={nextSlide} className="orv right"></button>
			</div>
		</div>
	);
}
