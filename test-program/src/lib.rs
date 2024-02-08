use solana_merlin::Transcript;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, instruction::Instruction, msg,
    program_error::ProgramError, pubkey::Pubkey,
};
use solana_ristretto::{ristretto::RistrettoPoint, scalar::Scalar};
use std::mem::size_of;

solana_program::declare_id!("Mer1in1111111111111111111111111111111111111");

#[derive(Clone, Debug, PartialEq)]
pub enum BoomerangTestInstructions {
    /// Test transcript
    Transcript { message: Vec<u8>, append_u64: u64 },

    /// Test add
    AddRistretto {
        left_point: RistrettoPoint,
        right_point: RistrettoPoint,
    },

    /// Test subtract
    SubtractRistretto {
        left_point: RistrettoPoint,
        right_point: RistrettoPoint,
    },

    /// Test multiply
    MultiplyRistretto {
        point: RistrettoPoint,
        scalar: Scalar,
    },

    /// Test multiscalar multiplication
    MultiscalarMultiplyRistretto {
        scalars: Vec<Scalar>,
        points: Vec<RistrettoPoint>,
    },
}

impl BoomerangTestInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        const U32_BYTES: usize = 4;
        const POINT_BYTES: usize = 32;
        const SCALAR_BYTES: usize = 32;

        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        // NOTE: `split_at` can panic on invalid length bytes, but this should suffice for test
        // purposes.
        Ok(match tag {
            0 => {
                let (length, rest) = rest.split_at(U32_BYTES);
                let length = u32::from_le_bytes(
                    length
                        .try_into()
                        .map_err(|_| ProgramError::InvalidInstructionData)?,
                ) as usize;
                let (message, append_u64) = rest.split_at(length);
                let append_u64 = append_u64
                    .try_into()
                    .ok()
                    .map(u64::from_le_bytes)
                    .ok_or(ProgramError::InvalidInstructionData)?;

                Self::Transcript {
                    message: message.to_vec(),
                    append_u64,
                }
            }
            1 => {
                let (left_point, right_point) = rest.split_at(POINT_BYTES);

                let left_point = RistrettoPoint::from_bytes(left_point)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                let right_point = RistrettoPoint::from_bytes(right_point)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;

                Self::AddRistretto {
                    left_point,
                    right_point,
                }
            }
            2 => {
                let (left_point, right_point) = rest.split_at(POINT_BYTES);

                let left_point = RistrettoPoint::from_bytes(left_point)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                let right_point = RistrettoPoint::from_bytes(right_point)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;

                Self::SubtractRistretto {
                    left_point,
                    right_point,
                }
            }
            3 => {
                let (point, scalar) = rest.split_at(POINT_BYTES);

                let point = RistrettoPoint::from_bytes(point)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                let scalar =
                    Scalar::from_bytes(scalar).map_err(|_| ProgramError::InvalidInstructionData)?;

                Self::MultiplyRistretto { point, scalar }
            }
            4 => {
                let (scalars_length, rest) = rest.split_at(U32_BYTES);
                let scalars_length = u32::from_le_bytes(
                    scalars_length
                        .try_into()
                        .map_err(|_| ProgramError::InvalidInstructionData)?,
                ) as usize;

                let scalars_bytes_length = scalars_length
                    .checked_mul(SCALAR_BYTES)
                    .ok_or(ProgramError::InvalidInstructionData)?;
                let (scalars_bytes, rest) = rest.split_at(scalars_bytes_length);

                let mut scalars = Vec::with_capacity(scalars_length);
                for scalar_bytes in scalars_bytes.chunks(SCALAR_BYTES) {
                    let scalar = Scalar::from_bytes(scalar_bytes)
                        .map_err(|_| ProgramError::InvalidInstructionData)?;
                    scalars.push(scalar);
                }

                let (points_length, points_bytes) = rest.split_at(U32_BYTES);
                let points_length = u32::from_le_bytes(
                    points_length
                        .try_into()
                        .map_err(|_| ProgramError::InvalidInstructionData)?,
                ) as usize;

                let points_bytes_length = points_length
                    .checked_mul(POINT_BYTES)
                    .ok_or(ProgramError::InvalidInstructionData)?;

                if points_bytes.len() != points_bytes_length {
                    return Err(ProgramError::InvalidInstructionData);
                }

                let mut points = Vec::with_capacity(points_length);
                for point_bytes in points_bytes.chunks(POINT_BYTES) {
                    let point = RistrettoPoint::from_bytes(point_bytes)
                        .map_err(|_| ProgramError::InvalidInstructionData)?;
                    points.push(point)
                }

                Self::MultiscalarMultiplyRistretto { scalars, points }
            }

            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }

    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match self {
            Self::Transcript {
                message,
                append_u64,
            } => {
                buf.push(0);
                buf.extend_from_slice(&(message.len() as u32).to_le_bytes());
                buf.extend_from_slice(message);
                buf.extend_from_slice(&append_u64.to_le_bytes());
            }
            Self::AddRistretto {
                left_point,
                right_point,
            } => {
                buf.push(1);
                buf.extend_from_slice(&left_point.to_bytes());
                buf.extend_from_slice(&right_point.to_bytes());
            }
            Self::SubtractRistretto {
                left_point,
                right_point,
            } => {
                buf.push(2);
                buf.extend_from_slice(&left_point.to_bytes());
                buf.extend_from_slice(&right_point.to_bytes());
            }
            Self::MultiplyRistretto { point, scalar } => {
                buf.push(3);
                buf.extend_from_slice(&point.to_bytes());
                buf.extend_from_slice(&scalar.to_bytes());
            }
            Self::MultiscalarMultiplyRistretto { scalars, points } => {
                buf.push(4);
                buf.extend_from_slice(&(scalars.len() as u32).to_le_bytes());
                scalars
                    .iter()
                    .for_each(|scalar| buf.extend_from_slice(&scalar.to_bytes()));
                buf.extend_from_slice(&(points.len() as u32).to_le_bytes());
                points
                    .iter()
                    .for_each(|point| buf.extend_from_slice(&point.to_bytes()));
            }
        }
        buf
    }
}

