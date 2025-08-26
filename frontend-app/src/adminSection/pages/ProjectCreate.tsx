import { useForm } from "react-hook-form";
import { ForestProject, ForestProjectService, ForestProjectState } from "../../apiClient";
import { v4 as uuidv4 } from "uuid";
import {
	Button,
	TextField,
	Select,
	MenuItem,
	InputLabel,
	FormControl,
	Box,
	Breadcrumbs,
	Typography,
	Divider,
	Paper,
	Grid,
	Container,
} from "@mui/material";
import { formatDate } from "../../lib/conversions";
import { Link, useNavigate } from "react-router";
import ImageUploader from "../components/ImageUploader";
import { adminUploadImage } from "../libs/utils";
import useCommonStyles from "../../theme/useCommonStyles";
import ForestIcon from "@mui/icons-material/Folder";
import ImageIcon from "@mui/icons-material/Image";
import DescriptionIcon from "@mui/icons-material/Description";
import InsertChartIcon from "@mui/icons-material/InsertChart";
import LocationOnIcon from "@mui/icons-material/LocationOn";
import HomeIcon from "@mui/icons-material/Home";

export default function ProjectCreate({ fileBaseUrl }: { fileBaseUrl: string }) {
	const {
		register,
		handleSubmit,
		formState: { errors },
		setValue,
		watch,
	} = useForm<ForestProject>();
	const navigate = useNavigate();
	const styles = useCommonStyles();

	const onSubmit = (data: ForestProject) => {
		const now = new Date();
		data.created_at = formatDate(now);
		data.updated_at = formatDate(now);
		data.id = uuidv4();

		ForestProjectService.postAdminForestProjects(data)
			.then(() => {
				alert("Project created successfully");
				navigate("/admin/projects");
			})
			.catch(() => {
				alert("Failed to create project");
			});
	};

	return (
		<>
			<Breadcrumbs aria-label="breadcrumb" sx={{ mb: 2 }}>
				<Link to="/admin" style={styles.breadcrumbLink}>
					<HomeIcon sx={{ mr: 0.5 }} fontSize="small" />
					Admin
				</Link>
				<Link to="/admin/projects" style={styles.breadcrumbLink}>
					<ForestIcon sx={{ mr: 0.5 }} fontSize="small" />
					Projects
				</Link>
				<Typography color="text.primary" sx={{ display: "flex", alignItems: "center" }}>
					Create Project
				</Typography>
			</Breadcrumbs>

			<Box sx={styles.sectionHeader}>
				<ForestIcon />
				<Typography variant="h4" gutterBottom>
					Create New Forest Project
				</Typography>
			</Box>

			<Container maxWidth="lg" disableGutters>
				<Paper component="form" onSubmit={handleSubmit(onSubmit)} sx={styles.formContainer}>
					<Box sx={styles.formSection}>
						<Box sx={styles.formSectionHeader}>
							<ForestIcon />
							<Typography variant="h6">Basic Information</Typography>
						</Box>
						<Grid container spacing={3}>
							<Grid item xs={12} md={6}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										label="Name"
										variant="outlined"
										{...register("name", { required: true })}
										error={!!errors.name}
										helperText={errors.name ? "This field is required" : ""}
									/>
								</Box>
							</Grid>
							<Grid item xs={12} md={6}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										label="Label"
										variant="outlined"
										{...register("label", { required: true })}
										error={!!errors.label}
										helperText={errors.label ? "This field is required" : ""}
									/>
								</Box>
							</Grid>
							<Grid item xs={12}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										label="Short Description"
										variant="outlined"
										{...register("desc_short", { required: true })}
										error={!!errors.desc_short}
										helperText={errors.desc_short ? "This field is required" : ""}
									/>
								</Box>
							</Grid>
							<Grid item xs={12}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										label="Long Description"
										variant="outlined"
										multiline
										rows={4}
										{...register("desc_long", { required: true })}
										error={!!errors.desc_long}
										helperText={errors.desc_long ? "This field is required" : ""}
									/>
								</Box>
							</Grid>
							<Grid item xs={12} md={4}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										label="Area"
										variant="outlined"
										{...register("area", { required: true })}
										error={!!errors.area}
										helperText={errors.area ? "This field is required" : ""}
									/>
								</Box>
							</Grid>
							<Grid item xs={12} md={4}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										label="Carbon Credits"
										variant="outlined"
										type="number"
										{...register("carbon_credits", { required: true, valueAsNumber: true })}
										error={!!errors.carbon_credits}
										helperText={errors.carbon_credits ? "This field is required" : ""}
									/>
								</Box>
							</Grid>
							<Grid item xs={12} md={4}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										label="ROI Percent"
										variant="outlined"
										type="number"
										{...register("roi_percent", { required: true, valueAsNumber: true })}
										error={!!errors.roi_percent}
										helperText={errors.roi_percent ? "This field is required" : ""}
									/>
								</Box>
							</Grid>
							<Grid item xs={12} md={6}>
								<Box sx={styles.formField}>
									<FormControl fullWidth error={!!errors.state} variant="outlined">
										<InputLabel id="state-label">State</InputLabel>
										<Select labelId="state-label" {...register("state", { required: true })} label="State">
											<MenuItem value={ForestProjectState.DRAFT}>Draft</MenuItem>
											<MenuItem value={ForestProjectState.ACTIVE}>Active</MenuItem>
											<MenuItem value={ForestProjectState.FUNDED}>Funded</MenuItem>
											<MenuItem value={ForestProjectState.BOND}>Bond</MenuItem>
											<MenuItem value={ForestProjectState.ARCHIVED}>Archived</MenuItem>
										</Select>
										{errors.state && (
											<Typography color="error" variant="caption">
												This field is required
											</Typography>
										)}
									</FormControl>
								</Box>
							</Grid>
							<Grid item xs={12} md={6}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										label="Shares Available"
										variant="outlined"
										type="number"
										{...register("shares_available", { required: true, valueAsNumber: true })}
										error={!!errors.shares_available}
										helperText={errors.shares_available ? "This field is required" : ""}
									/>
								</Box>
							</Grid>
							<Grid item xs={12} md={6}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										label="Large Image URL"
										variant="outlined"
										{...register("image_large_url", { required: true })}
										error={!!errors.image_large_url}
										disabled
										helperText={errors.image_large_url ? "This field is required" : ""}
										InputLabelProps={{ shrink: !!watch("image_large_url") }}
									/>
									<Box sx={styles.imageUploaderContainer}>
										<ImageUploader
											aspectRatio={2.5}
											onChange={(v) =>
												adminUploadImage(fileBaseUrl, "image_large_url", v).then((url) => setValue("image_large_url", url))
											}
											url={watch("image_large_url")}
										/>
									</Box>
								</Box>
							</Grid>
							<Grid item xs={12} md={6}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										variant="outlined"
										label="Small Image URL"
										{...register("image_small_url", { required: true })}
										error={!!errors.image_small_url}
										disabled
										helperText={errors.image_small_url ? "This field is required" : ""}
										InputLabelProps={{ shrink: !!watch("image_small_url") }}
									/>
									<Box sx={styles.imageUploaderContainer}>
										<ImageUploader
											aspectRatio={2.2}
											onChange={(v) =>
												adminUploadImage(fileBaseUrl, "image_small_url", v).then((url) => setValue("image_small_url", url))
											}
											url={watch("image_small_url")}
										/>
									</Box>
								</Box>
							</Grid>
						</Grid>
					</Box>

					<Divider sx={styles.formDivider} />

					<Box sx={styles.formSection}>
						<Box sx={styles.formSectionHeader}>
							<ImageIcon />
							<Typography variant="h6">Property Media</Typography>
						</Box>
						<Grid container spacing={3}>
							<Grid item xs={12}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										label="Property Media Header"
										variant="outlined"
										multiline
										rows={4}
										{...register("property_media_header", { required: true })}
										error={!!errors.property_media_header}
										helperText={errors.property_media_header ? "This field is required" : ""}
									/>
								</Box>
							</Grid>
							<Grid item xs={12}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										label="Property Media Footer"
										variant="outlined"
										multiline
										rows={4}
										{...register("property_media_footer", { required: true })}
										error={!!errors.property_media_footer}
										helperText={errors.property_media_footer ? "This field is required" : ""}
									/>
								</Box>
							</Grid>
						</Grid>
					</Box>

					<Divider sx={styles.formDivider} />

					<Box sx={styles.formSection}>
						<Box sx={styles.formSectionHeader}>
							<DescriptionIcon />
							<Typography variant="h6">Offering Document</Typography>
						</Box>
						<Grid container spacing={3}>
							<Grid item xs={12} md={6}>
								<Box sx={styles.formField}>
									<TextField fullWidth variant="outlined" label="Offering Document Title" {...register("offering_doc_title")} />
								</Box>
							</Grid>
							<Grid item xs={12}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										variant="outlined"
										label="Offering Document Header"
										multiline
										rows={4}
										{...register("offering_doc_header")}
									/>
								</Box>
							</Grid>
							<Grid item xs={12}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										variant="outlined"
										label="Offering Document Image URL"
										{...register("offering_doc_img_url")}
										disabled
										InputLabelProps={{ shrink: !!watch("offering_doc_img_url") }}
									/>
									<Box sx={styles.imageUploaderContainer}>
										<ImageUploader
											aspectRatio={2.5}
											onChange={(v) =>
												adminUploadImage(fileBaseUrl, "offering_doc_img_url", v).then((url) =>
													setValue("offering_doc_img_url", url),
												)
											}
											url={watch("offering_doc_img_url")}
										/>
									</Box>
								</Box>
							</Grid>
							<Grid item xs={12}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										variant="outlined"
										label="Offering Document Footer"
										multiline
										rows={4}
										{...register("offering_doc_footer")}
									/>
								</Box>
							</Grid>
						</Grid>
					</Box>

					<Divider sx={styles.formDivider} />

					<Box sx={styles.formSection}>
						<Box sx={styles.formSectionHeader}>
							<InsertChartIcon />
							<Typography variant="h6">Financial Projection</Typography>
						</Box>
						<Grid container spacing={3}>
							<Grid item xs={12} md={6}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										variant="outlined"
										label="Financial Projection Title"
										{...register("financial_projection_title")}
									/>
								</Box>
							</Grid>
							<Grid item xs={12}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										variant="outlined"
										label="Financial Projection Header"
										multiline
										rows={4}
										{...register("financial_projection_header")}
									/>
								</Box>
							</Grid>
							<Grid item xs={12}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										variant="outlined"
										label="Financial Projection Image URL"
										{...register("financial_projection_img_url")}
										disabled={true}
										InputLabelProps={{ shrink: !!watch("financial_projection_img_url") }}
									/>
									<Box sx={styles.imageUploaderContainer}>
										<ImageUploader
											aspectRatio={2.5}
											onChange={(v) =>
												adminUploadImage(fileBaseUrl, "financial_projection_img_url", v).then((url) =>
													setValue("financial_projection_img_url", url),
												)
											}
											url={watch("financial_projection_img_url")}
										/>
									</Box>
								</Box>
							</Grid>
							<Grid item xs={12}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										variant="outlined"
										label="Financial Projection Footer"
										multiline
										rows={4}
										{...register("financial_projection_footer")}
									/>
								</Box>
							</Grid>
						</Grid>
					</Box>

					<Divider sx={styles.formDivider} />

					<Box sx={styles.formSection}>
						<Box sx={styles.formSectionHeader}>
							<LocationOnIcon />
							<Typography variant="h6">Geospatial Information</Typography>
						</Box>
						<Grid container spacing={3}>
							<Grid item xs={12} md={6}>
								<Box sx={styles.formField}>
									<TextField fullWidth variant="outlined" label="Geo Title" {...register("geo_title")} />
								</Box>
							</Grid>
							<Grid item xs={12}>
								<Box sx={styles.formField}>
									<TextField fullWidth variant="outlined" label="Geo Header" multiline rows={4} {...register("geo_header")} />
								</Box>
							</Grid>
							<Grid item xs={12}>
								<Box sx={styles.formField}>
									<TextField
										fullWidth
										variant="outlined"
										label="Geo Image URL"
										{...register("geo_img_url")}
										disabled={true}
										InputLabelProps={{ shrink: !!watch("geo_img_url") }}
									/>
									<Box sx={styles.imageUploaderContainer}>
										<ImageUploader
											aspectRatio={2.5}
											onChange={(v) => adminUploadImage(fileBaseUrl, "geo_img_url", v).then((url) => setValue("geo_img_url", url))}
											url={watch("geo_img_url")}
										/>
									</Box>
								</Box>
							</Grid>
							<Grid item xs={12}>
								<Box sx={styles.formField}>
									<TextField fullWidth variant="outlined" label="Geo Footer" multiline rows={4} {...register("geo_footer")} />
								</Box>
							</Grid>
						</Grid>
					</Box>

					<Box sx={styles.formActions}>
						<Button type="submit" variant="contained" color="primary" sx={styles.formSubmitButton}>
							Create Project
						</Button>
					</Box>
				</Paper>
			</Container>
		</>
	);
}
