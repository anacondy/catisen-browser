// Catisen Browser - Decentralized Cryptographic Sync Chain
// P0 9.4: 32 random bytes -> 64 hex chars; no secret logging; pair_device not implemented.

use rand::RngCore;
use qrcode::QrCode;
use image::{Luma, ImageBuffer};

pub struct SyncChain {
    pub device_seed: String,
    #[allow(dead_code)]
    pub is_synced: bool,
}

impl SyncChain {
    pub fn new() -> Self {
        SyncChain {
            device_seed: String::new(),
            is_synced: false,
        }
    }

    /// Generates a cryptographic identity: 32 random bytes rendered as 64 lowercase hex chars.
    /// P0 9.4: previously 12 words from 12-word dictionary (43 bits). Now 256 bits.
    pub fn generate_new_identity(&mut self) -> String {
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        // 64 lowercase hex chars
        let mut hex = String::with_capacity(64);
        for b in bytes {
            use std::fmt::Write;
            let _ = write!(&mut hex, "{:02x}", b);
        }
        self.device_seed = hex.clone();
        eprintln!("[SyncChain] New identity generated (256-bit)");
        self.device_seed.clone()
    }

    /// Takes the seed and converts it into a raw string payload
    pub fn export_as_qr_payload(&self) -> String {
        if self.device_seed.is_empty() {
            return "ERROR: No sync chain established.".to_string();
        }
        // No secret in logs per P0
        let payload = format!("catisen-sync://{}", self.device_seed.replace(' ', "-"));
        payload
    }

    /// Renders the device sync seed into a visual QR Code image
    pub fn generate_visual_qr_matrix(&self) -> Result<ImageBuffer<Luma<u8>, Vec<u8>>, String> {
        let text_payload = self.export_as_qr_payload();
        if text_payload.starts_with("ERROR") {
            return Err("Cannot generate QR code. User has no identity seed.".to_string());
        }

        let code = QrCode::new(text_payload.as_bytes()).map_err(|e| e.to_string())?;

        let image = code.render::<Luma<u8>>().build();

        eprintln!("[SyncChain] QR matrix rendered ({}x{})", image.width(), image.height());
        Ok(image)
    }

    /// Pairing not implemented yet — must fail closed, not print fake success.
    #[allow(dead_code)]
    pub fn pair_device(_phrase: &str) -> Result<(), String> {
        Err("not implemented".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_chain_generation_and_export() {
        let mut chain = SyncChain::new();

        let initial_payload = chain.export_as_qr_payload();
        assert_eq!(initial_payload, "ERROR: No sync chain established.");

        let identity = chain.generate_new_identity();
        // 64 hex chars
        assert_eq!(identity.len(), 64);
        assert!(identity.chars().all(|c| c.is_ascii_hexdigit()));
        // lowercase
        assert_eq!(identity, identity.to_ascii_lowercase());
        // no spaces
        assert!(!identity.contains(' '));

        let payload = chain.export_as_qr_payload();
        assert!(payload.starts_with("catisen-sync://"));
        assert!(!payload.contains(' '));
        // payload should contain the hex identity
        assert!(payload.contains(&identity));

        // pair_device must return Err per P0
        assert!(SyncChain::pair_device("any phrase").is_err());
        assert!(SyncChain::pair_device(&identity).is_err());
    }

    #[test]
    fn identities_differ() {
        let mut a = SyncChain::new();
        let mut b = SyncChain::new();
        let id_a = a.generate_new_identity();
        let id_b = b.generate_new_identity();
        // Extremely unlikely to collide (2^-256); test that two sequential generations differ
        // Allow small flake? In practice they will differ; if they collide once in 2^256 it's okay to fail.
        assert_ne!(id_a, id_b, "two random identities should differ");
    }
}
