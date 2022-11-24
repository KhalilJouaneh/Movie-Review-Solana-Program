use borsh::{BorshSerialize, BorshDeserialize};

//Our program receives the instruction, extracts the data and formats it into a Rust type
//MovieAccountState is a Rust type
#[derive(BorshSerialize, BorshDeserialize)]
pub struct MovieAccountState {
    pub is_initialized: bool,
    pub rating: u8,
    pub title: String,
    pub description: String
}



