use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use audio_input::AudioFrame;
pub use std::rc::Rc;
pub use std::cell::RefCell;

pub type SharedData = Arc<Mutex<Option<AudioFrame>>>;

pub type StateHolder<T> = Rc<RefCell<T>>;

#[derive(Clone)]
pub struct ContinueState {
    inner: Arc<AtomicBool>,
}

impl ContinueState {
    pub fn new(b: bool) -> Self {
        ContinueState {
            inner: Arc::new(AtomicBool::new(b))
        }
    }

    pub fn get(&self) -> bool {
        self.inner.load(Ordering::SeqCst)
    }

    pub fn set(&self, b: bool) {
        self.inner.store(b, Ordering::SeqCst);
    }
}
