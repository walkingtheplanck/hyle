//! Shared result type for viewer-only error reporting.

/// Fallible result used by the viewer when surfacing setup and analysis errors.
pub(crate) type ViewerResult<T> = Result<T, String>;
