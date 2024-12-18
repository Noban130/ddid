use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
pub struct UserData {
  #[max_len=100]
  pub public_input: Vec<u8>,
}