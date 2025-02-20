import { useParams, useNavigate } from "react-router";
import { useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { Link } from "react-router";
import {
	Breadcrumbs,
	Typography,
	Box,
	Button,
	TextField,
	Select,
	MenuItem,
	InputLabel,
	FormControl,
	CircularProgress,
} from "@mui/material";
import {
	ForestProject,
	ForestProjectService,
	ForestProjectTokenContract,
	SecurityTokenContractType,
} from "../../apiClient";

export default function ProjectContractUpdate() {
	const { contract_address } = useParams<{ contract_address?: string }>();
	const navigate = useNavigate();
	const [loading, setLoading] = useState(true);
	const [contract, setContract] = useState<ForestProjectTokenContract | null>(null);
	const [project, setProject] = useState<ForestProject | null>(null);

	const {
		register,
		handleSubmit,
		setValue,
		formState: { errors },
	} = useForm<ForestProjectTokenContract>();

	useEffect(() => {
		if (contract_address) {
			ForestProjectService.getAdminForestProjectsContract(contract_address)
				.then((data) => {
					setContract(data);
					setLoading(false);
					Object.keys(data).forEach((key) => {
						setValue(key as keyof ForestProjectTokenContract, data[key as keyof ForestProjectTokenContract]);
					});
				})
				.catch(() => {
					alert("Failed to fetch contract details");
					setLoading(false);
				});
		}
	}, [contract_address, setValue]);

	useEffect(() => {
		if (contract) {
			ForestProjectService.getAdminForestProjects(contract.forest_project_id).then(setProject);
		}
	}, [contract]);

	const onSubmit = (data: ForestProjectTokenContract) => {
		ForestProjectService.putAdminForestProjectsContract(data)
			.then(() => {
				alert("Contract updated successfully");
				navigate(-1);
			})
			.catch(() => {
				alert("Failed to update contract");
			});
	};

	if (loading) {
		return <CircularProgress />;
	}

	if (!contract || !project) {
		return <Typography>No contract found</Typography>;
	}

	return (
		<>
			<Breadcrumbs aria-label="breadcrumb">
				<Link to="/admin">Admin</Link>
				<Link to="/admin/projects">Projects</Link>
				<Link to={`/admin/projects/${project.id}/details`}>{project.name}</Link>
				<Link to={`/admin/projects/${project.id}/contract/${contract_address}/details`}>
					{contract.contract_type} - {contract.contract_address}
				</Link>
				<Typography color="textPrimary">Contract Update</Typography>
			</Breadcrumbs>
			<div>
				<Box component="form" onSubmit={handleSubmit(onSubmit)} sx={{ display: "flex", flexDirection: "column", gap: 2 }}>
					<TextField
						label="Contract Address"
						{...register("contract_address", { required: true })}
						error={!!errors.contract_address}
						helperText={errors.contract_address ? "This field is required" : ""}
						disabled
					/>
					<FormControl error={!!errors.contract_type}>
						<InputLabel id="contract-type-label">Contract Type</InputLabel>
						<Select
							labelId="contract-type-label"
							{...register("contract_type", { required: true })}
							label="Contract Type"
							defaultValue={contract.contract_type}
						>
							<MenuItem value={SecurityTokenContractType.PROPERTY}>Property</MenuItem>
							<MenuItem value={SecurityTokenContractType.BOND}>Bond</MenuItem>
							<MenuItem value={SecurityTokenContractType.PROPERTY_PRE_SALE}>Property Pre Sale</MenuItem>
							<MenuItem value={SecurityTokenContractType.BOND_PRE_SALE}>Bond Pre Sale</MenuItem>
						</Select>
						{errors.contract_type && <span>This field is required</span>}
					</FormControl>
					<TextField
						label="Fund Token ID (optional)"
						{...register("fund_token_id", { setValueAs: (val: string) => val || undefined })}
					/>
					<TextField
						label="Market Token ID (optional)"
						{...register("market_token_id", { setValueAs: (val: string) => val || undefined })}
					/>
					<TextField label="Symbol (optional)" {...register("symbol", { setValueAs: (val: string) => val || undefined })} />
					<TextField label="Decimals (optional)" type="number" {...register("decimals", { valueAsNumber: true })} />
					<Button type="submit" variant="contained" color="primary">
						Update Contract
					</Button>
				</Box>
			</div>
		</>
	);
}
