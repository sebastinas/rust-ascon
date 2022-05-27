//! # [Authenticated Encryption and Associated Data (AEAD)][1] with [Ascon][2]
//!
//! ## Security Notes
//!
//! This crate has received no security audit. Use at your own risk.
//!
//! ## Usage
//!
//! Simple usage (allocating, no associated data):
//!
//! ```
//! use ascon_aead::{Ascon128, Ascon128Key, Nonce}; // Or `Ascon128a`
//! use ascon_aead::aead::{Aead, NewAead};
//!
//! let key = Ascon128Key::from_slice(b"very secret key.");
//! let cipher = Ascon128::new(key);
//!
//! let nonce = Nonce::from_slice(b"unique nonce 012"); // 128-bits; unique per message
//!
//! let ciphertext = cipher.encrypt(nonce, b"plaintext message".as_ref())
//!     .expect("encryption failure!"); // NOTE: handle this error to avoid panics!
//!
//! let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
//!     .expect("decryption failure!"); // NOTE: handle this error to avoid panics!
//!
//! assert_eq!(&plaintext, b"plaintext message");
//! ```
//!
//! ## In-place Usage (eliminates `alloc` requirement)
//!
//! Similar to other crates implementing [`aead`] interfaces, this crate also offers an optional
//! `alloc` feature which can be disabled in e.g. microcontroller environments that don't have a
//! heap. See [`aead::AeadInPlace`] for more details.
//!
//! ```
//! # #[cfg(feature = "heapless")] {
//! use ascon_aead::{Ascon128, Ascon128Key, Nonce}; // Or `Ascon128a`
//! use ascon_aead::aead::{AeadInPlace, NewAead};
//! use ascon_aead::aead::heapless::Vec;
//!
//! let key = Ascon128Key::from_slice(b"very secret key.");
//! let cipher = Ascon128::new(key);
//!
//! let nonce = Nonce::from_slice(b"unique nonce 012"); // 128-bits; unique per message
//!
//! let mut buffer: Vec<u8, 128> = Vec::new(); // Buffer needs 16-bytes overhead for authentication tag
//! buffer.extend_from_slice(b"plaintext message");
//!
//! // Encrypt `buffer` in-place, replacing the plaintext contents with ciphertext
//! cipher.encrypt_in_place(nonce, b"", &mut buffer).expect("encryption failure!");
//!
//! // `buffer` now contains the message ciphertext
//! assert_ne!(&buffer, b"plaintext message");
//!
//! // Decrypt `buffer` in-place, replacing its ciphertext context with the original plaintext
//! cipher.decrypt_in_place(nonce, b"", &mut buffer).expect("decryption failure!");
//! assert_eq!(&buffer, b"plaintext message");
//! # }
//! ```
//!
//! [1]: https://en.wikipedia.org/wiki/Authenticated_encryption
//! [2]: https://ascon.iaik.tugraz.at/index.html

#![no_std]
#![warn(missing_docs)]

mod asconcore;

pub use aead::{
    self,
    consts::{U0, U16},
    AeadCore, AeadInPlace, Buffer, Error, Key, NewAead,
};
use asconcore::Core;
pub use asconcore::{Nonce, Parameters, Parameters128, Parameters128a, Parameters80pq, Tag};

/// Ascon generic over some Parameters
///
/// This type is generic to support substituting various Ascon parameter sets. It is not intended to
/// use directly. Use the [`Ascon128`] and [`Ascon128a`] type aliases instead.
#[derive(Clone)]
pub struct Ascon<P: Parameters> {
    key: P::InternalKey,
}

/// Ascon-128
pub type Ascon128 = Ascon<Parameters128>;
/// Key for Ascon-128
pub type Ascon128Key = Key<Ascon128>;
/// Ascon-128a
pub type Ascon128a = Ascon<Parameters128a>;
/// Key for Ascon-128a
pub type Ascon128aKey = Key<Ascon128a>;

impl<P: Parameters> NewAead for Ascon<P> {
    type KeySize = P::KeySize;

    fn new(key: &Key<Self>) -> Self {
        Self {
            key: P::InternalKey::from(key),
        }
    }
}

impl<P: Parameters> AeadCore for Ascon<P> {
    type NonceSize = U16;
    type TagSize = U16;
    type CiphertextOverhead = U0;
}

impl<P: Parameters> AeadInPlace for Ascon<P> {
    fn encrypt_in_place_detached(
        &self,
        nonce: &Nonce,
        associated_data: &[u8],
        buffer: &mut [u8],
    ) -> Result<Tag, Error> {
        if (buffer.len() as u64)
            .checked_add(associated_data.len() as u64)
            .is_none()
        {
            return Err(Error);
        }

        let mut core = Core::<P>::new(&self.key, nonce);
        Ok(core.encrypt_inplace(buffer, associated_data))
    }

    fn decrypt_in_place_detached(
        &self,
        nonce: &Nonce,
        associated_data: &[u8],
        buffer: &mut [u8],
        tag: &Tag,
    ) -> Result<(), Error> {
        if (buffer.len() as u64)
            .checked_add(associated_data.len() as u64)
            .is_none()
        {
            return Err(Error);
        }

        let mut core = Core::<P>::new(&self.key, nonce);
        core.decrypt_inplace(buffer, associated_data, tag)
    }
}
