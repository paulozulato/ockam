use std::collections::BTreeMap;

use minicbor::{Decode, Encode};

use ockam::identity::credential::{Attributes, Timestamp};
use ockam::identity::IdentityIdentifier;
use ockam_core::compat::borrow::Cow;
use ockam_core::CowBytes;

#[derive(Debug, Decode, Encode)]
#[rustfmt::skip]
#[cbor(map)]
pub struct VerifyRequest<'a> {
    #[b(1)] cred: CowBytes<'a>,
    #[n(2)] subj: IdentityIdentifier,
    #[b(3)] auth: BTreeMap<IdentityIdentifier, CowBytes<'a>>
}

#[derive(Debug, Decode, Encode)]
#[rustfmt::skip]
#[cbor(map)]
pub struct VerifyResponse {
    #[b(1)] attrs: Attributes,
    #[n(2)] expires: Timestamp
}

impl<'a> VerifyRequest<'a> {
    pub fn new<C: Into<Cow<'a, [u8]>>>(cred: C, subj: IdentityIdentifier) -> Self {
        Self {
            cred: CowBytes(cred.into()),
            subj,
            auth: BTreeMap::new(),
        }
    }

    pub fn with_authority<T>(mut self, id: IdentityIdentifier, identity: T) -> Self
    where
        T: Into<Cow<'a, [u8]>>,
    {
        self.auth.insert(id, CowBytes(identity.into()));
        self
    }

    pub fn credential(&self) -> &[u8] {
        &self.cred
    }

    pub fn subject(&self) -> &IdentityIdentifier {
        &self.subj
    }

    pub fn authorities(&self) -> &BTreeMap<IdentityIdentifier, CowBytes<'a>> {
        &self.auth
    }

    pub fn authority(&self, id: &IdentityIdentifier) -> Option<&CowBytes<'a>> {
        self.auth.get(id)
    }
}

impl VerifyResponse {
    pub fn new(attrs: Attributes, expires: Timestamp) -> Self {
        Self { attrs, expires }
    }

    pub fn attributes(&self) -> &Attributes {
        &self.attrs
    }

    pub fn expires_at(&self) -> Timestamp {
        self.expires
    }
}
