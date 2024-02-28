//! The generators module contains the API functions for producing a set of
//! generators for a rangeproof.

#[cfg(not(target_os = "solana"))]
use sha3::{Sha3XofReader, Sha3_512, Shake256};
//#[cfg(target_os = "solana")]

use solana_ristretto::{
    constants::{RISTRETTO_BASEPOINT_COMPRESSED, RISTRETTO_BASEPOINT_POINT},
    ristretto::{RistrettoPoint, Scalar},
};

pub struct PedersenGens {
    /// Base for the committed value
    pub B: RistrettoPoint,
    /// Base for the blinding factor
    pub B_blinding: RistrettoPoint,
}

impl PedersenGens {
    pub fn commit(&self, value: Scalar, blinding: Scalar) -> RistrettoPoint {
        RistrettoPoint::multiscalar_multiply(&[value, blinding], &[self.B, self.B_blinding])
    }
}

impl Default for PedersenGens {
    fn default() -> Self {
        PedersenGens {
            B: RISTRETTO_BASEPOINT_POINT,
            B_blinding: RistrettoPoint::hash_from_bytes(RISTRETTO_BASEPOINT_COMPRESSED.as_bytes()),
        }
    }
}

// line 247
pub struct BulletproofGens {
    /// The maximum number of usable generators for each party.
    pub gens_capacity: usize,
    /// Number of values or parties
    pub party_capacity: usize,
    /// Precomputed \\(\mathbf G\\) generators for each party.
    G_vec: Vec<Vec<RistrettoPoint>>,
    /// Precomputed \\(\mathbf H\\) generators for each party.
    H_vec: Vec<Vec<RistrettoPoint>>,
}

impl BulletproofGens {
    pub fn new(gens_capacity: usize, party_capacity: usize) -> Self {
        let mut gens = BulletproofGens {
            gens_capacity: 0,
            party_capacity,
            G_vec: (0..party_capacity).map(|_| Vec::new()).collect(),
            H_vec: (0..party_capacity).map(|_| Vec::new()).collect(),
        };
        gens.increase_capacity(gens_capacity);
        gens
    }

    /// Increases the generators' capacity to the amount specified.
    /// If less than or equal to the current capacity, does nothing.
    pub fn increase_capacity(&mut self, new_capacity: usize) {
        if self.gens_capacity >= new_capacity {
            return;
        }

        for i in 0..self.party_capacity {
            let party_index = i as u32;
            let mut label = [b'G', 0, 0, 0, 0];
            // TODO little endian stuff
            self.G_vec[i].extend(
                &mut GeneratorsChain::new(&label)
                    .fast_forward(self.gens_capacity)
                    .take(new_capacity - self.gens_capacity),
            );
        }
        self.gens_capacity = new_capacity;
    }

    /// Return an iterator over the aggregation of the parties' G generators with given size `n`.
    pub(crate) fn G(&self, n: usize, m: usize) -> impl Iterator<Item = &RistrettoPoint> {
        AggregatedGensIter {
            n,
            m,
            array: &self.G_vec,
            party_idx: 0,
            gen_idx: 0,
        }
    }

    /// Return an iterator over the aggregation of the parties' H generators with given size `n`.
    pub(crate) fn H(&self, n: usize, m: usize) -> impl Iterator<Item = &RistrettoPoint> {
        AggregatedGensIter {
            n,
            m,
            array: &self.H_vec,
            party_idx: 0,
            gen_idx: 0,
        }
    }
}

struct GeneratorsChain {
    // if executed off-chain, use sha3 library
    #[cfg(not(target_os = "solana"))]
    reader: Sha3XofReader,
    // if executed on-chain, use the solana syscalls for sha3
    //#[cfg(target_os = "solana")]
    // TODO? syscall?
}

impl GeneratorsChain {
    /// Creates a chain of generators, determined by the hash of `label`.
    fn new(label: &[u8]) -> Self {
        // if executed off-chain, use sha3 library
        #[cfg(not(target_os = "solana"))]
        {
            let mut shake = Shake256::default();
            shake.input(b"GeneratorsChain");
            shake.input(label);

            GeneratorsChain {
                reader: shake.xof_result(),
            }
        }
    }

    /// Advances the reader n times, squeezing and discarding
    /// the result.
    fn fast_forward(mut self, n: usize) -> Self {
        for _ in 0..n {
            let mut buf = [0u8; 64];
            self.reader.read(&mut buf);
        }
        self
    }
}

//TODO Is that needed?
impl Default for GeneratorsChain {
    fn default() -> Self {
        Self::new(&[])
    }
}

struct AggregatedGensIter<'a> {
    array: &'a Vec<Vec<RistrettoPoint>>,
    n: usize,
    m: usize,
    party_idx: usize,
    gen_idx: usize,
}

impl<'a> Iterator for AggregatedGensIter<'a> {
    type Item = &'a RistrettoPoint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.gen_idx >= self.n {
            self.gen_idx = 0;
            self.party_idx += 1;
        }

        if self.party_idx >= self.m {
            None
        } else {
            let cur_gen = self.gen_idx;
            self.gen_idx += 1;
            Some(&self.array[self.party_idx][cur_gen])
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.n * (self.m - self.party_idx) - self.gen_idx;
        (size, Some(size))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO implement tests here
}
