//! Binding table and scope stack.
//!
//! Spec: `PHASE-1-LANGUAGE-SPEC.md` §2.1–2.2, §3.3 NFC comparison

use script_lex::Span;
use unicode_normalization::UnicodeNormalization;

/// How a name may be used after introduction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingKind {
    /// `let` — mutable binding.
    Mutable,
    /// `const` or `def` — immutable binding (cannot reassign the name).
    Immutable,
    /// Prelude / host-injected name (treated as immutable for assignment).
    Builtin,
}

/// One name binding in a scope (name stored NFC-normalized).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binding {
    /// NFC-normalized identifier text (SPEC-P1 §3.3).
    pub name: String,
    pub kind: BindingKind,
    pub decl_span: Span,
    /// True if introduced under `export` (module public interface).
    pub exported: bool,
}

impl Binding {
    #[must_use]
    pub fn new(name: impl AsRef<str>, kind: BindingKind, decl_span: Span, exported: bool) -> Self {
        Self {
            name: nfc(name.as_ref()),
            kind,
            decl_span,
            exported,
        }
    }
}

/// Lexical scope: map of NFC names → binding.
#[derive(Debug, Clone, Default)]
pub struct Scope {
    pub bindings: Vec<Binding>,
}

impl Scope {
    #[must_use]
    pub fn new() -> Self {
        Self {
            bindings: Vec::new(),
        }
    }

    pub fn define(&mut self, binding: Binding) -> Result<(), String> {
        if self.bindings.iter().any(|b| b.name == binding.name) {
            return Err(format!(
                "duplicate binding `{}` in the same scope (NFC)",
                binding.name
            ));
        }
        self.bindings.push(binding);
        Ok(())
    }

    #[must_use]
    pub fn lookup_local(&self, name: &str) -> Option<&Binding> {
        let key = nfc(name);
        self.bindings.iter().rev().find(|b| b.name == key)
    }
}

/// Stack of scopes; top is innermost.
#[derive(Debug, Clone)]
pub struct ScopeStack {
    scopes: Vec<Scope>,
}

impl ScopeStack {
    #[must_use]
    pub fn new_module() -> Self {
        Self {
            scopes: vec![Scope::new()],
        }
    }

    pub fn push(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub fn pop(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn define(&mut self, binding: Binding) -> Result<(), String> {
        self.scopes
            .last_mut()
            .expect("scope stack non-empty")
            .define(binding)
    }

    #[must_use]
    pub fn resolve(&self, name: &str) -> Option<&Binding> {
        let key = nfc(name);
        for scope in self.scopes.iter().rev() {
            if let Some(b) = scope.lookup_local(&key) {
                return Some(b);
            }
        }
        None
    }

    #[must_use]
    pub fn depth(&self) -> usize {
        self.scopes.len()
    }
}

/// Normalize identifier spelling for comparison (SPEC-P1 §3.3).
#[must_use]
pub fn nfc(s: &str) -> String {
    s.nfc().collect()
}
