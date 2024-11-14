use alloy::primitives::{Address, FixedBytes, PrimitiveSignature};
use alloy::primitives::{keccak256, B256, U256};


const CONSTANT_LEADING_BYTES: &[u8] = &[0x19, 0x01];

#[cfg(test)]
fn generate_domain_separator(
    name: &str,
    version: &str,
    chain_id: U256,
    verifying_contract: Address
) -> FixedBytes<32> {
    let domain_typehash = keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");
    let name_hash = keccak256(name.as_bytes());
    let version_hash = keccak256(version.as_bytes());

    let chain_id_bytes:FixedBytes<32> = chain_id.into();
    let contract_bytes = B256::from_slice(verifying_contract.to_string().as_bytes());
    
    keccak256(&[domain_typehash, name_hash, version_hash, chain_id_bytes, contract_bytes].concat())
}

pub fn verify_eip2612_signature(
    permit_message: FixedBytes<32>,
    signature: PrimitiveSignature,
    signer: Address,
) -> bool {
    let recovered_address = signature.recover_address_from_msg(permit_message).unwrap();

    recovered_address == signer
}

pub fn get_permit_signature_fields(
    signature: PrimitiveSignature,
) -> (u8, [u8; 32], [u8; 32]) {
    let r: FixedBytes<32> = signature.r().into();
    let s: FixedBytes<32> = signature.s().into();

    (signature.v().into(), r.0, s.0)
}
