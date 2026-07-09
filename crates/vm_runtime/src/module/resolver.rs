//! Host module resolver capability boundary.
//!
//! Spec: `PHASE-3-HOST-BOUNDARY-CONTRACT.md` §11, `PHASE-3-MODULE-RUNTIME-CONTRACT.md` §12

use std::collections::BTreeSet;

use vm_core::error::registry::{RuntimeErrorCode, VmStructuralErrorCode};
use vm_core::id::{CapabilityId, ModuleId};

use crate::runtime_error::{RuntimeFailure, RuntimeResult};

/// Granted host capabilities for resolver gating.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CapabilitySet {
    granted: BTreeSet<u32>,
}

impl CapabilitySet {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn grant(&mut self, capability: CapabilityId) {
        self.granted.insert(capability.raw());
    }

    #[must_use]
    pub fn has(&self, capability: CapabilityId) -> bool {
        self.granted.contains(&capability.raw())
    }
}

/// Resolver lookup request (host-defined mapping).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleResolverRequest {
    pub logical_path: String,
}

/// Host-defined module resolver trait.
pub trait HostModuleResolver {
    fn resolve(&self, request: &ModuleResolverRequest) -> RuntimeResult<ModuleId>;
}

/// Capability-gated wrapper for effectful resolvers.
pub struct CapabilityGatedResolver<R> {
    inner: R,
    required_capability: CapabilityId,
}

impl<R> CapabilityGatedResolver<R> {
    #[must_use]
    pub const fn new(inner: R, required_capability: CapabilityId) -> Self {
        Self {
            inner,
            required_capability,
        }
    }

    /// Resolve with capability check using this wrapper's required capability.
    pub fn resolve_gated(
        &self,
        capabilities: &CapabilitySet,
        request: &ModuleResolverRequest,
    ) -> RuntimeResult<ModuleId>
    where
        R: HostModuleResolver,
    {
        resolve_with_capability(
            &self.inner,
            capabilities,
            self.required_capability,
            request,
        )
    }
}

impl<R: HostModuleResolver> HostModuleResolver for CapabilityGatedResolver<R> {
    fn resolve(&self, request: &ModuleResolverRequest) -> RuntimeResult<ModuleId> {
        self.inner.resolve(request)
    }
}

/// Resolve a module id with capability check before host access.
pub fn resolve_with_capability<R: HostModuleResolver + ?Sized>(
    resolver: &R,
    capabilities: &CapabilitySet,
    required_capability: CapabilityId,
    request: &ModuleResolverRequest,
) -> RuntimeResult<ModuleId> {
    if !capabilities.has(required_capability) {
        return Err(RuntimeFailure::language(RuntimeErrorCode::CapabilityError));
    }
    resolver.resolve(request)
}

/// Placeholder resolver for bootstrap tests (deterministic path -> id mapping).
#[derive(Debug, Clone, Default)]
pub struct StubModuleResolver {
    mappings: BTreeSet<(String, u32)>,
}

impl StubModuleResolver {
    pub fn map(&mut self, path: impl Into<String>, module_id: ModuleId) {
        self.mappings.insert((path.into(), module_id.raw()));
    }
}

impl HostModuleResolver for StubModuleResolver {
    fn resolve(&self, request: &ModuleResolverRequest) -> RuntimeResult<ModuleId> {
        for (path, id) in &self.mappings {
            if path == &request.logical_path {
                return Ok(ModuleId::new(*id));
            }
        }
        Err(RuntimeFailure::language(RuntimeErrorCode::ImportError))
    }
}

/// Normalize resolver structural failures to runtime failures.
#[must_use]
pub fn resolver_structural(message: impl Into<String>) -> RuntimeFailure {
    RuntimeFailure::structural(
        VmStructuralErrorCode::InvalidRuntimePlanError,
        message,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolver_without_capability_rejected() {
        let resolver = StubModuleResolver::default();
        let caps = CapabilitySet::new();
        let request = ModuleResolverRequest {
            logical_path: "a.b".to_string(),
        };
        let err = resolve_with_capability(
            &resolver,
            &caps,
            CapabilityId::new(1),
            &request,
        )
        .expect_err("no capability");
        assert!(matches!(
            err,
            RuntimeFailure::Language(RuntimeErrorCode::CapabilityError)
        ));
    }

    #[test]
    fn resolver_with_capability_succeeds() {
        let mut resolver = StubModuleResolver::default();
        resolver.map("a.b", ModuleId::new(7));
        let mut caps = CapabilitySet::new();
        caps.grant(CapabilityId::new(1));
        let request = ModuleResolverRequest {
            logical_path: "a.b".to_string(),
        };
        let id = resolve_with_capability(
            &resolver,
            &caps,
            CapabilityId::new(1),
            &request,
        )
        .expect("resolve");
        assert_eq!(id, ModuleId::new(7));
    }
}