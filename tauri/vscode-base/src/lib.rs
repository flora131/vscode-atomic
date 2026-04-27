//! vscode-base — port of src/vs/base/ primitives.

pub mod disposable;
pub mod event;
pub mod errors;
pub mod observable;
pub mod uri;
pub mod cancellation;

pub use errors::VsCodeError;
pub use disposable::{Disposable, DisposableStore, DisposableHandle};
pub use event::{Emitter, EventEmitter, ListenerHandle};
pub use observable::{IObserver, Observable, Derived, CallbackObserver};
pub use uri::{VsUri, UriError};
pub use cancellation::{VsCancellationToken, CancellationTokenSource};

#[cfg(test)]
mod tests {
    use super::event::EventEmitter;

    #[test]
    fn event_emitter_fires_to_listeners() {
        let emitter: EventEmitter<i32> = EventEmitter::new();
        let results = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
        let r = results.clone();
        emitter.on(move |v| r.lock().unwrap().push(*v));
        emitter.fire(&42);
        emitter.fire(&7);
        assert_eq!(*results.lock().unwrap(), vec![42, 7]);
    }

    #[test]
    fn event_emitter_multiple_listeners() {
        let emitter: EventEmitter<u8> = EventEmitter::new();
        let a = std::sync::Arc::new(std::sync::Mutex::new(0u8));
        let b = std::sync::Arc::new(std::sync::Mutex::new(0u8));
        let a2 = a.clone();
        let b2 = b.clone();
        emitter.on(move |v| *a2.lock().unwrap() = *v);
        emitter.on(move |v| *b2.lock().unwrap() = *v + 10);
        emitter.fire(&5);
        assert_eq!(*a.lock().unwrap(), 5);
        assert_eq!(*b.lock().unwrap(), 15);
    }
}
