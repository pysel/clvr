#[cfg(test)]
mod tests {
    use std::io::Read;
    use std::str::FromStr;

    use alloy::hex;
    use alloy::primitives::{Address, FixedBytes, PrimitiveSignature}; // Ensure to import necessary types
    use alloy::primitives::{keccak256, U256};
    use alloy::primitives::B256;
    use alloy::signers::k256::ecdsa::{SigningKey, VerifyingKey};
    use alloy::signers::local::PrivateKeySigner;
    use alloy::signers::Signer;
    use once_cell::sync::Lazy;

    use crate::server::eip2612::verify_eip2612_signature;

    const PRIV_KEY_MOCK: &str = "19d02659c2bb206dca930737e9e6b27d4e1dbcfb9784b87920ee5956e3383970";
    const PUB_KEY_MOCK_RAW: [u8; 64] = [
        164, 59, 102, 209, 234, 238, 3, 240, 125, 100, 146, 4, 145, 248, 179, 72, 122, 144, 245, 39, 242, 52, 44, 140, 172, 205, 85, 213, 6, 80, 132, 73, 108, 87, 212, 9, 214, 219, 6, 250, 239, 216, 160, 170, 17, 6, 172, 214, 149, 1, 19, 78, 17, 207, 116, 178, 233, 92, 129, 180, 81, 218, 52, 51
    ];

    const CONSTANT_LEADING_BYTES: &[u8] = &[0x19, 0x01];

    const DOMAIN_NAME: &str = "CLVR";
    const DOMAIN_VERSION: &str = "1";
    const CHAIN_ID: Lazy<U256> = Lazy::new(|| U256::from(137));
    const VERIFYING_CONTRACT: Lazy<Address> = Lazy::new(|| Address::from_raw_public_key(&PUB_KEY_MOCK_RAW));

    const PRIV_KEY_MOCK_SPENDER: &str = "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";

    const OWNER: Lazy<Address> = Lazy::new(|| Address::from_raw_public_key(&PUB_KEY_MOCK_RAW));
    const SPENDER: Lazy<Address> = Lazy::new(|| Address::from_raw_public_key(&PUB_KEY_MOCK_RAW));
    const VALUE: Lazy<U256> = Lazy::new(|| U256::from(1000000));
    const NONCE: Lazy<U256> = Lazy::new(|| U256::from(0));
    const DEADLINE: Lazy<U256> = Lazy::new(|| U256::from(1715999999));

    fn generate_domain_separator(
        name: &str,
        version: &str,
        chain_id: U256,
        verifying_contract: Address
    ) -> FixedBytes<32> {
        let domain_typehash = keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");
        let name_hash = keccak256(name.as_bytes());
        let version_hash = keccak256(version.as_bytes());

        let chain_id_bytes: FixedBytes<32> = chain_id.into();
        
        let combined_bytes: Vec<u8> = [
            domain_typehash.as_slice(),
            name_hash.as_slice(),
            version_hash.as_slice(),
            chain_id_bytes.as_slice(),
            verifying_contract.as_slice(),
        ].concat();
        
        keccak256(&combined_bytes)
    }

    fn generate_permit_body(
        owner: Address,
        spender: Address,
        value: U256,
        nonce: U256,
        deadline: U256
    ) -> FixedBytes<32> {
        let body_typehash = keccak256("Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)");

        let combined_bytes: Vec<u8> = [
            body_typehash.as_slice(),
            owner.as_slice(),
            spender.as_slice(),
            value.to_string().as_bytes(),
            nonce.to_string().as_bytes(),
            deadline.to_string().as_bytes(),
        ].concat();

        keccak256(&combined_bytes)
    }

    fn generate_permit_message(
        domain_separator: FixedBytes<32>,
        permit_body: FixedBytes<32>
    ) -> FixedBytes<32> {
        keccak256(&[CONSTANT_LEADING_BYTES, domain_separator.as_slice(), permit_body.as_slice()].concat())
    }
    
    #[tokio::test]
    async fn test_verify_eip2612_signature() {
        let signer: PrivateKeySigner = PrivateKeySigner::from_str(PRIV_KEY_MOCK).unwrap();
        let signer: PrivateKeySigner = signer.with_chain_id(Some(137));
        let owner = signer.address();

        // generate permit message to sign
        let domain_separator = generate_domain_separator(DOMAIN_NAME, DOMAIN_VERSION, *CHAIN_ID, *VERIFYING_CONTRACT);
        let permit_body = generate_permit_body(*OWNER, *SPENDER, *VALUE, *NONCE, *DEADLINE);
        let permit_message = generate_permit_message(domain_separator, permit_body);

        // generate a signature
        let signature = signer.sign_message(permit_message.as_slice()).await.unwrap();

        let result = verify_eip2612_signature(permit_message, signature, owner);

        assert!(result, "Signature verification failed");

        let result = verify_eip2612_signature(permit_message, signature, *SPENDER);

        assert!(!result, "Signature verification should fail");
    }
}