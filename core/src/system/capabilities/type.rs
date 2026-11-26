/// Function pointer signature for platform-host native OS capability actions `(kind, arg1, arg2) -> dispatched`.
pub type NativeActionBridge = fn(&str, &str, &str) -> bool;
