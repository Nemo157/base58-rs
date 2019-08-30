#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_results)]
#![warn(variant_size_differences)]

#![allow(unknown_lints)] // For clippy
#![allow(renamed_and_removed_lints)] // clippy namespaced lint compat

#![allow(const_static_lifetime)] // 1.13 compat
#![allow(redundant_field_names)] // 1.13 compat

//! Another [Base58][] codec implementation.
//!
//! Compared to [`base58`][] this is significantly faster at decoding (about
//! 2.4x as fast when decoding 32 bytes), almost the same speed for encoding
//! (about 3% slower when encoding 32 bytes) and doesn't have the 128 byte
//! limitation.
//!
//! Compared to [`rust-base58`][] this is massively faster (over ten times as
//! fast when decoding 32 bytes, almost 40 times as fast when encoding 32
//! bytes) and has no external dependencies.
//!
//! Compared to both this supports a configurable alphabet and user provided
//! buffers for zero-allocation {en,de}coding.
//!
//! [Base58]: https://en.wikipedia.org/wiki/Base58
//! [`base58`]: https://github.com/debris/base58
//! [`rust-base58`]: https://github.com/nham/rust-base58
//!
//! # Optional Features
//!
//! ## `check` (off-by-default)
//!
//! Integrated support for [Base58Check][], this allows automatically
//! calculating the checksum during encoding and verifying during decoding.
//!
//! [Base58Check]: https://en.bitcoin.it/wiki/Base58Check_encoding
//!
//! # Examples
//!
//! ## Basic example
//!
//! ```rust
//! let decoded = bs58::decode("he11owor1d").into_vec().unwrap();
//! let encoded = bs58::encode(decoded).into_string();
//! assert_eq!("he11owor1d", encoded);
//! ```
//!
//! ## Changing the alphabet
//!
//! ```rust
//! let decoded = bs58::decode("he11owor1d")
//!     .with_alphabet(bs58::alphabet::RIPPLE)
//!     .into_vec()
//!     .unwrap();
//! let encoded = bs58::encode(decoded)
//!     .with_alphabet(bs58::alphabet::FLICKR)
//!     .into_string();
//! assert_eq!("4DSSNaN1SC", encoded);
//! ```
//!
//! ## Decoding into an existing buffer
//!
//! ```rust
//! let (mut decoded, mut encoded) = ([0xFF; 8], String::with_capacity(10));
//! bs58::decode("he11owor1d").into(&mut decoded).unwrap();
//! bs58::encode(decoded).into(&mut encoded);
//! assert_eq!("he11owor1d", encoded);
//! ```
//!

#[cfg(feature = "check")]
extern crate sha2;

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod alphabet;
pub mod decode;
pub mod encode;
mod traits;

const CHECKSUM_LEN: usize = 4;

#[allow(deprecated)]
pub use traits::{ FromBase58, ToBase58 };

/// Setup decoder for the given string using the [default alphabet][].
///
/// [default alphabet]: alphabet/constant.DEFAULT.html
///
/// # Examples
///
/// ## Basic example
///
/// ```rust
/// assert_eq!(
///     vec![0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58],
///     bs58::decode("he11owor1d").into_vec().unwrap());
/// ```
///
/// ## Changing the alphabet
///
/// ```rust
/// assert_eq!(
///     vec![0x60, 0x65, 0xe7, 0x9b, 0xba, 0x2f, 0x78],
///     bs58::decode("he11owor1d")
///         .with_alphabet(bs58::alphabet::RIPPLE)
///         .into_vec().unwrap());
/// ```
///
/// ## Decoding into an existing buffer
///
/// ```rust
/// let mut output = [0xFF; 10];
/// assert_eq!(8, bs58::decode("he11owor1d").into(&mut output).unwrap());
/// assert_eq!(
///     [0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58, 0xFF, 0xFF],
///     output);
/// ```
///
/// ## Errors
///
/// ### Invalid Character
///
/// ```rust
/// assert_eq!(
///     bs58::decode::Error::InvalidCharacter { character: 'l', index: 2 },
///     bs58::decode("hello world").into_vec().unwrap_err());
/// ```
///
/// ### Non-ASCII Character
///
/// ```rust
/// assert_eq!(
///     bs58::decode::Error::NonAsciiCharacter { index: 5 },
///     bs58::decode("he11o🇳🇿").into_vec().unwrap_err());
/// ```
///
/// ### Too Small Buffer
///
/// This error can only occur when reading into a provided buffer, when using
/// `.into_vec` a vector large enough is guaranteed to be used.
///
/// ```rust
/// let mut output = [0; 7];
/// assert_eq!(
///     bs58::decode::Error::BufferTooSmall,
///     bs58::decode("he11owor1d").into(&mut output).unwrap_err());
/// ```
pub fn decode<I: AsRef<[u8]>>(input: I) -> decode::DecodeBuilder<'static, I> {
    decode::DecodeBuilder::new(input, alphabet::DEFAULT)
}

/// Setup encoder for the given bytes using the [default alphabet][].
///
/// [default alphabet]: alphabet/constant.DEFAULT.html
///
/// # Examples
///
/// ## Basic example
///
/// ```rust
/// let input = [0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58];
/// assert_eq!("he11owor1d", bs58::encode(input).into_string());
/// ```
///
/// ## Changing the alphabet
///
/// ```rust
/// let input = [0x60, 0x65, 0xe7, 0x9b, 0xba, 0x2f, 0x78];
/// assert_eq!(
///     "he11owor1d",
///     bs58::encode(input)
///         .with_alphabet(bs58::alphabet::RIPPLE)
///         .into_string());
/// ```
///
/// ## Encoding into an existing string
///
/// ```rust
/// let input = [0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58];
/// let mut output = "goodbye world".to_owned();
/// bs58::encode(input).into(&mut output);
/// assert_eq!("he11owor1d", output);
/// ```
pub fn encode<I: AsRef<[u8]>>(input: I) -> encode::EncodeBuilder<'static, I> {
    encode::EncodeBuilder::new(input, alphabet::DEFAULT)
}
