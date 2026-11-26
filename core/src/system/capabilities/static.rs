use super::*;

/// Optional process-wide native OS capability bridge registered by Android/iOS/Desktop hosts.
pub(crate) static EXTERNAL_NATIVE_ACTION_BRIDGE: AtomicPtr<()> = AtomicPtr::new(null_mut());
