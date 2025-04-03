ALTER TABLE security_sft_multi_yielder_yields
DROP CONSTRAINT security_sft_multi_yielder_yields_pkey;

ALTER TABLE security_sft_multi_yielder_yields
ADD CONSTRAINT security_sft_multi_yielder_yields_pkey PRIMARY KEY (
    contract_address,
    token_contract_address,
    token_id,
    yield_contract_address,
    yield_token_id,
    yield_type
);
