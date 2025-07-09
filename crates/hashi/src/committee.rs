use sui_sdk_types::Ed25519PublicKey;

pub struct Committee {
    //
}

pub struct CommitteeMember {
    protocol_public_key: Ed25519PublicKey,
    tls_public_key: Ed25519PublicKey,
    https_address: String,
    weight: u64,
}

impl CommitteeMember {
    pub fn protocol_public_key(&self) -> &Ed25519PublicKey {
        &self.protocol_public_key
    }

    pub fn tls_public_key(&self) -> &Ed25519PublicKey {
        &self.tls_public_key
    }

    pub fn https_address(&self) -> &str {
        &self.https_address
    }

    pub fn weight(&self) -> u64 {
        self.weight
    }
}
