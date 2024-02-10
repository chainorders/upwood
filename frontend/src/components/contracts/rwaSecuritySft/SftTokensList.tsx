import { ContractAddress } from "@concordium/web-sdk";
import { useSearchParams } from "react-router-dom";
import { useEffect, useState } from "react";
import { NftToken } from "../../../lib/contracts-api-client";
import { useContractsApi } from "../../ContractsApiProvider";
import TokensList from "../../common/TokensList";

type Props = {
	contract: ContractAddress.Type;
};
export default function SftTokensList(props: Props) {
	const { contract } = props;
	const [pageCount, setPageCount] = useState(0);
	const [searchParams] = useSearchParams();
	const [page, setPage] = useState(Number(searchParams.get("page") || "0"));
	const [loading, setLoading] = useState(false);
	const [error, setError] = useState("");
	const [tokens, setTokens] = useState<NftToken[]>([]);
	const { provider: backendApi } = useContractsApi();

	useEffect(() => {
		setLoading(true);
		setError("");
		backendApi.default
			.getRwaSecuritySftTokens({
				index: Number(contract.index),
				subindex: Number(contract.subindex),
				page,
			})
			.then((response) => {
				setTokens(response.data);
				setPageCount(response.page_count);
				setPage(response.page);
			})
			.catch((error) => setError(error.message))
			.finally(() => setLoading(false));
	}, [contract, backendApi, page]);

	return TokensList({
		contract,
		tokens,
		pageCount,
		page,
		loading,
		error,
	});
}
