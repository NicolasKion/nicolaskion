//! A stale-while-revalidate cache for one upstream value, in memory. A
//! failed fetch never throws the last good value away.

use std::future::Future;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
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
        let millis = u64::try_from(by.as_millis()).unwrap_or(u64::MAX);
        self.offset_ms.fetch_add(millis, Ordering::SeqCst);
    }
}

struct Entry<T> {
    value: T,
    stored: Instant,
}

/// One cached value. Younger than `fresh` it is served as is; between
/// `fresh` and `stale` it is served while a background task refreshes it;
/// older (or missing) it is fetched before answering. A failed fetch
/// leaves the last good value in place and serves it.
pub struct SwrCache<T> {
    name: &'static str,
    fresh: Duration,
    stale: Duration,
    clock: Clock,
    entry: RwLock<Option<Entry<T>>>,
    /// Held while a fetch runs, so visitors share one fetch.
    fetching: Arc<tokio::sync::Mutex<()>>,
}

impl<T: Clone + Send + Sync + 'static> SwrCache<T> {
    pub fn new(name: &'static str, fresh: Duration, stale: Duration, clock: Clock) -> Arc<Self> {
        Arc::new(Self {
            name,
            fresh,
            stale,
            clock,
            entry: RwLock::new(None),
            fetching: Arc::new(tokio::sync::Mutex::new(())),
        })
    }

    /// The cached value, fetched with `fetch` when needed. `None` only when
    /// nothing was ever fetched and fetching fails now.
    pub async fn get<F, Fut>(self: &Arc<Self>, fetch: F) -> Option<T>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<T, UpstreamError>> + Send + 'static,
    {
        if let Some((value, age)) = self.current() {
            if age < self.fresh {
                return Some(value);
            }
            if age < self.stale {
                self.refresh_in_background(fetch);
                return Some(value);
            }
        }

        let _fetching = self.fetching.lock().await;
        // Another visitor may have fetched it while this one waited.
        if let Some((value, age)) = self.current()
            && age < self.stale
        {
            return Some(value);
        }
        match fetch().await {
            Ok(value) => {
                self.store(value.clone());
                Some(value)
            }
            Err(error) => {
                let last = self.current().map(|(value, _)| value);
                tracing::warn!(
                    cache = self.name,
                    %error,
                    serving_last_value = last.is_some(),
                    "upstream fetch failed"
                );
                last
            }
        }
    }

    /// The value and its age.
    fn current(&self) -> Option<(T, Duration)> {
        let entry = self.entry.read().expect("cache lock");
        entry.as_ref().map(|entry| {
            let age = self.clock.now().saturating_duration_since(entry.stored);
            (entry.value.clone(), age)
        })
    }

    fn store(&self, value: T) {
        *self.entry.write().expect("cache lock") = Some(Entry {
            value,
            stored: self.clock.now(),
        });
    }

    /// Fetches in a background task unless a fetch is running already.
    fn refresh_in_background<F, Fut>(self: &Arc<Self>, fetch: F)
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<T, UpstreamError>> + Send + 'static,
    {
        let Ok(fetching) = self.fetching.clone().try_lock_owned() else {
            return;
        };
        let cache = Arc::clone(self);
        tokio::spawn(async move {
            let _fetching = fetching;
            match fetch().await {
                Ok(value) => cache.store(value),
                Err(error) => {
                    tracing::warn!(cache = cache.name, %error, "background refresh failed; serving the stale value");
                }
            }
        });
    }
}
