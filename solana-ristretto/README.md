# solana-ristretto
This crate implements types `Scalar` and `RistrettoPoint` that represent
scalars and Ristretto points for curve25519. The arithmetic for the
`RistrettoPoint` types are implemented such that if
`target_os = "solana"`, then syscalls that are implemented in the
Solana runtime are used for the arithmetic..

Everything in this crate is for testing purposes only.
