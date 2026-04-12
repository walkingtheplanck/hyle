//! Shared diagnostic types used by analysis reports.

use std::fmt::{Display, Formatter};

/// Severity level for an analysis diagnostic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    /// Informational output that does not indicate a problem.
    Info,
    /// A potential problem or questionable construct.
    Warning,
    /// A definite problem in the analyzed construct.
    Error,
}

impl Display for Severity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Info => f.write_str("info"),
            Severity::Warning => f.write_str("warning"),
            Severity::Error => f.write_str("error"),
        }
    }
}

/// The primary subject referenced by a diagnostic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Subject {
    /// The diagnostic refers to the automaton spec as a whole.
    Spec,
    /// The diagnostic refers to a specific rule.
    Rule {
        /// Zero-based rule index.
        index: usize,
    },
    /// The diagnostic refers to a specific named neighborhood.
    Neighborhood {
        /// Zero-based neighborhood index.
        index: usize,
    },
    /// The diagnostic refers to topology configuration.
    Topology,
}

impl Display for Subject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Subject::Spec => f.write_str("spec"),
            Subject::Rule { index } => write!(f, "rule {index}"),
            Subject::Neighborhood { index } => write!(f, "neighborhood {index}"),
            Subject::Topology => f.write_str("topology"),
        }
    }
}

/// A single analysis diagnostic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    /// Severity level.
    pub severity: Severity,
    /// Stable machine-readable code.
    pub code: &'static str,
    /// Human-readable message.
    pub message: String,
    /// Primary subject associated with the message.
    pub subject: Subject,
}

impl Diagnostic {
    /// Construct a new diagnostic.
    pub fn new(
        severity: Severity,
        code: &'static str,
        message: impl Into<String>,
        subject: Subject,
    ) -> Self {
        Self {
            severity,
            code,
            message: message.into(),
            subject,
        }
    }

    /// Construct an informational diagnostic.
    pub fn info(code: &'static str, message: impl Into<String>, subject: Subject) -> Self {
        Self::new(Severity::Info, code, message, subject)
    }

    /// Construct a warning diagnostic.
    pub fn warning(code: &'static str, message: impl Into<String>, subject: Subject) -> Self {
        Self::new(Severity::Warning, code, message, subject)
    }

    /// Construct an error diagnostic.
    pub fn error(code: &'static str, message: impl Into<String>, subject: Subject) -> Self {
        Self::new(Severity::Error, code, message, subject)
    }
}

impl Display for Diagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} ({}): {}",
            self.severity, self.code, self.subject, self.message
        )
    }
}
