// Catisen Hardware & Prompt Permissions Engine
// Regulates access to sensitive User APIs (Camera, Microphone, Geolocation)
// Default posture is ALWAYS 'Ask' or 'Deny'

use std::collections::HashMap;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum PermissionType {
    Geolocation,
    Camera,
    Microphone,
    Notifications,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PermissionState {
    Granted,
    Denied,
    Ask, // Appears as a pop-up in the UI pausing execution until the user clicks
}

#[derive(Debug, Default)]
pub struct PermissionManager {
    // Maps a domain (String) and a PermissionType to a user-defined State
    pub site_permissions: HashMap<(String, PermissionType), PermissionState>,
}

impl PermissionManager {
    pub fn new() -> Self {
        PermissionManager {
            site_permissions: HashMap::new(),
        }
    }

    /// Queries the active permission state for a given website domain and hardware feature.
    /// Default capability for unknown sites is 'Ask', triggering a user UI interaction.
    pub fn query_permission(&self, domain: &str, p_type: PermissionType) -> PermissionState {
        self.site_permissions
            .get(&(domain.to_string(), p_type.clone()))
            .cloned()
            .unwrap_or(PermissionState::Ask)
    }

    /// Updates a permission selection based on User input
    pub fn set_permission(&mut self, domain: &str, p_type: PermissionType, state: PermissionState) {
        // Tor/Privacy Security Behavior: Unencrypted sites should NEVER be allowed 
        // to query your camera/microphone. It's a massive deanonymization risk!
        if !domain.starts_with("https://") && !domain.contains(".onion") {
            println!("🚫 SECURITY BLOCK: Auto-denying Hardware access for unencrypted plain HTTP site {}", domain);
            self.site_permissions.insert((domain.to_string(), p_type), PermissionState::Denied);
            return;
        }

        self.site_permissions.insert((domain.to_string(), p_type), state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strict_http_rejection() {
        let mut manager = PermissionManager::new();
        // Trying to grant camera to an insecure site
        manager.set_permission("http://insecure.test", PermissionType::Camera, PermissionState::Granted);
        
        // It must automatically fallback/force Denied according to the security framework
        let result = manager.query_permission("http://insecure.test", PermissionType::Camera);
        assert_eq!(result, PermissionState::Denied);
    }
}