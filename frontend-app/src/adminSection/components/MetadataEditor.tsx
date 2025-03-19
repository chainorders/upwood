import React, { useCallback, useEffect, useState } from "react";
import {
    Box,
    CircularProgress,
    Alert,
    IconButton,
} from "@mui/material";
import RefreshIcon from "@mui/icons-material/Refresh";
import TokenMetadataForm from "./TokenMetadataForm";
import { TokenMetadata } from "../libs/types";

interface MetadataEditorProps {
    defaultMetadata: TokenMetadata;
    metadataUrl?: string;
    fileBaseUrl: string;
    onMetadataSubmit: (data: TokenMetadata) => Promise<void>;
}

const MetadataEditor: React.FC<MetadataEditorProps> = ({
    defaultMetadata,
    metadataUrl,
    fileBaseUrl,
    onMetadataSubmit,
}) => {
    const [metadata, setMetadata] = useState<TokenMetadata>(defaultMetadata);
    const [isMetadataLoading, setIsMetadataLoading] = useState<boolean>(false);
    const [metadataError, setMetadataError] = useState<string | null>(null);

    // Fetch metadata from URL
    const fetchMetadata = useCallback(
        async (url: string) => {
            if (!url || url.trim() === "") {
                // Reset to default metadata if URL is empty
                setMetadata(defaultMetadata);
                setMetadataError(null);
                setIsMetadataLoading(false);
                return;
            }

            setIsMetadataLoading(true);
            setMetadataError(null);

            try {
                const response = await fetch(url);

                if (!response.ok) {
                    throw new Error(`Failed to fetch metadata: ${response.status} ${response.statusText}`);
                }

                const data = await response.json();
                setMetadata(data);
            } catch (error) {
                console.error("Error fetching metadata:", error);
                setMetadataError(error instanceof Error ? error.message : "Failed to fetch metadata");

                // Set default values on error
                setMetadata(defaultMetadata);
            } finally {
                setIsMetadataLoading(false);
            }
        },
        [defaultMetadata]
    );

    // Trigger metadata fetch when URL changes
    useEffect(() => {
        if (!metadataUrl) {
            setMetadata(defaultMetadata);
            setMetadataError(null);
            setIsMetadataLoading(false);
            return;
        }

        fetchMetadata(metadataUrl);
    }, [fetchMetadata, metadataUrl, defaultMetadata]);

    if (isMetadataLoading) {
        return (
            <Box sx={{ display: "flex", justifyContent: "center", p: 4 }}>
                <CircularProgress />
            </Box>
        );
    }

    if (metadataError) {
        return (
            <>
                <Alert
                    severity="error"
                    sx={{ mb: 2 }}
                    action={
                        <IconButton color="inherit" size="small" onClick={() => metadataUrl && fetchMetadata(metadataUrl)}>
                            <RefreshIcon />
                        </IconButton>
                    }
                >
                    {metadataError}
                </Alert>
                <TokenMetadataForm
                    initialData={metadata}
                    onSubmit={onMetadataSubmit}
                    submitButtonText="Generate Metadata URL"
                    noForm={true}
                    fileBaseUrl={fileBaseUrl}
                />
            </>
        );
    }

    return (
        <TokenMetadataForm
            initialData={metadata}
            onSubmit={onMetadataSubmit}
            submitButtonText="Generate Metadata URL"
            noForm={true}
            fileBaseUrl={fileBaseUrl}
        />
    );
};

export default MetadataEditor;
