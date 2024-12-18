use bellman::{
    gadgets::
        multipack::{
            bytes_to_bits_le, compute_multipacking
        },
    groth16::{
        create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof, Proof
    }
};
use rand::rngs::OsRng;
use bls12_381::{Bls12, Scalar};
use sha2::{Digest, Sha256};
mod problem;


pub fn zkpproof_sign_up(embeddinghash : [u8; 64], microchipid : [u8; 16]) -> Vec<Scalar> {
    println!("Prepare input...");
    let input_arr : [u8; 80] = [&embeddinghash[..], &microchipid[..]].concat().try_into().unwrap();
    let hidden_value = input_arr;
    let hash_bit = bytes_to_bits_le(&Sha256::digest(&hidden_value));
    let public_input = compute_multipacking::<Scalar>(&hash_bit);

    return public_input
}

pub fn zkpproof_sign_in(embeddinghash : [u8; 64], microchipid : [u8; 16]) -> Proof<Bls12> {
    println!("Learn zk-SNARKs with Terry");
    let params = {
        let c = problem::OurProblem { value: Some([100; 80]) };

        generate_random_parameters::<Bls12, _, _>(c, &mut OsRng).unwrap()
    };

    println!("Prepare input...");
    let input_arr : [u8; 80] = [&embeddinghash[..], &microchipid[..]].concat().try_into().unwrap();
    let hidden_value = input_arr;

    let c = problem::OurProblem {
        value: Some(hidden_value),
    };
    println!("Create proof...");
    let proof = create_random_proof(c, &params, &mut OsRng).unwrap();
    return proof;
}


// fn string_to_scalar(input_txt : String) -> Scalar {
//     // Remove the "0x" prefix if it's there
//     let hex_str = &input_txt[2..];
    
//     // Convert the hex string to a byte vector
//     let bytes = hex::decode(hex_str).expect("Invalid hex string");
//     println!("Bytes: {:?}", bytes);
//     // Convert to [u8; 32]
//     // Convert Vec<u8> to [u8; 32]
//     let bytes_array: [u8; 32] = bytes
//         .try_into()
//         .expect("Failed to convert Vec<u8> to [u8; 32]");
//     println!("Array: {:?}", bytes_array);
//     let output_scalar = Scalar::from_bytes(&bytes_array).unwrap_or_else(|| {
//         panic!("CtOption is invalid, cannot unwrap");
//     });
    
//     return output_scalar
// }