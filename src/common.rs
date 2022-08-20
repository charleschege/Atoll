use generic_array::{typenum::U64, GenericArray};

pub const LAMPORTS: u64 = 1_000_000_000;

/// The byte representation of an Ed25519 Signature. Stored as a `GenericArray`
/// since Rust doesn't yet support `u256` primitive due to limitations in LLVM compiler.
pub type SignatureGenericArray = GenericArray<u8, U64>;
