/// A UI operation result preserving a contextual, human-readable error.
pub type AutomationResult<T> = Result<T, String>;
