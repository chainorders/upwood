import { NewsArticle } from "../apiClient";

interface NewsCardProps {
	article: NewsArticle;
}

export default function NewsCard({ article: item }: NewsCardProps) {
	return (
		<>
			<div className="news-card">
				<div className="container">
					<div className="container-in">
						<div className="col-12">
							<div className="image">
								<img src={item.image_url} />
								<div className="caption">{item.label}</div>
							</div>
						</div>
					</div>
				</div>
				<div className={`container`}>
					<div className="container-in">
						<div className="col-12">
							<div className="news-name">{item.title}</div>
							<div className="news-description">{item.content}</div>
						</div>
					</div>
					<div className="container-in text-align-right">
						<div className="col-12">
							<a href={item.article_url} target="_blank" className="read">
								READ FULL ARTICLE
							</a>
						</div>
					</div>
				</div>
			</div>
		</>
	);
}
