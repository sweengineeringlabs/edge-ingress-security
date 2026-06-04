//! Configuration for the built-in [`MethodAclPolicy`].

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Method-level allowlist keyed by the caller's verified subject
/// (CN for mTLS, JWT `sub` for bearer).
///
/// Entries are exact, case-sensitive matches.  Empty value lists
/// mean "allow all methods" for that subject; absent subjects are
/// denied by default.  A wildcard subject `"*"` applies to every
/// authenticated caller.
///
/// # Examples
///
/// ```rust
/// use swe_edge_ingress_grpc_authz::MethodAclConfig;
///
/// let acl = MethodAclConfig::deny_all()
///     .allow("svc-a", vec!["/pkg.MyService/Query".to_string()])
///     .allow_for_any_authenticated("/grpc.health.v1.Health/Check");
///
/// assert!(acl.by_subject.contains_key("svc-a"));
/// assert!(acl.by_subject.contains_key("*"));
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MethodAclConfig {
    /// `subject -> [method-paths]`.
    #[serde(default)]
    pub by_subject: HashMap<String, Vec<String>>,
}

impl MethodAclConfig {
    /// Build an empty config — denies every authenticated caller.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_ingress_grpc_authz::MethodAclConfig;
    /// let acl = MethodAclConfig::deny_all();
    /// assert!(acl.by_subject.is_empty());
    /// ```
    pub fn deny_all() -> Self {
        Self::default()
    }

    /// Allow `subject` to invoke each listed `method` path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_ingress_grpc_authz::MethodAclConfig;
    ///
    /// let acl = MethodAclConfig::deny_all()
    ///     .allow("svc-a", vec!["/pkg.Svc/Greet".to_string()]);
    /// assert!(acl.by_subject["svc-a"].contains(&"/pkg.Svc/Greet".to_string()));
    /// ```
    pub fn allow(
        mut self,
        subject: impl Into<String>,
        methods: impl IntoIterator<Item = String>,
    ) -> Self {
        self.by_subject
            .entry(subject.into())
            .or_default()
            .extend(methods);
        self
    }

    /// Allow EVERY authenticated caller to invoke `method` (wildcard subject).
    ///
    /// Unauthenticated callers (no identity in metadata) are still denied —
    /// the wildcard only expands for callers that have a verified identity.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_ingress_grpc_authz::MethodAclConfig;
    ///
    /// let acl = MethodAclConfig::deny_all()
    ///     .allow_for_any_authenticated("/grpc.health.v1.Health/Check");
    /// assert!(acl.by_subject["*"].contains(&"/grpc.health.v1.Health/Check".to_string()));
    /// ```
    pub fn allow_for_any_authenticated(mut self, method: impl Into<String>) -> Self {
        self.by_subject
            .entry("*".to_string())
            .or_default()
            .push(method.into());
        self
    }

    /// Load from TOML.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_ingress_grpc_authz::MethodAclConfig;
    ///
    /// let toml = r#"
    ///     [by_subject]
    ///     "svc-a" = ["/pkg.Svc/Query"]
    ///     "*" = ["/grpc.health.v1.Health/Check"]
    /// "#;
    /// let acl = MethodAclConfig::from_toml(toml).unwrap();
    /// assert!(acl.by_subject.contains_key("svc-a"));
    /// ```
    pub fn from_toml(s: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(s)
    }

    /// Returns `true` when `subject` is allowed to call `method`.
    pub(crate) fn allows(&self, subject: Option<&str>, method: &str) -> bool {
        // Wildcard subject — applies ONLY to authenticated callers
        // (subject = Some(_)).  An absent subject means upstream
        // authn never populated identity; wildcards MUST NOT
        // silently elevate that case to "allowed".
        if subject.is_some() {
            if let Some(any) = self.by_subject.get("*") {
                if any.iter().any(|m| m == method) {
                    return true;
                }
            }
        }
        let Some(sub) = subject else { return false };
        match self.by_subject.get(sub) {
            None => false,
            Some(methods) if methods.is_empty() => true,
            Some(methods) => methods.iter().any(|m| m == method),
        }
    }
}

impl swe_edge_configbuilder::ConfigSection for MethodAclConfig {
    fn section_name() -> &'static str {
        const NAME: &str = "acl";
        NAME
    }
}
