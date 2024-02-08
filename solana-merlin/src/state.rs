use solana_program::hash::{hashv, Hash};

/// The internal hash state used to keep track of internal state used in Fiat-Shamir transforms.
#[derive(Clone)]
pub(crate) struct HashState {
    state: Hash,
}

impl HashState {
    /// Create a new hash state.
    pub fn new(bytes: &'static [u8]) -> Self {
        let hash = hashv(&[b"INIT_STATE", bytes]);
        Self { state: hash }
    }

    /// Absorb bytes into the hash state.
    pub fn absorb(&mut self, bytes: &[u8]) {
        let hash = hashv(&[b"ABSORB", self.state.as_ref(), bytes]);
        self.state = hash;
    }

    /// Squeeze bytes out from the hash state.
    pub fn squeeze(&mut self) -> Hash {
        let hash = hashv(&[b"SQUEEZE", self.state.as_ref()]);
        self.state = hash;
        hash
    }
}
