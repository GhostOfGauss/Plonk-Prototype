pub mod matrix;
pub mod mds;
pub mod round_constant;
pub mod poseidon_ref;
pub mod round_numbers;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PoseidonError{
    #[error("Buffer is full")]
    FullBuffer
}