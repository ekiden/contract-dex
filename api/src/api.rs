rpc_api! {
    metadata {
        name = dex_contract;
        version = "0.1.0";
        client_attestation_required = false;
    }

    rpc create(CreateRequest) -> (state, CreateResponse);

    rpc transfer(state, TransferRequest) -> (state, TransferResponse);

    rpc get_balance(state, GetBalanceRequest) -> (GetBalanceResponse);
}
