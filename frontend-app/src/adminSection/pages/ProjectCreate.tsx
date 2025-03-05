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
} from "@mui/material";
import { formatDate } from "../../lib/conversions";
import { Link, useNavigate } from "react-router";

export default function ProjectCreate() {
	const {
		register,
		handleSubmit,
		formState: { errors },
	} = useForm<ForestProject>();
	const navigate = useNavigate();

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
			<Breadcrumbs aria-label="breadcrumb">
				<Link to="/admin">Admin</Link>
				<Link to="/admin/projects">Projects</Link>
				<Typography color="textPrimary">Create</Typography>
			</Breadcrumbs>
			<div>
				<Box component="form" onSubmit={handleSubmit(onSubmit)} sx={{ display: "flex", flexDirection: "column", gap: 2 }}>
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
						<Select labelId="state-label" {...register("state", { required: true })} label="State">
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
						helperText={errors.image_large_url ? "This field is required" : ""}
					/>
					<TextField
						label="Small Image URL"
						{...register("image_small_url", { required: true })}
						error={!!errors.image_small_url}
						helperText={errors.image_small_url ? "This field is required" : ""}
					/>
					<TextField label="Geo Spatial URL" {...register("geo_spatial_url")} />
					<TextField
						label="Shares Available"
						type="number"
						{...register("shares_available", { required: true, valueAsNumber: true })}
						error={!!errors.shares_available}
						helperText={errors.shares_available ? "This field is required" : ""}
					/>
					<TextField label="Offering Document Link" {...register("offering_doc_link")} />
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
					<Button type="submit" variant="contained" color="primary">
						Create Project
					</Button>
				</Box>
			</div>
		</>
	);
}
