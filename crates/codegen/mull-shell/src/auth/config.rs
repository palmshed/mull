use super::model::TEAM_PRINCIPAL_TYPE;
use crate::env::{PROD_RELAY_WS_URL, PROD_WS_ORIGIN};
use serde::{Deserialize, Serialize};
fn default_oidc_scopes() -> Vec<String> {
    vec![
        "openid".into(),
        "profile".into(),
        "email".into(),
        "offline_access".into(),
        "api:access".into(),
    ]
}
/// Default scopes for the Palmshed OAuth2 provider. Includes `mull-cli:access`
/// which authorizes the token for API proxy requests.
pub fn default_oauth2_scopes() -> Vec<String> {
    vec![
        "openid".into(),
        "profile".into(),
        "email".into(),
        "offline_access".into(),
        "mull-cli:access".into(),
        "api:access".into(),
        "conversations:read".into(),
        "conversations:write".into(),
        "workspaces:read".into(),
        "workspaces:write".into(),
    ]
}
pub fn default_github_scopes() -> Vec<String> {
    vec!["read:user".into(), "user:email".into()]
}
fn default_team_oauth2_scopes() -> Vec<String> {
    vec![
        "profile".into(),
        "offline_access".into(),
        "mull-cli:access".into(),
        "api:access".into(),
        "team:read".into(),
        "conversations:read".into(),
        "conversations:write".into(),
        "workspaces:read".into(),
        "workspaces:write".into(),
    ]
}
/// Pin automatic auth to one method (`[auth] preferred_method` in config.toml).
///
/// When set, only that method is used for automatic selection; if it is
/// unavailable, auth fails (no silent fallthrough to the other method).
/// Unset keeps today's multi-method fallthrough (session preferred when both
/// exist). Config-toml only — not remote settings, settings UI, or env.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreferredAuthMethod {
    /// `PALMSHED_API_KEY` / auth.json `mull::api_key` / per-model BYOK (`api_key`).
    ApiKey,
    /// OIDC / OAuth2 session (`cached_token`, interactive `palmshed.ai` / `oidc`,
    /// including devbox-minted OIDC).
    Oidc,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MullComConfig {
    pub mull_ws_origin: String,
    pub mull_ws_url: String,
    pub token_header: String,
    /// OIDC config for customer-provided IdPs. See [`OidcAuthConfig`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oidc: Option<OidcAuthConfig>,
    /// OAuth2 provider config. When set, preferred over the legacy relay flow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth2: Option<OAuth2ProviderConfig>,
    /// External auth provider command (stdout = token, stderr = user UX, exit 0 = success).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_provider_command: Option<String>,
    /// Login button label (env: `MULL_AUTH_PROVIDER_LABEL`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_provider_label: Option<String>,
    /// Token TTL in seconds for external auth providers that output bare
    /// tokens without `expires_in`. Synthesizes `expires_at` so proactive
    /// refresh works. Env: `MULL_AUTH_TOKEN_TTL`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_token_ttl: Option<u64>,
    /// Admin kill switch: when `Some(true)`, the `api_key` auth method is
    /// neither advertised nor accepted, so `PALMSHED_API_KEY`/per-model credentials
    /// can't bypass the deployment's IdP login. Env: `MULL_DISABLE_API_KEY_AUTH`.
    /// Parity with common force-login-method admin knobs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_api_key_auth: Option<bool>,
    /// Restrict login to a specific team — the login token's team principal must
    /// equal this. Put in `requirements.toml` to enforce as non-overridable policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force_login_team_uuid: Option<ForceLoginTeam>,
    /// Pin automatic auth to `api_key` or `oidc`. When set and the chosen
    /// method is unavailable, auth fails (no fallthrough). Unset keeps
    /// multi-method fallthrough. Config.toml only (`[auth] preferred_method`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_method: Option<PreferredAuthMethod>,
}
/// Team login restriction. TOML string or array; an empty array fails closed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ForceLoginTeam {
    /// The only allowed team.
    Single(String),
    /// Allowed teams; empty = fail closed.
    AnyOf(Vec<String>),
}
/// Customer OIDC Identity Provider configuration (`[mull_com_config.oidc]`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcAuthConfig {
    pub issuer: String,
    pub client_id: String,
    #[serde(default = "default_oidc_scopes")]
    pub scopes: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
}
/// OAuth2 provider configuration (`MULL_OAUTH2_ISSUER` / `MULL_OAUTH2_CLIENT_ID`).
///
/// Uses the standard OAuth 2.1 Auth Code + PKCE flow via [`OidcAuthConfig`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2ProviderConfig {
    pub issuer: String,
    pub client_id: String,
    #[serde(default = "default_oauth2_scopes")]
    pub scopes: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    /// Client-supplied referrer for OAuth usage-attribution analytics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referrer: Option<String>,
}
pub const PALMSHED_OAUTH2_ISSUER: &str = "https://auth.palmshed.ai";
pub const GITHUB_ISSUER: &str = "https://github.com";
pub const GITHUB_AUTHORIZE_URL: &str = "https://github.com/login/oauth/authorize";
pub const GITHUB_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
pub const GITHUB_USER_API_URL: &str = "https://api.github.com/user";
pub const GITHUB_DEVICE_CODE_URL: &str = "https://github.com/login/device/code";

