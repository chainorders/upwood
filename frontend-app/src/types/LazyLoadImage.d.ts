declare module "react-lazy-load-image-component" {
	import { ComponentType, ImgHTMLAttributes, ReactElement } from "react";

	export interface LazyLoadImageProps extends ImgHTMLAttributes<HTMLImageElement> {
		onLoad?: (event: Event) => void;
		afterLoad?: () => void;
		beforeLoad?: () => void;
		delayMethod?: "throttle" | "debounce";
		delayTime?: number;
		effect?: string;
		placeholder?: ReactElement;
		placeholderSrc?: string;
		threshold?: number;
		useIntersectionObserver?: boolean;
		visibleByDefault?: boolean;
		wrapperClassName?: string;
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		wrapperProps?: Record<string, any>;
	}

	export const LazyLoadImage: ComponentType<LazyLoadImageProps>;
	export default LazyLoadImage;
}
