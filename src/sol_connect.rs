use anchor_client;
#[allow(unused_imports)]
pub use solana_client::rpc_client::RpcClient;
pub use borsh::{BorshDeserialize, BorshSerialize};

#[allow(unused_imports)]
pub use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signature::Signature,
    signature::{Keypair, Signer},
    signer::EncodableKey,
    system_program,
    transaction::Transaction,
};
use std::str::FromStr;
use crate::handlers::zkphandler::ScalarWrapper;

#[derive(BorshSerialize, BorshDeserialize)]
#[borsh(crate = "borsh")]
pub struct UserData {
    public_input : Vec<ScalarWrapper>
}

#[derive(BorshSerialize, BorshDeserialize)]
#[borsh(crate = "borsh")]
pub struct SigninData {
    proof_bytes: Vec<u8>,
    vk_to_send : Vec<u8>
}
pub fn user_sign_up(public_input: Vec<ScalarWrapper>) {
     // create a Rpc client connection
     let url = "https://api.devnet.solana.com".to_string();
     let timeout = std::time::Duration::from_secs(50);
     let connection = RpcClient::new_with_timeout(url, timeout);
     let program_id = Pubkey::from_str("EiPQE6iT1GWf8AWrw1XrjmnxGLa63Scuku7eef3v3Smb").unwrap();
    //  let account_new = Keypair::new().pubkey();
     let payer = Keypair::read_from_file("src/wallet-keypair.json").unwrap();

    let seed_text = b"user_data";
    // Convert string to &[u8]
    let seed_text_slice: &[u8] = seed_text;
    let (account_new, _) = Pubkey::find_program_address(&[&seed_text_slice], &program_id);

    let instruction_name = "user_sign_up";
    
    println!("instruction_name:{}", instruction_name);
     //  construct instruction data
     let instruction_data = UserData {
        public_input
     };

     // setup signers
     let signers = &[&payer];
     // set up accounts
     let accounts = vec![
         AccountMeta::new(account_new, false),
         AccountMeta::new_readonly(payer.pubkey(), true),
         AccountMeta::new_readonly(system_program::ID, false),
         ];
         
         println!("Accounts : {:?}", accounts);
     // call signed call
     let _tx_signature = sign_up_call(
         &connection,
         &program_id,
         &payer,
         signers,
         instruction_name,
         instruction_data,
         accounts,
     )
     .unwrap();
 }

 pub fn user_sign_in(proof_bytes: Vec<u8>, vk_to_send : Vec<u8>) {
    // create a Rpc client connection
    let url = "https://api.devnet.solana.com".to_string();
    let timeout = std::time::Duration::from_secs(50);
    let connection = RpcClient::new_with_timeout(url, timeout);
    let program_id = Pubkey::from_str("EiPQE6iT1GWf8AWrw1XrjmnxGLa63Scuku7eef3v3Smb").unwrap();
   //  let account_new = Keypair::new().pubkey();
    let payer = Keypair::read_from_file("src/wallet-keypair.json").unwrap();

   let instruction_name = "user_sign_in";
   
   println!("instruction_name:{}", instruction_name);
    //  construct instruction data
    let instruction_data = SigninData{
        proof_bytes,
        vk_to_send
    };

    // setup signers
    let signers = &[&payer];
    // set up accounts
    let seed_text = b"user_data";
    // Convert string to &[u8]
    let seed_text_slice: &[u8] = seed_text;
    let (account_new, _) = Pubkey::find_program_address(&[&seed_text_slice], &program_id);
    let accounts = vec![
         AccountMeta::new(account_new, false),
         AccountMeta::new_readonly(payer.pubkey(), true),
         AccountMeta::new_readonly(system_program::ID, false),
         ];
     // call signed call
     let _tx_signature = sign_in_call(
        &connection,
        &program_id,
        &payer,
        signers,
        instruction_name,
        instruction_data,
        accounts,
    )
    .unwrap();
}

pub fn sign_up_call(
    connection: &RpcClient,
    program_id: &Pubkey,
    payer: &Keypair,
    signers: &[&Keypair],
    instruction_name: &str,
    instruction_data: UserData,
    accounts: Vec<AccountMeta>,
) -> Result<Signature, Box<dyn std::error::Error>>

{
    // get discriminant
    let instruction_discriminant = get_discriminant("global", instruction_name);

    // construct instruction
    let ix = Instruction::new_with_borsh(
        program_id.clone(),
        &(instruction_discriminant, instruction_data),
        accounts.clone(),
    );

    // get latest block hash
    let blockhash = connection.get_latest_blockhash().unwrap();

    // construct message
    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);

    //construct transaction
    let mut tx = Transaction::new_unsigned(msg);

    // sign transaction
    tx.sign(signers, tx.message.recent_blockhash);

    // send and confirm transaction
    let tx_signature = connection
    .send_and_confirm_transaction_with_spinner(&tx)
    .map_err(|err| {
        println!("Transaction Error : {:?}", err);
        }).unwrap();
    println!("Signed Up Successfuly!. Transaction ID: {}", tx_signature);

    Ok(tx_signature)
}
pub fn sign_in_call(
    connection: &RpcClient,
    program_id: &Pubkey,
    payer: &Keypair,
    signers: &[&Keypair],
    instruction_name: &str,
    instruction_data: SigninData,
    accounts: Vec<AccountMeta>,
) -> Result<Signature, Box<dyn std::error::Error>>

{
    // get discriminant
    let instruction_discriminant = get_discriminant("global", instruction_name);

    // construct instruction
    let ix = Instruction::new_with_borsh(
        program_id.clone(),
        &(instruction_discriminant, instruction_data),
        accounts.clone(),
    );

    // get latest block hash
    let blockhash = connection.get_latest_blockhash().unwrap();

    // construct message
    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);

    //construct transaction
    let mut tx = Transaction::new_unsigned(msg);

    // sign transaction
    tx.sign(signers, tx.message.recent_blockhash);

    // send and confirm transaction
    let tx_signature = connection
    .send_and_confirm_transaction_with_spinner(&tx)
    .map_err(|err| {
        println!("Transaction Error : {:?}", err);
        }).unwrap();
    println!("Signed In Successfuly!. Transaction ID: {}", tx_signature);

    Ok(tx_signature)
}

/// returns function signature
///
/// accepts name space and name function
pub fn get_discriminant(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", namespace, name);

    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(
        &anchor_client::anchor_lang::solana_program::hash::hash(preimage.as_bytes()).to_bytes()
            [..8],
    );
    
    // println!("signature-hash:{:?}", sighash);
    sighash
}