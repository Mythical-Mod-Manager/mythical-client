use pin_project::pin_project;
use std::{collections::HashSet, fmt::Debug, future::Future, hash::Hash, pin::Pin, sync::RwLock};

/// A structure that helps asynchronously lock named resources, for example directories or paths.
/// This is especially useful for locking folders and files that may be used by
/// multiple parallel operations. The order of operations is not guaranteed.
pub struct TaskGuard<T> {
    locks: RwLock<HashSet<T>>,
}

pub struct TaskGuardLock<'a, T: Eq + Hash> {
    key: T,
    guard: &'a TaskGuard<T>,
}

#[pin_project]
pub struct GuardWaiter<'a, T> {
    guard: &'a TaskGuard<T>,
    key: T,
}

impl<'a, T> GuardWaiter<'a, T> {
    pub fn new(guard: &'a TaskGuard<T>, key: T) -> Self {
        GuardWaiter { guard, key }
    }
}

impl<'a, T: Eq + Hash + Clone + Debug> Future for GuardWaiter<'a, T> {
    type Output = TaskGuardLock<'a, T>;

    fn poll(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();
        let guard = this.guard;
        let key = this.key;

        let read = guard.locks.read().unwrap();
        if read.contains(&key) {
            return std::task::Poll::Pending;
        }
        drop(read);

        let mut write = guard.locks.write().unwrap();
        if write.contains(&key) {
            return std::task::Poll::Pending;
        }
        write.insert(key.clone());

        std::task::Poll::Ready(TaskGuardLock {
            key: key.clone(),
            guard,
        })
    }
}

impl<T: Eq + Hash + Clone> TaskGuard<T> {
    pub fn new() -> Self {
        TaskGuard {
            locks: RwLock::new(HashSet::new()),
        }
    }

    pub fn lock<'a>(&'a self, key: T) -> GuardWaiter<'a, T> {
        GuardWaiter::new(self, key)
    }
}

impl<T: Eq + Hash> Drop for TaskGuardLock<'_, T> {
    fn drop(&mut self) {
        let mut write = self.guard.locks.write().unwrap();
        write.remove(&self.key);
    }
}
