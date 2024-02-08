#[cfg(not(target_os = "solana"))]
use curve25519_dalek::scalar::Scalar as DalekScalar;
use solana_zk_token_sdk::curve25519::scalar::PodScalar;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Scalar(pub(crate) PodScalar);

impl Scalar {
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0 .0
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let bytes: [u8; 32] = bytes
            .try_into()
            .map_err(|_| "Scalar bytes have invalid length".to_string())?;
        Ok(Self(PodScalar(bytes)))
    }
}

#[cfg(not(target_os = "solana"))]
impl Scalar {
    pub fn add_scalar(self, other: Scalar) -> Result<Self, String> {
        let left: DalekScalar = self.0.try_into().unwrap();
        let right: DalekScalar = other.0.try_into().unwrap();
        let result = left + right;
        Ok(Self(result.into()))
    }

    pub fn subtract_scalar(self, other: Scalar) -> Result<Self, String> {
        let left: DalekScalar = self.0.try_into().unwrap();
        let right: DalekScalar = other.0.try_into().unwrap();
        let result = left - right;
        Ok(Self(result.into()))
    }

    pub fn multiply_scalar(self, other: Scalar) -> Result<Self, String> {
        let left: DalekScalar = self.0.try_into().unwrap();
        let right: DalekScalar = other.0.try_into().unwrap();
        let result = left * right;
        Ok(Self(result.into()))
    }
}
