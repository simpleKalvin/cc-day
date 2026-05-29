use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Tauri 状态管理的 newtype wrapper。
/// show 前置 true，首次 Focused(true) 后置 false，期间忽略 Focused(false)。
pub struct IsShowingFlag(pub Arc<AtomicBool>);

impl IsShowingFlag {
    pub fn will_show(&self) {
        self.0.store(true, Ordering::SeqCst);
    }
}
