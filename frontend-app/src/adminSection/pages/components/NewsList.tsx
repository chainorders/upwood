import {
	Table,
	TableBody,
	TableCell,
	TableContainer,
	TableHead,
	TableRow,
	Paper,
	Button,
	ButtonGroup,
	Box,
	Typography,
	Pagination,
} from "@mui/material";
import { NewsArticle } from "../../../apiClient/models/NewsArticle";
import { useNavigate } from "react-router";
import { useEffect, useState } from "react";
import { PagedResponse_NewsArticle, UserCommunicationService } from "../../../apiClient";
import AddNewsArticlePopup from "./AddNewsArticlePopup";

export default function NewsList() {
	const navigate = useNavigate();
	const [articles, setArticles] = useState<PagedResponse_NewsArticle>();
	const [articlesPage, setArticlesPage] = useState<number>(0);
	const [articlesPerPage, setArticlesPerPage] = useState<number>(10);
	const [refreshCounter, setRefreshCounter] = useState(0);
	const [open, setOpen] = useState(false);

	useEffect(() => {
		UserCommunicationService.getNewsArticlesList(articlesPage, articlesPerPage).then(setArticles);
	}, [articlesPage, articlesPerPage, refreshCounter]);

	const handleDelete = (id: string) => {
		UserCommunicationService.deleteAdminNewsArticles(id).then(() => {
			setRefreshCounter((c) => c + 1);
		});
	};

	const handleClickOpen = () => {
		setOpen(true);
	};

	const handleClose = () => {
		setOpen(false);
	};

	const handleRefresh = () => {
		setRefreshCounter((c) => c + 1);
		handleClose();
	};

	return (
		<Box>
			<Box display="flex" justifyContent="space-between" alignItems="center">
				<Typography variant="h4" gutterBottom>
					News Articles
				</Typography>
				<Button variant="contained" color="primary" onClick={handleClickOpen}>
					Add New Article
				</Button>
			</Box>
			<TableContainer component={Paper}>
				<Table>
					<TableHead>
						<TableRow>
							<TableCell>Title</TableCell>
							<TableCell>Label</TableCell>
							<TableCell>Content</TableCell>
							<TableCell>Image URL</TableCell>
							<TableCell>Article URL</TableCell>
							<TableCell>Created At</TableCell>
							<TableCell>Order Index</TableCell>
							<TableCell>Actions</TableCell>
						</TableRow>
					</TableHead>
					<TableBody>
						{articles?.data.map((article) => (
							<TableRow key={article.id}>
								<TableCell>{article.title}</TableCell>
								<TableCell>{article.label}</TableCell>
								<TableCell>{article.content}</TableCell>
								<TableCell>
									<a href={article.image_url} target="_blank" rel="noopener noreferrer">
										{article.image_url}
									</a>
								</TableCell>
								<TableCell>
									<a href={article.article_url} target="_blank" rel="noopener noreferrer">
										{article.article_url}
									</a>
								</TableCell>
								<TableCell>{article.created_at}</TableCell>
								<TableCell>{article.order_index}</TableCell>
								<TableCell>
									<ButtonGroup size="small">
										<Button variant="outlined" color="secondary" onClick={() => handleDelete(article.id)}>
											Delete
										</Button>
									</ButtonGroup>
								</TableCell>
							</TableRow>
						))}
					</TableBody>
				</Table>
			</TableContainer>
			<Pagination
				count={articles?.page_count || 0}
				page={(articles?.page || 0) + 1}
				onChange={(e, page) => setArticlesPage(page - 1)}
				style={{ marginTop: 20, display: "flex", justifyContent: "center" }}
			/>
			<AddNewsArticlePopup open={open} onClose={handleClose} onRefresh={handleRefresh} />
		</Box>
	);
}
