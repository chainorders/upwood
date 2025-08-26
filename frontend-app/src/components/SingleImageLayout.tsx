interface Item {
	title?: string;
	header?: string;
	footer?: string;
	image?: string;
}
interface SingleImageProps {
	data: Item;
}
export default function SingleImageLayout({ data }: SingleImageProps) {
	return (
		<div className="multiimage">
			<div className="title">{data.title}</div>
			<div className="description">{data.header}</div>
			<div className="images">
				{data.image && (
					<div className="container">
						<div className="container-in">
							<div className="col-12">
								<div className="im">
									<img
										src={data.image}
										// layout="responsive"
										width={100}
										height={100}
									/>
								</div>
							</div>
						</div>
					</div>
				)}
			</div>
			<div className="description">{data.footer}</div>
		</div>
	);
}
