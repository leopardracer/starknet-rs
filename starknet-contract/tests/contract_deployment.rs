use rand::{rngs::StdRng, RngCore, SeedableRng};
use starknet_accounts::{ExecutionEncoding, SingleOwnerAccount};
use starknet_contract::ContractFactory;
use starknet_core::types::{contract::legacy::LegacyContractClass, Felt};
use starknet_providers::{jsonrpc::HttpTransport, JsonRpcClient};
use starknet_signers::{LocalWallet, SigningKey};
use url::Url;

/// Cairo short string encoding for `SN_SEPOLIA`.
const CHAIN_ID: Felt = Felt::from_raw([
    507980251676163170,
    18446744073709551615,
    18446744073708869172,
    1555806712078248243,
]);

#[tokio::test]
async fn can_deploy_contract_to_alpha_sepolia_with_invoke_v3() {
    let rpc_url = std::env::var("STARKNET_RPC")
        .unwrap_or_else(|_| "https://pathfinder.rpc.sepolia.starknet.rs/rpc/v0_9".into());
    let provider = JsonRpcClient::new(HttpTransport::new(Url::parse(&rpc_url).unwrap()));
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap(),
    ));
    let address =
        Felt::from_hex("0x034dd51aa591d174b60d1cb45e46dfcae47946fae1c5e62933bbf48effedde4d")
            .unwrap();
    let account =
        SingleOwnerAccount::new(provider, signer, address, CHAIN_ID, ExecutionEncoding::New);

    let artifact = serde_json::from_str::<LegacyContractClass>(include_str!(
        "../test-data/cairo0/artifacts/oz_account.txt"
    ))
    .unwrap();

    let factory = ContractFactory::new(artifact.class_hash().unwrap(), account);

    let mut salt_buffer = [0u8; 32];
    let mut rng = StdRng::from_entropy();
    rng.fill_bytes(&mut salt_buffer[1..]);

    let result = factory
        .deploy_v3(vec![Felt::ONE], Felt::from_bytes_be(&salt_buffer), true)
        .l1_gas(0)
        .l1_gas_price(1000000000000000)
        .l2_gas(1000000)
        .l2_gas_price(10000000000)
        .l1_data_gas(1000)
        .l1_data_gas_price(100000000000000)
        .send()
        .await;

    match result {
        Ok(_) => {}
        Err(err) => panic!("Contract deployment failed: {err}"),
    }
}
