# solana-merlin
A version of the rust [merlin](https://github.com/dalek-cryptography/merlin)
using the keccak hash function.

The main type that is implemented by the crate is the `Transcript` type, which
keeps track of incoming/outgoing messages between a sender and a receiver in a
cryptographic protocol. The most natural and efficient way to implement this
type is using a variable length hash function (extendable-output function
(XOF)). However, since an XOF is not yet supported by the Solana runtime,
this crate uses the standard Keccak256 hash function to keep track of the
states needed in a cryptographic protocol.

Everything in this crate is for testing purposes only.
