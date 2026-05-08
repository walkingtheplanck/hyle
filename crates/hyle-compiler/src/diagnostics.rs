use std::error::Error;
use std::fmt;

/// Diagnostic severity emitted during compilation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    /// Non-fatal issue.
    Warning,
    /// Fatal issue that prevents compilation.
    Error,
}

/// A single compiler diagnostic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    /// Severity for the message.
    pub severity: DiagnosticSeverity,
    /// Optional source path associated with the diagnostic.
    pub path: Option<String>,
    /// Human-readable message.
    pub message: String,
}

impl Diagnostic {
    /// Creates an error diagnostic.
    pub fn error(path: Option<impl Into<String>>, message: impl Into<String>) -> Self {
        Self {
            severity: DiagnosticSeverity::Error,
            path: path.map(Into::into),
            message: message.into(),
        }
    }

    /// Creates a warning diagnostic.
    pub fn warning(path: Option<impl Into<String>>, message: impl Into<String>) -> Self {
        Self {
            severity: DiagnosticSeverity::Warning,
            path: path.map(Into::into),
            message: message.into(),
        }
    }
}

/// A bundle of diagnostics collected during compilation.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DiagnosticReport {
    /// Ordered diagnostics collected so far.
    pub diagnostics: Vec<Diagnostic>,
}

impl DiagnosticReport {
    /// Creates an empty report.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends a diagnostic to the report.
    pub fn push(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Returns true when no diagnostics are present.
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Returns true when the report contains at least one error.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
    }
}

impl fmt::Display for DiagnosticReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, diagnostic) in self.diagnostics.iter().enumerate() {
            if index > 0 {
                writeln!(formatter)?;
            }

            match &diagnostic.path {
                Some(path) => write!(
                    formatter,
                    "{:?}: {}: {}",
                    diagnostic.severity, path, diagnostic.message
                )?,
                None => write!(
                    formatter,
                    "{:?}: {}",
                    diagnostic.severity, diagnostic.message
                )?,
            }
        }

        Ok(())
    }
}

impl Error for DiagnosticReport {}
