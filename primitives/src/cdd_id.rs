use crate::{investor_zkproof_data::v1, IdentityId};

use codec::{Decode, Encode};
use confidential_identity_v1 as ci_v1;
use confidential_identity_v2 as v2;
use polymesh_primitives_derive::SliceU8StrongTyped;
#[cfg(feature = "std")]
use polymesh_primitives_derive::{DeserializeU8StrongTyped, SerializeU8StrongTyped};
use scale_info::TypeInfo;

/// The investor UID identifies the legal entity of an investor.
/// It should be kept encrypted in order to have the investor's portfolio hidden between several
/// IdentityIds.
///
/// That UID is generated by any trusted CDD provider, based on the investor's Personal
/// Identifiable Information (PII). That process is driven by the specification of the Polymath
/// Unique Identity System (PUIS).
#[derive(Encode, Decode, TypeInfo, SliceU8StrongTyped)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "std",
    derive(SerializeU8StrongTyped, DeserializeU8StrongTyped)
)]
pub struct InvestorUid([u8; 16]);

impl InvestorUid {
    /// Transform into a fixed array of bytes.
    #[inline]
    pub fn to_bytes(self) -> [u8; 16] {
        self.0
    }
}

impl From<[u8; 16]> for InvestorUid {
    fn from(s: [u8; 16]) -> Self {
        Self(s)
    }
}

impl From<[u8; 32]> for InvestorUid {
    fn from(s: [u8; 32]) -> Self {
        let mut short: [u8; 16] = Default::default();
        short.copy_from_slice(&s[..16]);
        Self(short)
    }
}

/// A CDD ID is a link between the investor UID and a certain Identity DID such that no one can
/// extract the investor UID from the CDD ID while the investor can create a Zero Knowledge Proof to
/// prove that that DID belongs to them.
/// The main purpose of such a claim is to preserve privacy of the investor using several identities
/// to handle their portfolio.
#[derive(Encode, Decode, TypeInfo, SliceU8StrongTyped)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "std",
    derive(SerializeU8StrongTyped, DeserializeU8StrongTyped)
)]
pub struct CddId([u8; 32]);

impl CddId {
    /// Creates a new CDD Id for PIUS v1.
    pub fn new_v1(did: IdentityId, investor_uid: InvestorUid) -> Self {
        let cdd_claim_data = v1::InvestorZKProofData::make_cdd_claim(&did, &investor_uid);
        let raw_cdd_id = ci_v1::compute_cdd_id(&cdd_claim_data).compress().to_bytes();

        Self(raw_cdd_id)
    }

    /// Creates a new CDD Id for PIUS v2.
    pub fn new_v2(did: IdentityId, investor_uid: InvestorUid) -> Self {
        use v2::ProviderTrait;

        let data = v2::CddClaimData::new(did.as_bytes(), investor_uid.as_slice());
        let raw = v2::claim_proofs::Provider::create_cdd_id(&data)
            .0
            .compress()
            .to_bytes();

        Self(raw)
    }

    /// Only the zero-filled `CddId` is considered as invalid.
    #[inline]
    pub fn is_default_cdd(&self) -> bool {
        self.0 == [0u8; 32]
    }
}

impl From<[u8; 32]> for CddId {
    #[inline]
    fn from(data: [u8; 32]) -> Self {
        Self(data)
    }
}

#[cfg(test)]
mod tests {
    use crate::{CddId, IdentityId, InvestorUid};

    #[test]
    fn cdd_id_generation() {
        let alice_id_1 = IdentityId::from(1);
        let alice_id_2 = IdentityId::from(2);
        let alice_uid = InvestorUid::from(b"alice_uid".as_ref());

        let alice_cdd_id_1 = CddId::new_v1(alice_id_1, alice_uid);
        let alice_cdd_id_1_prima = CddId::new_v1(alice_id_1, alice_uid);
        let alice_cdd_id_2 = CddId::new_v1(alice_id_2, alice_uid);

        assert_ne!(alice_id_1, alice_id_2);
        assert_ne!(alice_cdd_id_1, alice_cdd_id_2);
        assert_eq!(alice_cdd_id_1, alice_cdd_id_1_prima);

        let alice_cdd_id_1_v2 = CddId::new_v2(alice_id_1, alice_uid);
        let alice_cdd_id_1_v2_prima = CddId::new_v2(alice_id_1, alice_uid);
        let alice_cdd_id_2_v2 = CddId::new_v2(alice_id_2, alice_uid);

        assert_eq!(alice_cdd_id_1_v2, alice_cdd_id_1_v2_prima);
        assert_ne!(alice_cdd_id_1_v2, alice_cdd_id_2_v2);
    }
}
