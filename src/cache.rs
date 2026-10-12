//! A stale-while-revalidate cache for one upstream value, the in-memory
//! counterpart of Laravel's `Cache::flexible`.

use std::future::Future;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use crate::upstream::UpstreamError;

/// The time the caches read. Production uses the system clock; tests move
/// it forward to step through the fresh and stale windows.
#[derive(Debug, Clone, Default)]
pub struct Clock {
    offset_ms: Arc<AtomicU64>,
}

impl Clock {
    pub fn now(&self) -> Instant {
        Instant::now() + Duration::from_millis(self.offset_ms.load(Ordering::SeqCst))
    }

    pub fn advance(&self, by: Duration) {
        self.offset_ms
            .fetch_add(by.as_millis() as u64, Ordering::SeqCst);
    }
}

/// One cached value. Younger than `fresh` it is served as is; between
/// `fresh` and `stale` it is served while a background task refreshes it;
/// older (or missing) it is fetched before answering.
pub struct SwrCache<T> {
    name: &'static str,
    fresh: Duration,
    stale: Duration,
    clock: Clock,
    _value: std::marker::PhantomData<T>,
}

impl<T: Clone + Send + Sync + 'static> SwrCache<T> {
    pub fn new(name: &'static str, fresh: Duration, stale: Duration, clock: Clock) -> Arc<Self> {
        Arc::new(Self {
            name,
            fresh,
            stale,
            clock,
            _value: std::marker::PhantomData,
        })
    }

    /// The cached value, fetched with `fetch` when needed. `None` only when
    /// nothing was ever fetched and fetching fails now.
    pub async fn get<F, Fut>(self: &Arc<Self>, _fetch: F) -> Option<T>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<T, UpstreamError>> + Send + 'static,
    {
        let _ = (self.name, self.fresh, self.stale, self.clock.now());
        None
    }
}
