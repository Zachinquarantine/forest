// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use address::Error as AddressError;
use encoding::Error as EncodingError;
use libsecp256k1::Error as SecpError;
use anyhow::Error as AnyhowError;
use thiserror::Error;

/// Crypto error
#[derive(Debug, PartialEq, Error)]
pub enum Error {
    /// Failed to produce a signature
    #[error("Failed to sign data {0}")]
    SigningError(String),
    /// Unable to perform ecrecover with the given params
    #[error("Could not recover public key from signature: {0}")]
    InvalidRecovery(String),
    /// Provided public key is not understood
    #[error("Invalid generated pub key to create address: {0}")]
    InvalidPubKey(#[from] AddressError),
}

impl From<AnyhowError> for Error {
    fn from(err: AnyhowError) -> Error {
        // Pass error encountered in signer trait as module error type
        Error::SigningError(err.to_string())
    }
}

impl From<SecpError> for Error {
    fn from(err: SecpError) -> Error {
        match err {
            SecpError::InvalidRecoveryId => Error::InvalidRecovery(format!("{:?}", err)),
            _ => Error::SigningError(format!("{:?}", err)),
        }
    }
}

impl From<EncodingError> for Error {
    fn from(err: EncodingError) -> Error {
        // Pass error encountered in signer trait as module error type
        Error::SigningError(err.to_string())
    }
}
