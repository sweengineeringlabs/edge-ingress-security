# Changelog

All notable changes to `swe-edge-ingress-grpc-authz` are documented here.

## [Unreleased]

### Added
- Initial `AuthzInterceptor` with pluggable `AuthzPolicy`.
- Built-in `MethodAclPolicy` backed by TOML-loadable `MethodAclConfig`.
