use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

static GLOBAL_THREAD_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

pub type Id = usize;

pub(crate) fn next() -> usize {
	GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst)
}