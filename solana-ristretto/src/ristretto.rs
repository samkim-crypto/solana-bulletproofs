#[cfg(not(target_os = "solana"))]
use curve25519_dalek::{
    ristretto::CompressedRistretto as DalekCompressedRistrettoPoint,
    ristretto::RistrettoPoint as DalekRistrettoPoint, scalar::Scalar as DalekScalar,
    traits::VartimeMultiscalarMul, digest::{Digest, generic_array::typenum::U64},
};
#[cfg(target_os = "solana")]
use solana_zk_token_sdk::curve25519::ristretto::{
    add_ristretto, multiply_ristretto, multiscalar_multiply_ristretto, subtract_ristretto,
};
use {crate::scalar::Scalar, solana_zk_token_sdk::curve25519::ristretto::PodRistrettoPoint};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RistrettoPoint(pub(crate) PodRistrettoPoint);

impl RistrettoPoint {
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0 .0
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let bytes: [u8; 32] = bytes
            .try_into()
            .map_err(|_| "Point bytes have invalid length".to_string())?;
        Ok(Self(PodRistrettoPoint(bytes)))
    }
}

impl RistrettoPoint {
    pub fn add(&self, other: &Self) -> Result<Self, String> {
        // if executed off-chain, use the dalek implementation
        #[cfg(not(target_os = "solana"))]
        {
            let left: DalekRistrettoPoint = (&self.0).try_into().unwrap();
            let right: DalekRistrettoPoint = (&other.0).try_into().unwrap();
            let result = left + right;
            Ok(Self((&result).into()))
        }
        // if executed on-chain, use the solana syscall
        #[cfg(target_os = "solana")]
        {
            let result = add_ristretto(&self.0, &other.0).unwrap();
            Ok(Self(result))
        }
    }

    pub fn subtract(&self, other: &Self) -> Result<Self, String> {
        // if executed off-chain, use the dalek implementation
        #[cfg(not(target_os = "solana"))]
        {
            let left: DalekRistrettoPoint = (&self.0).try_into().unwrap();
            let right: DalekRistrettoPoint = (&other.0).try_into().unwrap();
            let result = left - right;
            Ok(Self((&result).into()))
        }
        // if executed on-chain, use the solana syscall
        #[cfg(target_os = "solana")]
        {
            let result = subtract_ristretto(&self.0, &other.0).unwrap();
            Ok(Self(result))
        }
    }

    pub fn multiply(&self, other: &Scalar) -> Result<Self, String> {
        // if executed off-chain, use the dalek implementation
        #[cfg(not(target_os = "solana"))]
        {
            let point: DalekRistrettoPoint = (&self.0).try_into().unwrap();
            let scalar: DalekScalar = (other.0).try_into().unwrap();
            let result = point * scalar;
            Ok(Self((&result).into()))
        }
        // if executed on-chain, use the solana syscall
        #[cfg(target_os = "solana")]
        {
            let result = multiply_ristretto(&other.0, &self.0).unwrap();
            Ok(Self(result))
        }
    }
}

impl RistrettoPoint {
    pub fn multiscalar_multiply(
        scalars: &[Scalar],
        points: &[RistrettoPoint],
    ) -> Result<Self, String> {
        #[cfg(not(target_os = "solana"))]
        {
            let scalars = scalars
                .iter()
                .map(|scalar| DalekScalar::try_from(scalar.0).ok())
                .collect::<Option<Vec<_>>>()
                .unwrap();
            let points = points
                .iter()
                .map(|point| DalekRistrettoPoint::try_from(&point.0).ok());
            let result = DalekRistrettoPoint::optional_multiscalar_mul(scalars, points).unwrap();
            Ok(Self((&result).into()))
        }
        #[cfg(target_os = "solana")]
        {
            let scalars = scalars.iter().map(|scalar| scalar.0).collect::<Vec<_>>();
            let points = points.iter().map(|point| point.0).collect::<Vec<_>>();
            let result = multiscalar_multiply_ristretto(&scalars, &points).unwrap();
            Ok(Self(result))
        }
    }
}

impl RistrettoPoint {
    pub fn hash_from_bytes<D>(input: &[u8]) -> RistrettoPoint 
        where D: Digest<OutputSize = U64> + Default 
    {
        // if executed off-chain, use the dalek implementation
        #[cfg(not(target_os = "solana"))]
        {
            let dalekpoint = DalekRistrettoPoint::hash_from_bytes::<D>(input);

            let compresseddalek = dalekpoint.compress();
            let bytes = compresseddalek.as_bytes();
            RistrettoPoint::from_bytes(bytes).unwrap()
        }
        // if executed on-chain, use the solana equivalent from PodRistrettoPoint
        #[cfg(target_os = "solana")]
        {
            // TODO implement
        }
    }
}

pub struct CompressedRistretto(pub [u8; 32]); // TODO construct from solana?

impl CompressedRistretto {
    /// Copy the bytes of this `CompressedRistretto`.
    pub const fn to_bytes(&self) -> [u8; 32] {
        self.0
    }

    // TODO What else do we need implemented here?
}
