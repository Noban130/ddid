mod account;
use account::*;
mod verify_key;
use verify_key::VERIFYINGKEY;

use anchor_lang::prelude::*;
use bls12_381::{Bls12, Scalar};
use bincode;
use bellman::groth16::{verify_proof, Proof};


declare_id!("EgM2WEYJ6cDjkEmRvN6S5t2CgeJYaRLWK82qwwDBaSBy");

const DISCRIMINATOR : usize = 8;


#[program]
pub mod zk_login {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn user_sign_up(ctx: Context<SignUp>, public_input: Vec<u8>) -> Result<()> {
        
        let sing_up_data = &mut ctx.accounts.user_data;
        sing_up_data.public_input = public_input;
        msg!("user hash: {:?}", sing_up_data.public_input);
        Ok(())
    }

    pub fn user_sign_in(ctx: Context<SignUp>, proof: Proof<Bls12>) -> Result<()> {
        let public_input_bytes = ctx.accounts.user_data.public_input;
 
        msg!("Verify proof...");
        // let result = verify_proof(&pvk, &proof, &x);
        let result = verify_proof(&VERIFYINGKEY, &proof, &public_input);
        println!("Result: {}", result.is_ok());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct SignUp<'info> {
    #[account(
        init,
        payer = user,
        seeds = [b"user_data", user.key().as_ref()],
        bump,
        space = DISCRIMINATOR + UserData::INIT_SPACE
    )]
    pub user_data: Account<'info, UserData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

