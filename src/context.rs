use leptos::prelude::Owner;
use parking_lot::RwLock;
use std::sync::Arc;
use wasm_bindgen::prelude::wasm_bindgen;

/// Represents a leptos context; i.e. a node somewhere in the reactive graph
#[wasm_bindgen]
#[derive(Clone)]
pub struct LeptosContext {
    inner: Arc<RwLock<Option<Owner>>>,
}
impl LeptosContext {
    pub fn with<R>(&self, f: impl FnOnce() -> R) -> R {
        if let Some(o) = (*self.inner.read()).clone() {
            o.with(f)
        } else {
            tracing::warn!("Leptos context already cleaned up!");
            f()
        }
    }
}

#[wasm_bindgen]
impl LeptosContext {
    /// Cleans up the reactive system.
    /// Not calling this is a memory leak
    pub fn cleanup(&self) -> Result<(), wasm_bindgen::JsError> {
        if let Some(mount) = self.inner.write().take() {
            mount.cleanup(); //flams_web_utils::try_catch(move || mount.cleanup())?;
        }
        Ok(())
    }

    pub fn wasm_clone(&self) -> Self {
        self.clone()
    }
}
impl From<Owner> for LeptosContext {
    #[inline]
    fn from(value: Owner) -> Self {
        Self {
            inner: std::sync::Arc::new(RwLock::new(Some(value))),
        }
    }
}
