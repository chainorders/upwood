import { useForm } from "react-hook-form";
import { ForestProject, ForestProjectService, ForestProjectState } from "../../apiClient";
import { useEffect, useState } from "react";
import { useParams } from "react-router";
import { Button, TextField, Select, MenuItem, InputLabel, FormControl, Box, Grid } from "@mui/material";
import { formatDate } from "../../lib/conversions";

export default function ProjectUpdate() {
	const { id } = useParams<{ id: string }>();
	const {
		register,
		handleSubmit,
		setValue,
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
		<Box sx={{ flexGrow: 1, padding: 2 }}>
			<Grid container spacing={2}>
				<Grid item xs={12}>
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
							<Select
								labelId="state-label"
								{...register("state", { required: true })}
								label="State"
								defaultValue={project?.state || ""}
							>
								<MenuItem value={ForestProjectState.DRAFT}>Draft</MenuItem>
								<MenuItem value={ForestProjectState.ACTIVE}>Active</MenuItem>
								<MenuItem value={ForestProjectState.FUNDED}>Funded</MenuItem>
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
							Update Project
						</Button>
					</Box>
				</Grid>
			</Grid>
		</Box>
	);
}