pub fn github_client_secret() -> Option<String> {
    std::env::var("MULL_GITHUB_CLIENT_SECRET").ok()
}
/// Production accounts-app origin allowlist — the only origins builds without
/// non-production builds accept. Lives in its own const, referenced by both
/// profiles below, so the frozen-contract test (monorepo CI compiles with
/// that feature enabled) still pins this production-origin const.
const PROD_ACCOUNTS_APP_ORIGINS: &[&str] = &["https://accounts.palmshed.ai"];
/// See the opt-in non-production feature variant above — builds without
/// the feature accept only the production accounts app.
pub fn allowed_accounts_app_origins() -> Vec<String> {
    PROD_ACCOUNTS_APP_ORIGINS
        .iter()
        .map(|o| o.to_string())
        .collect()
}
/// Build a CORS layer that accepts requests from the accounts-app deployments
/// listed in [`allowed_accounts_app_origins`] for the given HTTP method.
///
/// Callers can chain additional configuration (e.g. `.allow_headers(...)` or
/// `.allow_private_network(true)`) onto the returned layer.
pub fn accounts_app_cors_layer(method: axum::http::Method) -> tower_http::cors::CorsLayer {
    tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::AllowOrigin::list(
            allowed_accounts_app_origins()
                .iter()
                .filter_map(|origin| match origin.parse() {
                    Ok(value) => Some(value),
                    Err(_) => {
                        tracing::warn!(origin, "skipping malformed accounts-app CORS origin");
                        None
                    }
                }),
        ))
        .allow_methods([method])
}
/// Local-dev OAuth2 issuer (accounts-app running on localhost).
const PALMSHED_OAUTH2_LOCAL_ISSUER: &str = "http://localhost:22255";
const DEFAULT_OAUTH2_REFERRER: &str = "mull";
/// Returns `true` when `MULL_LOCAL_AUTH=1` is set,
/// indicating the local accounts-app should be used as the OAuth2 issuer.
pub fn use_local_auth() -> bool {
    std::env::var("MULL_LOCAL_AUTH")
        .map(|v| !v.is_empty() && v != "0")
        .unwrap_or(false)
}
/// Returns the active Palmshed OAuth2 issuer — the local-dev issuer when
/// `MULL_LOCAL_AUTH=1` is set, otherwise the production issuer.
pub fn mull_oauth2_issuer() -> &'static str {
    if use_local_auth() {
        PALMSHED_OAUTH2_LOCAL_ISSUER
    } else {
        PALMSHED_OAUTH2_ISSUER
    }
}
/// Returns `true` if `issuer` is a recognised Palmshed OAuth2 issuer
/// (production **or** local-dev). Use this instead of comparing against
/// [`PALMSHED_OAUTH2_ISSUER`] directly so that local-dev sessions are still
/// treated as first-party Palmshed auth.
pub fn is_mull_oauth2_issuer(issuer: &str) -> bool {
    issuer == PALMSHED_OAUTH2_ISSUER || issuer == PALMSHED_OAUTH2_LOCAL_ISSUER
}
pub fn is_github_issuer(issuer: &str) -> bool {
    issuer == GITHUB_ISSUER
}
/// auth.json scope key used by the pre-OIDC `mull login --legacy` flow.
/// Matches the key format produced by the original `accounts.palmshed.ai` relay auth.
pub const LEGACY_AUTH_SCOPE: &str = "https://accounts.palmshed.ai/sign-in";
impl MullComConfig {
    /// Whether `api_key` auth is disabled. Pinning a team
    /// (`force_login_team_uuid`) implies this — team membership can't be verified
    /// from a bare API key, so it must go through IdP login. The
    /// `MULL_DISABLE_API_KEY_AUTH` env lockdown is sticky: because the env value
    /// seeds `default()` (the merge base), a lower-trust user `config.toml` could
    /// otherwise set `disable_api_key_auth = false` and override it — so the env
    /// is OR-ed in here and cannot be turned back off by a user layer. Trusted
    /// `requirements.toml` already wins over `config.toml` via layer precedence.
    pub fn api_key_auth_disabled(&self) -> bool {
        self.disable_api_key_auth == Some(true)
            || self.force_login_team_uuid.is_some()
            || env_lockdown_forced()
    }
    /// When `preferred_method = api_key`, automatic OIDC paths (devbox mint,
    /// interactive browser login, external auth provider) must not run — the
    /// pin is fail-closed. Explicit `mull login --devbox` / `--api-key` bypass
    /// this by not consulting automatic flow helpers.
    pub fn blocks_automatic_oidc(&self) -> bool {
        matches!(self.preferred_method, Some(PreferredAuthMethod::ApiKey))
    }
    /// The auth.json scope key for this config.
    pub fn auth_scope(&self) -> String {
        if let Some(ref oidc) = self.oidc {
            format!("{}::{}", oidc.issuer.trim_end_matches('/'), oidc.client_id)
        } else if let Some(ref oauth2) = self.oauth2 {
            oauth2.auth_scope()
        } else {
            unreachable!("oauth2 config is always present (Palmshed default or env override)")
        }
    }
}
impl OAuth2ProviderConfig {
    pub fn is_team_principal(&self) -> bool {
        self.principal_type.as_deref() == Some(TEAM_PRINCIPAL_TYPE)
    }
    pub fn from_env() -> Option<Self> {
        // GitHub OAuth: MULL_GITHUB_CLIENT_ID
        if let Ok(client_id) = std::env::var("MULL_GITHUB_CLIENT_ID") {
            return Some(Self {
                issuer: GITHUB_ISSUER.to_owned(),
                client_id,
                scopes: std::env::var("MULL_GITHUB_SCOPES")
                    .map(|s| s.split(',').map(|s| s.trim().to_owned()).collect())
                    .unwrap_or_else(|_| default_github_scopes()),
                principal_type: None,
                principal_id: None,
                referrer: None,
            });
        }
        let issuer = std::env::var("MULL_OAUTH2_ISSUER").ok()?;
        let client_id = std::env::var("MULL_OAUTH2_CLIENT_ID").ok()?;
        let principal_type = std::env::var("MULL_OAUTH2_PRINCIPAL_TYPE").ok();
        let principal_id = std::env::var("MULL_OAUTH2_PRINCIPAL_ID").ok();
        let default_scopes = match principal_type.as_deref() {
            Some(TEAM_PRINCIPAL_TYPE) => default_team_oauth2_scopes(),
            _ => default_oauth2_scopes(),
        };
        Some(Self {
            issuer,
            client_id,
            scopes: std::env::var("MULL_OAUTH2_SCOPES")
                .map(|s| s.split(',').map(|s| s.trim().to_owned()).collect())
                .unwrap_or(default_scopes),
            principal_type,
            principal_id,
            referrer: Some(
                std::env::var("MULL_OAUTH2_REFERRER")
                    .unwrap_or_else(|_| DEFAULT_OAUTH2_REFERRER.to_owned()),
            ),
        })
    }
    /// Convert to [`OidcAuthConfig`] to reuse the OIDC login flow.
    pub fn as_oidc(&self) -> OidcAuthConfig {
        OidcAuthConfig {
            issuer: self.issuer.clone(),
            client_id: self.client_id.clone(),
            scopes: self.scopes.clone(),
            audience: None,
        }
    }
    pub fn base_auth_scope(&self) -> String {
        format!("{}::{}", self.issuer.trim_end_matches('/'), self.client_id)
    }
    pub fn auth_scope(&self) -> String {
        self.base_auth_scope()
    }
}
impl Default for MullComConfig {
    fn default() -> Self {
        let oidc = OidcAuthConfig::from_env();
        let oauth2 = if oidc.is_some() {
            None
        } else {
            Some(
                OAuth2ProviderConfig::from_env().unwrap_or_else(|| OAuth2ProviderConfig {
                    issuer: mull_oauth2_issuer().to_owned(),
                    client_id: obfstr::obfstr!("b1a00492-073a-47ea-816f-4c329264a828").to_owned(),
                    scopes: default_oauth2_scopes(),
                    principal_type: None,
                    principal_id: None,
                    referrer: Some(DEFAULT_OAUTH2_REFERRER.to_owned()),
                }),
            )
        };
        Self {
            mull_ws_origin: std::env::var("MULL_WS_ORIGIN")
                .unwrap_or_else(|_| PROD_WS_ORIGIN.to_owned()),
            mull_ws_url: std::env::var("MULL_WS_URL")
                .unwrap_or_else(|_| PROD_RELAY_WS_URL.to_owned()),
            token_header: "mull-cli".to_owned(),
            oidc,
            oauth2,
            auth_provider_command: std::env::var("MULL_AUTH_PROVIDER_COMMAND").ok(),
            auth_provider_label: std::env::var("MULL_AUTH_PROVIDER_LABEL").ok(),
            auth_token_ttl: std::env::var("MULL_AUTH_TOKEN_TTL")
                .ok()
                .and_then(|v| v.parse().ok()),
            disable_api_key_auth: std::env::var("MULL_DISABLE_API_KEY_AUTH")
                .ok()
                .map(|v| env_flag_enabled(&v)),
            force_login_team_uuid: None,
            preferred_method: None,
        }
    }
}
/// Parse a boolean env-var value for mull's on/off flags. A bare presence
/// enables the flag, but the common falsy spellings (`0`, `false`, `off`,
/// `no`, empty) count as disabled — so e.g. `MULL_DISABLE_API_KEY_AUTH=false`
/// does NOT turn the kill switch on.
fn env_flag_enabled(value: &str) -> bool {
    !matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "" | "0" | "false" | "off" | "no"
    )
}
/// True when the admin has set `MULL_DISABLE_API_KEY_AUTH` to a truthy value in
/// the process environment. Read live (call-time) and OR-ed into
/// `api_key_auth_disabled()` so the env lockdown is non-overridable by a
/// user-layer `config.toml`.
fn env_lockdown_forced() -> bool {
    std::env::var("MULL_DISABLE_API_KEY_AUTH")
        .ok()
        .is_some_and(|v| env_flag_enabled(&v))
}
impl OidcAuthConfig {
    pub fn from_env() -> Option<Self> {
        let issuer = std::env::var("MULL_OIDC_ISSUER").ok()?;
        let client_id = std::env::var("MULL_OIDC_CLIENT_ID").ok()?;
        Some(Self {
            issuer,
            client_id,
            scopes: std::env::var("MULL_OIDC_SCOPES")
                .map(|s| s.split(',').map(|s| s.trim().to_owned()).collect())
                .unwrap_or_else(|_| default_oidc_scopes()),
            audience: std::env::var("MULL_OIDC_AUDIENCE").ok(),
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn team_auth_scope_is_base_scope() {
        let cfg = OAuth2ProviderConfig {
            issuer: "https://auth.palmshed.ai".into(),
            client_id: "client-123".into(),
            scopes: default_team_oauth2_scopes(),
            principal_type: Some("Team".into()),
            principal_id: Some("team-abc".into()),
            referrer: Some("mull".into()),
        };
        assert_eq!(cfg.auth_scope(), "https://auth.palmshed.ai::client-123");
    }
    #[test]
    fn env_flag_enabled_treats_falsy_spellings_as_off() {
        for off in ["", " ", "0", "false", "FALSE", "off", "No", "  false  "] {
            assert!(!env_flag_enabled(off), "{off:?} should be off");
        }
        for on in ["1", "true", "yes", "on", "enabled"] {
            assert!(env_flag_enabled(on), "{on:?} should be on");
        }
    }
    #[test]
    fn personal_auth_scope_is_base_scope() {
        let cfg = OAuth2ProviderConfig {
            issuer: "https://auth.palmshed.ai".into(),
            client_id: "client-123".into(),
            scopes: default_oauth2_scopes(),
            principal_type: None,
            principal_id: None,
            referrer: Some("mull".into()),
        };
        assert_eq!(cfg.auth_scope(), "https://auth.palmshed.ai::client-123");
    }
    /// FROZEN loopback contract: the accounts-app origins the CLI's loopback
    /// callback server accepts cross-origin requests from. The consent page
    /// (served from accounts.palmshed.ai) delivers the code via `fetch(..., cors)`, so
    /// removing an origin breaks loopback delivery for already-installed CLIs.
    /// Keep in sync with the oauth2-provider / accounts-app deployments.
    /// Non-production / local-dev origins are opt-in only.
    #[test]
    fn allowed_accounts_app_origins_are_frozen() {
        assert_eq!(PROD_ACCOUNTS_APP_ORIGINS, &["https://accounts.palmshed.ai"]);
        assert_eq!(allowed_accounts_app_origins(), PROD_ACCOUNTS_APP_ORIGINS);
    }
    /// FROZEN client contract: the 10 scopes the Palmshed OAuth2 client requests.
    /// The server must keep accepting all of them; existing tokens carry
    /// exactly this set. Frozen OAuth client scope contract.
    #[test]
    fn default_oauth2_scopes_are_frozen() {
        let scopes = default_oauth2_scopes();
        let scopes: Vec<&str> = scopes.iter().map(String::as_str).collect();
        assert_eq!(
            scopes,
            [
                "openid",
                "profile",
                "email",
                "offline_access",
                "mull-cli:access",
                "api:access",
                "conversations:read",
                "conversations:write",
                "workspaces:read",
                "workspaces:write",
            ]
        );
    }
    #[test]
    fn preferred_method_deserializes_from_toml() {
        let cfg: MullComConfig = toml::from_str(
            r#"
            preferred_method = "api_key"
            "#,
        )
        .expect("parse");
        assert_eq!(cfg.preferred_method, Some(PreferredAuthMethod::ApiKey));
        let cfg: MullComConfig = toml::from_str(
            r#"
            preferred_method = "oidc"
            "#,
        )
        .expect("parse");
        assert_eq!(cfg.preferred_method, Some(PreferredAuthMethod::Oidc));
        let cfg: MullComConfig = toml::from_str("").expect("parse empty");
        assert_eq!(cfg.preferred_method, None);
    }
}
