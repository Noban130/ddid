
use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    schema::ZkpSignUpSchema,
    zkpgenerate::{zkpproof_sign_in, zkpproof_sign_up},
};

use bellman::groth16::{Proof, VerifyingKey};
use bls12_381::{Bls12, Scalar, G1Affine, G2Affine};

pub use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct ScalarWrapper([u8; 32]);

impl From<Scalar> for ScalarWrapper {
    fn from(scalar: Scalar) -> Self {
        ScalarWrapper(scalar.to_bytes())
    }
}

impl Into<Scalar> for ScalarWrapper {
    fn into(self) -> Scalar {
        Scalar::from_bytes(&self.0).unwrap()
    }
}

use crate::sol_connect::{user_sign_up, user_sign_in};

pub async fn zkp_signup(
    Json(body): Json<ZkpSignUpSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let embeddinghash_num = hash_to_array(&body.embedding_hash);
    let embeddinghash_num_arr = embeddinghash_num.unwrap();
    let microchipid_num_arr = body.microchip_id.to_be_bytes();
    let public_input = zkpproof_sign_up(embeddinghash_num_arr, microchipid_num_arr);
    println!("Public Input : {:?}", public_input);
    let public_input_to_send = vec![ScalarWrapper::from(public_input[0]), ScalarWrapper::from(public_input[1])];
    println!("Public Input to send : {:?}", public_input_to_send);
    user_sign_up(public_input_to_send);
    Ok({})
}

pub async fn zkp_signin(
    Json(body): Json<ZkpSignUpSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let embeddinghash_num = hash_to_array(&body.embedding_hash);
    let embeddinghash_num_arr = embeddinghash_num.unwrap();
    let microchipid_num_arr = body.microchip_id.to_be_bytes();
    let (proof, vk) = zkpproof_sign_in(embeddinghash_num_arr, microchipid_num_arr);
    println!("Proof: {:?}", proof);
    let proof_bytes = serialize_proof(&proof);
    let vk_to_send = serialize_verifying_key(&vk);
    user_sign_in(proof_bytes, vk_to_send);

    let zkp_response = serde_json::json!({"status": "success","data": serde_json::json!({
        "zkp": "Proof generated successfuly!"
    })});

    return Ok(Json(zkp_response));
}

fn hash_to_array(hash: &str) -> Result<[u8; 64], String> {
    if hash.len() != 64 {
        return Err(format!("Expected 64 characters, got {}", hash.len()));
    }

    let mut result = [0u8; 64];
    for (i, chunk) in hash.as_bytes().chunks(2).enumerate() {
        let hash_str = std::str::from_utf8(chunk).map_err(|_| "Invalid UTF-8 in hash")?;
        result[i] = u8::from_str_radix(hash_str, 16).map_err(|_| "Invalid hash digit")?;
    }
    Ok(result)
}

fn serialize_g1(element: &G1Affine) -> [u8; 48] {
    element.to_compressed()
}

fn serialize_g2(element: &G2Affine) -> [u8; 96] {
    element.to_compressed()
}

fn serialize_proof(proof: &Proof<Bls12>) -> Vec<u8> {
    let mut serialized = Vec::new();
    serialized.extend_from_slice(&serialize_g1(&proof.a));
    serialized.extend_from_slice(&serialize_g2(&proof.b));
    serialized.extend_from_slice(&serialize_g1(&proof.c));
    serialized
}


// Serialize the VerifyingKey
fn serialize_verifying_key(vk: &VerifyingKey<Bls12>) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend(serialize_g1(&vk.alpha_g1));   // G1 point
    bytes.extend(serialize_g2(&vk.beta_g2));   // G2 point
    bytes.extend(serialize_g2(&vk.gamma_g2));  // G2 point
    bytes.extend(serialize_g2(&vk.delta_g2));  // G2 point
    for ic in &vk.ic {
        bytes.extend(serialize_g1(ic));        // G1 points
    }
    bytes
}