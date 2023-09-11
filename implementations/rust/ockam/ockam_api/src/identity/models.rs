#![allow(missing_docs)]

use ockam_core::{CowBytes, CowStr};

use minicbor::{Decode, Encode};

#[derive(Debug, Clone, Encode, Decode)]
#[rustfmt::skip]
#[cbor(map)]
pub struct CreateResponse<'a> {
    #[b(1)] identity: CowBytes<'a>,
    #[b(2)] identity_id: CowStr<'a>,
}

impl<'a> CreateResponse<'a> {
    pub fn new(identity: impl Into<CowBytes<'a>>, identity_id: impl Into<CowStr<'a>>) -> Self {
        Self {
            identity: identity.into(),
            identity_id: identity_id.into(),
        }
    }
    pub fn identity(&self) -> &[u8] {
        &self.identity
    }
    pub fn identity_id(&self) -> &str {
        &self.identity_id
    }
}

#[derive(Debug, Clone, Encode, Decode)]
#[rustfmt::skip]
#[cbor(map)]
pub struct ValidateIdentityChangeHistoryRequest<'a> {
    #[b(1)] identity: CowBytes<'a>,
}

impl<'a> ValidateIdentityChangeHistoryRequest<'a> {
    pub fn new(identity: impl Into<CowBytes<'a>>) -> Self {
        Self {
            identity: identity.into(),
        }
    }
    pub fn identity(&self) -> &[u8] {
        &self.identity
    }
}

#[derive(Debug, Clone, Encode, Decode)]
#[rustfmt::skip]
#[cbor(map)]
pub struct ValidateIdentityChangeHistoryResponse<'a> {
    #[b(1)] identity_id: CowStr<'a>,
}

impl<'a> ValidateIdentityChangeHistoryResponse<'a> {
    pub fn new(identity_id: impl Into<CowStr<'a>>) -> Self {
        Self {
            identity_id: identity_id.into(),
        }
    }
    pub fn identity_id(&self) -> &str {
        &self.identity_id
    }
}

#[derive(Debug, Clone, Encode, Decode)]
#[rustfmt::skip]
#[cbor(map)]
pub struct CompareIdentityChangeHistoryRequest<'a> {
    #[b(1)] current_identity: CowBytes<'a>,
    #[b(2)] known_identity: CowBytes<'a>,
}

impl<'a> CompareIdentityChangeHistoryRequest<'a> {
    pub fn new(
        current_identity: impl Into<CowBytes<'a>>,
        known_identity: impl Into<CowBytes<'a>>,
    ) -> Self {
        Self {
            current_identity: current_identity.into(),
            known_identity: known_identity.into(),
        }
    }
    pub fn current_identity(&self) -> &[u8] {
        &self.current_identity
    }
    pub fn known_identity(&self) -> &[u8] {
        &self.known_identity
    }
}

#[derive(Debug, Clone, Encode, Decode)]
#[rustfmt::skip]
#[cbor(map)]
pub struct CreateSignatureRequest<'a> {
    #[b(1)] identity: CowBytes<'a>,
    #[b(2)] data: CowBytes<'a>,
    #[b(3)] vault_name: Option<CowStr<'a>>,
}

impl<'a> CreateSignatureRequest<'a> {
    pub fn new(identity: impl Into<CowBytes<'a>>, data: impl Into<CowBytes<'a>>) -> Self {
        Self {
            identity: identity.into(),
            data: data.into(),
            vault_name: None,
        }
    }
    pub fn identity(&self) -> &[u8] {
        &self.identity
    }
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    pub fn vault_name(&self) -> Option<String> {
        self.vault_name.as_ref().map(|x| x.to_string())
    }
}

#[derive(Debug, Clone, Encode, Decode)]
#[rustfmt::skip]
#[cbor(map)]
pub struct CreateSignatureResponse<'a> {
    #[b(1)] signature: CowBytes<'a>,
}

impl<'a> CreateSignatureResponse<'a> {
    pub fn new(signature: impl Into<CowBytes<'a>>) -> Self {
        Self {
            signature: signature.into(),
        }
    }
    pub fn signature(&self) -> &[u8] {
        &self.signature
    }
}

#[derive(Debug, Clone, Encode, Decode)]
#[rustfmt::skip]
#[cbor(map)]
pub struct VerifySignatureRequest<'a> {
    #[b(1)] signer_identity: CowBytes<'a>,
    #[b(2)] data: CowBytes<'a>,
    #[b(3)] signature: CowBytes<'a>,
}

impl<'a> VerifySignatureRequest<'a> {
    pub fn new(
        signer_identity: impl Into<CowBytes<'a>>,
        data: impl Into<CowBytes<'a>>,
        signature: impl Into<CowBytes<'a>>,
    ) -> Self {
        Self {
            signer_identity: signer_identity.into(),
            data: data.into(),
            signature: signature.into(),
        }
    }
    pub fn signer_identity(&self) -> &[u8] {
        &self.signer_identity
    }
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    pub fn signature(&self) -> &[u8] {
        &self.signature
    }
}

#[derive(Debug, Clone, Encode, Decode)]
#[rustfmt::skip]
#[cbor(map)]
pub struct VerifySignatureResponse {
    #[n(1)] verified: bool,
}

impl VerifySignatureResponse {
    pub fn new(verified: bool) -> Self {
        Self { verified }
    }
    pub fn verified(&self) -> bool {
        self.verified
    }
}
