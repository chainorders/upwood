import { useForm } from "react-hook-form";
import { ForestProject, ForestProjectService, ForestProjectState } from "../../apiClient";
import { useEffect, useState } from "react";
import { useParams } from "react-router";
import {
	Button,
	TextField,
	Select,
	MenuItem,
	InputLabel,
	FormControl,
	Box,
	Grid,
	Breadcrumbs,
	Typography,
	Divider,
} from "@mui/material";
import { formatDate } from "../../lib/conversions";
import { Link } from "react-router";
import ImageUploader from "../components/ImageUploader";
import { adminUploadImage } from "../libs/utils";

export default function ProjectUpdate({ fileBaseUrl }: { fileBaseUrl: string }) {
	const { id } = useParams<{ id: string }>();
	const {
		register,
		handleSubmit,
		setValue,
		watch,
		formState: { errors },
	} = useForm<ForestProject>();
	const [project, setProject] = useState<ForestProject | null>(null);

	useEffect(() => {
		if (id) {
			ForestProjectService.getAdminForestProjects(id).then((response) => {
				setProject(response);
				Object.keys(response).forEach((key) => {
					setValue(key as keyof ForestProject, response[key as keyof ForestProject]);
					setValue("state", response.state); // Ensure state value is set
				});
			});
		}
	}, [id, setValue]);
	const onSubmit = (data: ForestProject) => {
		const now = new Date();
		data.updated_at = formatDate(now);
		console.log(data);

		ForestProjectService.putAdminForestProjects(data)
			.then(() => {
				alert("Project updated successfully");
			})
			.catch(() => {
				alert("Failed to update project");
			});
	};

	if (!project) {
		return <div>Loading...</div>;
	}

	return (
		<>
			<Breadcrumbs aria-label="breadcrumb">
				<Link to="/admin">Admin</Link>
				<Link to="/admin/projects">Projects</Link>
				<Link to={`/admin/projects/${id}/details`}>{project.name}</Link>
				<Typography color="textPrimary">Update</Typography>
			</Breadcrumbs>
			<Box sx={{ flexGrow: 1, padding: 2 }}>
				<Grid container spacing={2}>
					<Grid item xs={12}>
						<Box component="form" onSubmit={handleSubmit(onSubmit)} sx={{ display: "flex", flexDirection: "column", gap: 2 }}>
							<Typography variant="h6">Basic Information</Typography>
							<TextField
								label="Name"
								{...register("name", { required: true })}
								error={!!errors.name}
								helperText={errors.name ? "This field is required" : ""}
							/>
							<TextField
								label="Label"
								{...register("label", { required: true })}
								error={!!errors.label}
								helperText={errors.label ? "This field is required" : ""}
							/>
							<TextField
								label="Short Description"
								{...register("desc_short", { required: true })}
								error={!!errors.desc_short}
								helperText={errors.desc_short ? "This field is required" : ""}
							/>
							<TextField
								label="Long Description"
								multiline
								rows={4}
								{...register("desc_long", { required: true })}
								error={!!errors.desc_long}
								helperText={errors.desc_long ? "This field is required" : ""}
							/>
							<TextField
								label="Area"
								{...register("area", { required: true })}
								error={!!errors.area}
								helperText={errors.area ? "This field is required" : ""}
							/>
							<TextField
								label="Carbon Credits"
								type="number"
								{...register("carbon_credits", { required: true, valueAsNumber: true })}
								error={!!errors.carbon_credits}
								helperText={errors.carbon_credits ? "This field is required" : ""}
							/>
							<TextField
								label="ROI Percent"
								type="number"
								{...register("roi_percent", { required: true, valueAsNumber: true })}
								error={!!errors.roi_percent}
								helperText={errors.roi_percent ? "This field is required" : ""}
							/>
							<FormControl error={!!errors.state}>
								<InputLabel id="state-label">State</InputLabel>
								<Select
									labelId="state-label"
									{...register("state", { required: true })}
									label="State"
									defaultValue={project?.state || ""}
								>
									<MenuItem value={ForestProjectState.DRAFT}>Draft</MenuItem>
									<MenuItem value={ForestProjectState.ACTIVE}>Active</MenuItem>
									<MenuItem value={ForestProjectState.FUNDED}>Funded</MenuItem>
									<MenuItem value={ForestProjectState.BOND}>Bond</MenuItem>
									<MenuItem value={ForestProjectState.ARCHIVED}>Archived</MenuItem>
								</Select>
								{errors.state && <span>This field is required</span>}
							</FormControl>
							<TextField
								label="Large Image URL"
								{...register("image_large_url", { required: true })}
								error={!!errors.image_large_url}
								disabled
								helperText={errors.image_large_url ? "This field is required" : ""}
								InputLabelProps={{ shrink: !!watch("image_large_url") }}
							/>
							<ImageUploader
								aspectRatio={2.5}
								onChange={(v) =>
									adminUploadImage(fileBaseUrl, "image_large_url", v).then((url) => setValue("image_large_url", url))
								}
								url={watch("image_large_url")}
							/>
							<TextField
								label="Small Image URL"
								{...register("image_small_url", { required: true })}
								error={!!errors.image_small_url}
								disabled
								helperText={errors.image_small_url ? "This field is required" : ""}
								InputLabelProps={{ shrink: !!watch("image_small_url") }}
							/>
							<ImageUploader
								aspectRatio={2.2}
								onChange={(v) =>
									adminUploadImage(fileBaseUrl, "image_small_url", v).then((url) => setValue("image_small_url", url))
								}
								url={watch("image_small_url")}
							/>
							<TextField
								label="Shares Available"
								type="number"
								{...register("shares_available", { required: true, valueAsNumber: true })}
								error={!!errors.shares_available}
								helperText={errors.shares_available ? "This field is required" : ""}
							/>

							<Divider sx={{ my: 2 }} />
							<Typography variant="h6">Property Media</Typography>
							<TextField
								label="Property Media Header"
								multiline
								rows={4}
								{...register("property_media_header", { required: true })}
								error={!!errors.property_media_header}
								helperText={errors.property_media_header ? "This field is required" : ""}
							/>
							<TextField
								label="Property Media Footer"
								multiline
								rows={4}
								{...register("property_media_footer", { required: true })}
								error={!!errors.property_media_footer}
								helperText={errors.property_media_footer ? "This field is required" : ""}
							/>

							<Divider sx={{ my: 2 }} />
							<Typography variant="h6">Offering Document</Typography>
							<TextField label="Offering Document Title" {...register("offering_doc_title")} />
							<TextField label="Offering Document Header" multiline rows={4} {...register("offering_doc_header")} />
							<TextField 
								label="Offering Document Image URL" 
								{...register("offering_doc_img_url")} 
								disabled
								InputLabelProps={{ shrink: !!watch("offering_doc_img_url") }}
							/>
							<ImageUploader
								aspectRatio={2.5}
								onChange={(v) =>
									adminUploadImage(fileBaseUrl, "offering_doc_img_url", v).then((url) => setValue("offering_doc_img_url", url))
								}
								url={watch("offering_doc_img_url")}
							/>
							<TextField label="Offering Document Footer" multiline rows={4} {...register("offering_doc_footer")} />

							<Divider sx={{ my: 2 }} />
							<Typography variant="h6">Financial Projection</Typography>
							<TextField label="Financial Projection Title" {...register("financial_projection_title")} />
							<TextField label="Financial Projection Header" multiline rows={4} {...register("financial_projection_header")} />
							<TextField 
								label="Financial Projection Image URL" 
								{...register("financial_projection_img_url")} 
								disabled
								InputLabelProps={{ shrink: !!watch("financial_projection_img_url") }}
							/>
							<ImageUploader
								aspectRatio={2.5}
								onChange={(v) =>
									adminUploadImage(fileBaseUrl, "financial_projection_img_url", v).then((url) => setValue("financial_projection_img_url", url))
								}
								url={watch("financial_projection_img_url")}
							/>
							<TextField label="Financial Projection Footer" multiline rows={4} {...register("financial_projection_footer")} />

							<Divider sx={{ my: 2 }} />
							<Typography variant="h6">Geospatial Information</Typography>
							<TextField label="Geo Title" {...register("geo_title")} />
							<TextField label="Geo Header" multiline rows={4} {...register("geo_header")} />
							<TextField 
								label="Geo Image URL" 
								{...register("geo_img_url")} 
								disabled
								InputLabelProps={{ shrink: !!watch("geo_img_url") }}
							/>
							<ImageUploader
								aspectRatio={2.5}
								onChange={(v) =>
									adminUploadImage(fileBaseUrl, "geo_img_url", v).then((url) => setValue("geo_img_url", url))
								}
								url={watch("geo_img_url")}
							/>
							<TextField label="Geo Footer" multiline rows={4} {...register("geo_footer")} />

							<Button type="submit" variant="contained" color="primary" sx={{ mt: 2 }}>
								Update Project
							</Button>
						</Box>
					</Grid>
				</Grid>
			</Box>
		</>
	);
}
