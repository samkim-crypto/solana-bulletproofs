#[cfg(not(target_os = "solana"))]
use curve25519_dalek::constants::{
        RISTRETTO_BASEPOINT_COMPRESSED as RISTRETTO_BASEPOINT_COMPRESSED_DALEK,
        RISTRETTO_BASEPOINT_POINT as RISTRETTO_BASEPOINT_POINT_DALEK,
    };
//#[cfg(target_os = "solana")]

use crate::ristretto::{RistrettoPoint, CompressedRistretto};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref RISTRETTO_BASEPOINT_POINT: RistrettoPoint = {
        // if executed off-chain, use the dalek basepoint
        #[cfg(not(target_os = "solana"))]
        {
            let compresseddalek = RISTRETTO_BASEPOINT_POINT_DALEK.compress();
            let bytes = compresseddalek.as_bytes();
            RistrettoPoint::from_bytes(bytes).unwrap()
        }
        // if executed on-chain, use the solana basepoint
        #[cfg(target_os = "solana")]
        {
            // TODO
        }
    };
}

lazy_static! {
    pub static ref RISTRETTO_BASEPOINT_COMPRESSED: CompressedRistretto = {
        // if executed off-chain, use the dalek basepoint
        #[cfg(not(target_os = "solana"))]
        {
            let compressed_dalek_point = RISTRETTO_BASEPOINT_COMPRESSED_DALEK;
            let bytes = compressed_dalek_point.as_bytes();
            CompressedRistretto(bytes.clone())
        }
        // if executed on-chain, use the solana basepoint
        #[cfg(target_os = "solana")]
        {
            // TODO
        }
    };
}
