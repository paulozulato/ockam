use ockam_core::compat::string::String;
use ockam_core::compat::vec::Vec;
use ockam_core::Result;

use crate::{AuthorityService, Identity, IdentityError};

/// A trust context defines which authorities are trusted to attest to which attributes, within a context.
/// Our first implementation assumes that there is only one authority and it is trusted to attest to all attributes within this context.
#[derive(Clone)]
pub struct TrustContext {
    /// This is the ID of the trust context; which is primarily used for ABAC policies
    id: String,
    /// Authority capable of retrieving credentials
    authority: Option<AuthorityService>,
    /// List of authorities which can be used to verify credentials in addition to the authority
    /// issuing credentials above
    trusted_authorities: Vec<Identity>,
}

impl TrustContext {
    /// Create a new Trust Context
    pub fn new(
        id: String,
        authority: Option<AuthorityService>,
        trusted_authorities: Vec<Identity>,
    ) -> Self {
        Self {
            id,
            authority,
            trusted_authorities,
        }
    }

    /// Return the ID of the Trust Context
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return the Authority of the Trust Context
    pub fn authority(&self) -> Result<&AuthorityService> {
        self.authority
            .as_ref()
            .ok_or_else(|| IdentityError::UnknownAuthority.into())
    }

    /// Return the authority identities attached to this trust context
    pub async fn authorities(&self) -> Result<Vec<Identity>> {
        let mut result = self.trusted_authorities.clone();
        result.push(self.authority()?.identity().await?);
        Ok(result)
    }
}
