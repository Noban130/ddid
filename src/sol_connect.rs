use anchor_client::solana_sdk::{pubkey::Pubkey, signer::Signer, signature::Keypair};
use anchor_client::Client;
use solana_client::rpc_client::RpcClient;
use solana_sdk::transaction::Transaction;
use solana_sdk::system_instruction;
use anchor_client::program::{Program, ProgramError};
use solana_sdk::commitment_config::CommitmentConfig;
use bls12_381::{Bls12, Scalar};

use anchor_lang::prelude::*;
use bellman::groth16::Proof;
use bincode; // For compact serialization
use ark_ff::PrimeField;

const PROGRAM_ID: &str = "EgM2WEYJ6cDjkEmRvN6S5t2CgeJYaRLWK82qwwDBaSBy"; // Replace with your program ID
const RPC_URL: &str = "https://api.devnet.solana.com"; // Solana RPC URL


pub fn user_sign_up(public_input: Vec<Scalar>) -> Result<(), ProgramError> {
    // Establish connection to Solana
    let rpc_client = RpcClient::new_with_commitment(RPC_URL.to_string(), CommitmentConfig::confirmed());

    // Generate keypair for the wallet (or use an existing one)
    let user_keypair = Keypair::new();

    // Create an anchor client to interact with the Solana program
    let client = Client::new_with_options(
        rpc_client,
        user_keypair.clone(),
        CommitmentConfig::confirmed(),
    );

    // Load the program and interact with it
    let program = client.program(Pubkey::from_str(PROGRAM_ID)?);
    // Define the account address to store user data
    let user_data_account = Pubkey::find_program_address(
        &[b"user_data", &user_keypair.pubkey().to_bytes()],
        &program.id(),
    ).0;
    let public_input_bytes = serialize_scalars(&public_input);
    // Call the user_sign_up function on the program
    program.rpc()
        .user_sign_up(public_input_bytes)
        .accounts(
            accounts::UserSignUp {
                user_data: user_data_account,
                user: user_keypair.pubkey(),
                system_program: system_instruction::program_id(),
            }
        )
        .signer(user_keypair)
        .send()?;

    println!("User signed up successfully");
    Ok(())
}

pub fn user_sign_in(proof: Proof<Bls12>) -> Result<(), ProgramError> {
    // Serialize the proof
    let proof_bytes = serialize_proof(&proof);
    // Establish connection to Solana
    let rpc_client = RpcClient::new_with_commitment(RPC_URL.to_string(), CommitmentConfig::confirmed());

    // Generate keypair for the wallet (or use an existing one)
    let user_keypair = Keypair::new();

    // Create an anchor client to interact with the Solana program
    let client = Client::new_with_options(
        rpc_client,
        user_keypair.clone(),
        CommitmentConfig::confirmed(),
    );

    // Load the program and interact with it
    let program = client.program(Pubkey::from_str(PROGRAM_ID)?);
    // Fetch user data (you need to verify that this exists and contains the correct public input)
    let user_data_account = Pubkey::find_program_address(
        &[b"user_data", &user_keypair.pubkey().to_bytes()],
        &program.id(),
    ).0;

    // Call the user_sign_in function with proof
    program.rpc()
        .user_sign_in(proof_bytes)
        .accounts(
            accounts::UserSignUp {
                user_data: user_data_account,
                user: user_keypair.pubkey(),
                system_program: system_instruction::program_id(),
            }
        )
        .signer(user_keypair)
        .send()?;

    println!("User sign-in completed");
    Ok(())
}

mod accounts {
    use anchor_lang::prelude::*;
    use bls12_381::Scalar;

    #[derive(Accounts)]
    pub struct UserSignUp<'info> {
        #[account(init, payer = user, space = 8 + UserData::INIT_SPACE)] // Adjust space according to UserData struct
        pub user_data: Account<'info, UserData>,
        #[account(mut)]
        pub user: Signer<'info>,
        pub system_program: Program<'info, System>,
    }
    #[account]
    pub struct UserData {
        #[max_len(200)]
        pub public_input: Vec<Scalar>,
    }
}


fn serialize_proof(proof: &Proof<Bls12>) -> Vec<u8> {
    bincode::serialize(proof).expect("Serialization failed")
}
fn serialize_scalars<T: PrimeField>(scalars: &Vec<T>) -> Vec<u8> {
    bincode::serialize(scalars).expect("Serialization failed")
}