/// Create a `BoomerangTestInstructions::Transcript` instruction
pub fn transcript(message: Vec<u8>, append_u64: u64) -> Instruction {
    Instruction {
        program_id: id(),
        accounts: vec![],
        data: BoomerangTestInstructions::Transcript {
            message,
            append_u64,
        }
        .pack(),
    }
}

/// Create a `BoomerangTestInstructions::AddRistretto` instruction
pub fn add_ristretto(left_point: &RistrettoPoint, right_point: &RistrettoPoint) -> Instruction {
    Instruction {
        program_id: id(),
        accounts: vec![],
        data: BoomerangTestInstructions::AddRistretto {
            left_point: *left_point,
            right_point: *right_point,
        }
        .pack(),
    }
}

/// Create a `BoomerangTestInstructions::SubtractRistretto` instruction
pub fn subtract_ristretto(
    left_point: &RistrettoPoint,
    right_point: &RistrettoPoint,
) -> Instruction {
    Instruction {
        program_id: id(),
        accounts: vec![],
        data: BoomerangTestInstructions::SubtractRistretto {
            left_point: *left_point,
            right_point: *right_point,
        }
        .pack(),
    }
}

/// Create a `BoomerangTestInstructions::MultiplyRistretto` instruction
pub fn multiply_ristretto(point: &RistrettoPoint, scalar: &Scalar) -> Instruction {
    Instruction {
        program_id: id(),
        accounts: vec![],
        data: BoomerangTestInstructions::MultiplyRistretto {
            point: *point,
            scalar: *scalar,
        }
        .pack(),
    }
}

/// Create a `BoomerangTestInstructions::MultiscalarMultiplyRistretto` instruction
pub fn multiscalar_multiply_ristretto(
    scalars: Vec<Scalar>,
    points: Vec<RistrettoPoint>,
) -> Instruction {
    Instruction {
        program_id: id(),
        accounts: vec![],
        data: BoomerangTestInstructions::MultiscalarMultiplyRistretto { points, scalars }.pack(),
    }
}

solana_program::entrypoint!(process_instruction);
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = BoomerangTestInstructions::unpack(instruction_data)?;

    match instruction {
        BoomerangTestInstructions::Transcript {
            message,
            append_u64,
        } => {
            msg!("Instruction: Transcript");

            let mut transcript = Transcript::new(b"example label");
            transcript.append_message(b"sample label", &message);
            transcript.append_u64(b"sample label", append_u64);

            let mut challenge_bytes = [0; 32];
            transcript.challenge_bytes(b"sample challenge", &mut challenge_bytes);
            msg!("Challenge bytes: {:?}", challenge_bytes);

            Ok(())
        }
        BoomerangTestInstructions::AddRistretto {
            left_point,
            right_point,
        } => {
            msg!("Instruction: AddRistretto");

            let result = left_point
                .add(&right_point)
                .map_err(|_| ProgramError::InvalidInstructionData)?;
            msg!("Result: {:?}", result.to_bytes());

            Ok(())
        }
        BoomerangTestInstructions::SubtractRistretto {
            left_point,
            right_point,
        } => {
            msg!("Instruction: SubtractRistretto");

            let result = left_point
                .subtract(&right_point)
                .map_err(|_| ProgramError::InvalidInstructionData)?;
            msg!("Result: {:?}", result.to_bytes());

            Ok(())
        }
        BoomerangTestInstructions::MultiplyRistretto { point, scalar } => {
            msg!("Instruction: MultiplyRistretto");

            let result = point
                .multiply(&scalar)
                .map_err(|_| ProgramError::InvalidInstructionData)?;
            msg!("Result: {:?}", result.to_bytes());

            Ok(())
        }
        BoomerangTestInstructions::MultiscalarMultiplyRistretto { scalars, points } => {
            msg!("Instruction: MultiscalarMultiplyRistretto");

            let result = RistrettoPoint::multiscalar_multiply(&scalars, &points)
                .map_err(|_| ProgramError::InvalidInstructionData)?;
            msg!("Result: {:?}", result.to_bytes());

            Ok(())
        }
    }
}
