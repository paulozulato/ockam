use super::super::IdentityError;
use core::ops::Deref;
use minicbor::bytes::ByteArray;
use minicbor::encode::Write;
use minicbor::{Decode, Decoder, Encode, Encoder};
use ockam_core::{Error, Result};
use ockam_vault::{PublicKey, SecretType};

/// Ed25519 Public Key
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ed25519PublicKey(pub [u8; 32]);

/// X25519 Public Key
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct X25519PublicKey(pub [u8; 32]);

/// P256 Public Key
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct P256ECDSAPublicKey(pub [u8; 64]);

impl<C> Encode<C> for Ed25519PublicKey {
    fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
        ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        ByteArray::from(self.0).encode(e, ctx)
    }
}

impl<'b, C> Decode<'b, C> for Ed25519PublicKey {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let data = ByteArray::<32>::decode(d, ctx)?;

        Ok(Self(*data.deref()))
    }
}

impl<C> Encode<C> for X25519PublicKey {
    fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
        ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        ByteArray::from(self.0).encode(e, ctx)
    }
}

impl<'b, C> Decode<'b, C> for X25519PublicKey {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let data = ByteArray::<32>::decode(d, ctx)?;

        Ok(Self(*data.deref()))
    }
}

impl<C> Encode<C> for P256ECDSAPublicKey {
    fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
        ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        ByteArray::from(self.0).encode(e, ctx)
    }
}

impl<'b, C> Decode<'b, C> for P256ECDSAPublicKey {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let data = ByteArray::<64>::decode(d, ctx)?;

        Ok(Self(*data.deref()))
    }
}

impl From<Ed25519PublicKey> for PublicKey {
    fn from(value: Ed25519PublicKey) -> Self {
        Self::new(value.0.to_vec(), SecretType::Ed25519)
    }
}

impl From<X25519PublicKey> for PublicKey {
    fn from(value: X25519PublicKey) -> Self {
        Self::new(value.0.to_vec(), SecretType::X25519)
    }
}

impl From<P256ECDSAPublicKey> for PublicKey {
    fn from(value: P256ECDSAPublicKey) -> Self {
        Self::new(value.0.to_vec(), SecretType::NistP256)
    }
}

impl TryFrom<PublicKey> for Ed25519PublicKey {
    type Error = Error;

    fn try_from(value: PublicKey) -> Result<Self> {
        match value.stype() {
            SecretType::Ed25519 => {
                let data = value.data()[0..32]
                    .try_into()
                    .map_err(|_| IdentityError::InvalidKeyType)?;
                Ok(Self(data))
            }
            _ => Err(IdentityError::InvalidKeyType.into()),
        }
    }
}

impl TryFrom<PublicKey> for X25519PublicKey {
    type Error = Error;

    fn try_from(value: PublicKey) -> Result<Self> {
        match value.stype() {
            SecretType::X25519 => {
                let data = value.data()[0..32]
                    .try_into()
                    .map_err(|_| IdentityError::InvalidKeyType)?;
                Ok(Self(data))
            }
            _ => Err(IdentityError::InvalidKeyType.into()),
        }
    }
}

impl TryFrom<PublicKey> for P256ECDSAPublicKey {
    type Error = Error;

    fn try_from(value: PublicKey) -> Result<Self> {
        match value.stype() {
            SecretType::NistP256 => {
                let data = value.data()[0..64]
                    .try_into()
                    .map_err(|_| IdentityError::InvalidKeyType)?;
                Ok(Self(data))
            }
            _ => Err(IdentityError::InvalidKeyType.into()),
        }
    }
}
