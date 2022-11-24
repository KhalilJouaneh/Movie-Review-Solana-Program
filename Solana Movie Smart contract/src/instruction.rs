use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{program_error::ProgramError};

pub enum MovieInstruction {
    AddMovieReview {
        title: String, 
        rating: u8,
        description: String
    }
}

impl MovieInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        
        let payload = MovieReviewPayload::try_from_slice(rest).unwrap();
        //to "unwrap" smthn in Rust is to say "Give me the result of the compuation, and if there was an error, panic and stop the program"
        //

        Ok(match variant {
            0 => Self::AddMovieReview {
                title: payload.title,
                rating: payload.rating,
                description: payload.description },
            _ => return Err(ProgramError::InvalidInstructionData)
            })
        }
    }


#[derive(BorshDeserialize)]
struct MovieReviewPayload {
    title: String,
    rating: u8,
    description: String
}


