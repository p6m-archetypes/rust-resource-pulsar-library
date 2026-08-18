use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessagingSettings {
    pub broker_url: String,
    pub topic: String,
    pub access: String,
    pub jwt_token: Option<String>,
    pub subscription_name: Option<String>,
}

impl Default for MessagingSettings {
    fn default() -> Self {
        Self {
            broker_url: broker_url_from_env(),
            topic: std::env::var("MESSAGING_TOPIC")
                .unwrap_or_else(|_| "public/default/events".to_string()),
            access: std::env::var("MESSAGING_ACCESS")
                .unwrap_or_else(|_| "produce".to_string()),
            // "local-dev-token-no-auth" is the sentinel used by the local Pulsar
            // cluster when authentication is disabled — treat it as no-auth.
            jwt_token: std::env::var("MESSAGING_JWT_TOKEN").ok()
                .filter(|t| !t.is_empty() && t != "local-dev-token-no-auth"),
            subscription_name: std::env::var("MESSAGING_SUBSCRIPTION_NAME").ok()
                .filter(|s| !s.is_empty()),
        }
    }
}

// PAO injects connection secret keys from the per-app PulsarCredential secret.
// Secret keys are camelCase (brokerUrl, jwtToken, subscriptionName); PAO's
// sanitize_env_key converts them to UPPER_SNAKE → MESSAGING_BROKER_URL etc.
fn broker_url_from_env() -> String {
    std::env::var("MESSAGING_BROKER_URL")
        .unwrap_or_else(|_| "pulsar://localhost:6650".to_string())
}
