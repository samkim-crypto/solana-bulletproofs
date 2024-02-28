use curve25519_dalek::constants::{
    RISTRETTO_BASEPOINT_COMPRESSED as RISTRETTO_BASEPOINT_COMPRESSED_DALEK,
    RISTRETTO_BASEPOINT_POINT as RISTRETTO_BASEPOINT_POINT_DALEK,
};
#[cfg(not(target_os = "solana"))]
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
/*#[cfg(target_os = "solana")]
{

}*/

#[cfg(not(target_os = "solana"))]
pub const RISTRETTO_BASEPOINT_POINT: RistrettoPoint = RISTRETTO_BASEPOINT_POINT_DALEK;
pub const RISTRETTO_BASEPOINT_COMPRESSED: CompressedRistretto =
    RISTRETTO_BASEPOINT_COMPRESSED_DALEK;

//#[cfg(target_os = "solana")]
// TODO base point
