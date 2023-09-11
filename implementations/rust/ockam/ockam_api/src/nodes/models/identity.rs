use minicbor::{Decode, Encode};
use ockam_core::compat::borrow::Cow;
use serde::Serialize;

use ockam_core::CowBytes;

#[derive(Debug, Clone, Decode, Encode, Serialize)]
#[rustfmt::skip]
#[cbor(map)]
pub struct LongIdentityResponse<'a> {
    #[b(1)] pub identity: CowBytes<'a>,
}

impl<'a> LongIdentityResponse<'a> {
    pub fn new(identity: impl Into<Cow<'a, [u8]>>) -> Self {
        Self {
            identity: CowBytes(identity.into()),
        }
    }
}

#[derive(Debug, Clone, Decode, Encode, Serialize)]
#[rustfmt::skip]
#[cbor(map)]
pub struct ShortIdentityResponse<'a> {
    #[b(1)] pub identity_id: Cow<'a, str>,
}

impl<'a> ShortIdentityResponse<'a> {
    pub fn new(identity_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            identity_id: identity_id.into(),
        }
    }
}
