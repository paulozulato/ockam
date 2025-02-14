use ockam_core::{
    errcode::{Kind, Origin},
    Error,
};

/// Identity crate error
#[derive(Clone, Copy, Debug)]
pub enum IdentityError {
    /// Invalid key type
    InvalidKeyType = 1,
    /// Invalid Identifier format
    InvalidIdentifier,
    /// Identity Change History is empty
    EmptyIdentity,
    /// Identity Verification Failed
    IdentityVerificationFailed,
    /// PurposeKeyAttestation Verification Failed
    PurposeKeyAttestationVerificationFailed,
    /// Credential Verification Failed
    CredentialVerificationFailed,
    /// Error occurred while getting current UTC Timestamp
    UnknownTimestamp,
    /// Unknown Authority
    UnknownAuthority,
    /// Unknown version of the Credential
    UnknownCredentialVersion,
    /// Unknown version of the Identity
    UnknownIdentityVersion,
    /// SecureChannelVerificationFailed
    SecureChannelVerificationFailed,
    /// SecureChannelTrustCheckFailed
    SecureChannelTrustCheckFailed,
    /// Invalid Nonce value
    InvalidNonce,
    /// Nonce overflow
    NonceOverflow,
    /// Unknown message destination
    UnknownChannelMsgDestination,
    /// Invalid LocalInfo type
    InvalidLocalInfoType,
    /// Duplicate Secure Channel
    DuplicateSecureChannel,
    /// Consistency Error
    ConsistencyError,
}

impl ockam_core::compat::error::Error for IdentityError {}
impl core::fmt::Display for IdentityError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

impl From<IdentityError> for Error {
    #[track_caller]
    fn from(err: IdentityError) -> Self {
        let kind = Kind::Unknown; // FIXME: fill these in with more
                                  // meaningful error kinds
        Error::new(Origin::Identity, kind, err)
    }
}
