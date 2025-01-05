-- Your SQL goes here
create type security_sft_multi_yielder_yield_type as enum ('quantity', 'simple_intrest');

create table
    security_sft_multi_yielder_yields (
        contract_address numeric(20) not null,
        token_contract_address numeric(20) not null,
        token_id numeric(20) not null,
        yield_contract_address numeric(20) not null,
        yield_token_id numeric(20) not null,
        yield_rate_numerator numeric(20) not null,
        yield_rate_denominator numeric(20) not null,
        yield_type security_sft_multi_yielder_yield_type not null,
        create_time timestamp not null,
        primary key (
            contract_address,
            token_contract_address,
            token_id,
            yield_contract_address,
            yield_token_id
        )
    );

create table
    security_sft_multi_yielder_yeild_distributions (
        id uuid primary key,
        contract_address numeric(20) not null,
        token_contract_address numeric(20) not null,
        from_token_version numeric(20) not null,
        to_token_version numeric(20) not null,
        yield_contract_address numeric(20) not null,
        yield_token_id numeric(20) not null,
        yield_amount numeric(20) not null,
        to_address Varchar not null,
        create_time timestamp not null
    );
