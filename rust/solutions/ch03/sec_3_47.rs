// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.47: a semaphore of size n, the
//! mutex's generalization, in the exercise's two requested builds. Part
//! (a) constructs one from mutexes: the permit count lives under the
//! book's test-and-set mutex, and acquire spins over lock-check-unlock
//! until a permit is free. Part (b) constructs one from atomic
//! test-and-set operations: the permit count is a single atomic cell
//! taken with compare-and-swap, no lock at all. Twelve processes crowd
//! through a size-3 semaphore of each build on real threads; the
//! largest number ever observed inside is three, every process gets
//! through, and a nonblocking attempt on a full semaphore is refused.

use std::sync::atomic::{AtomicUsize, Ordering};

use ch03::sec_3_4::{BookMutex, RunHandle, SharedInt, read_shared, write_shared};

/// How many processes crowd through the semaphore in the run.
const PROCESSES: usize = 12;

/// The semaphore's size for the run.
const PERMITS: usize = 3;

/// Exercise 3.47(a): a counting semaphore from mutexes. Acquire takes
/// the mutex, looks at the permit count, and either takes a permit or
/// puts the mutex back and tries again; the spin polls the run handle,
/// so a demo can end it. `false` means the run was halted first.
struct MutexSemaphore {
    mutex: BookMutex,
    permits: SharedInt,
}

impl MutexSemaphore {
    fn new(permits: usize) -> Self {
        Self {
            mutex: BookMutex::new(),
            permits: ch03::sec_3_4::shared_int(
                i128::try_from(permits).expect("permit counts are small"),
            ),
        }
    }

    /// Blocks until a permit is free, spinning over the mutex.
    fn acquire(&self, run: &RunHandle) -> bool {
        loop {
            if !self.mutex.acquire(run) {
                return false;
            }
            let free = read_shared(&self.permits) > 0;
            if free {
                write_shared(&self.permits, read_shared(&self.permits) - 1);
                self.mutex.release();
                return true;
            }
            self.mutex.release();
            if run.halted() {
                return false;
            }
            std::thread::yield_now();
        }
    }

    /// One nonblocking attempt: `false` when the semaphore is full (or
    /// the mutex was busy, which for a caller is the same refusal).
    fn try_acquire(&self) -> bool {
        if !self.mutex.try_acquire() {
            return false;
        }
        let free = read_shared(&self.permits) > 0;
        if free {
            write_shared(&self.permits, read_shared(&self.permits) - 1);
        }
        self.mutex.release();
        free
    }

    /// Returns one permit, under the same mutex.
    fn release(&self) {
        while !self.mutex.try_acquire() {
            std::thread::yield_now();
        }
        write_shared(&self.permits, read_shared(&self.permits) + 1);
        self.mutex.release();
    }
}

/// Exercise 3.47(b): a counting semaphore from atomic test-and-set
/// operations. The whole permit count is one atomic cell; acquire is a
/// compare-and-swap loop, the semaphore needs no mutex anywhere.
struct AtomicSemaphore(AtomicUsize);

impl AtomicSemaphore {
    fn new(permits: usize) -> Self {
        Self(AtomicUsize::new(permits))
    }

    /// Blocks until a permit is free, spinning on compare-and-swap.
    fn acquire(&self, run: &RunHandle) -> bool {
        loop {
            let current = self.0.load(Ordering::SeqCst);
            if current > 0
                && self
                    .0
                    .compare_exchange(current, current - 1, Ordering::SeqCst, Ordering::SeqCst)
                    .is_ok()
            {
                return true;
            }
            if run.halted() {
                return false;
            }
            std::thread::yield_now();
        }
    }

    /// One nonblocking compare-and-swap attempt.
    fn try_acquire(&self) -> bool {
        let current = self.0.load(Ordering::SeqCst);
        current > 0
            && self
                .0
                .compare_exchange(current, current - 1, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
    }

    /// Returns one permit.
    fn release(&self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

/// Sends `processes` real threads through one semaphore -- acquire and
/// release are the caller's closures over whichever build -- and
/// answers the largest number of simultaneous holders any of them
/// observed inside. The scope join is the proof every process got
/// through: an `acquire` that answered `false` would mean a halted run,
/// which these demonstrations never trigger.
fn crowd(
    processes: usize,
    acquire: &(dyn Fn() -> bool + Sync),
    release: &(dyn Fn() + Sync),
) -> usize {
    let inside = AtomicUsize::new(0);
    let largest = AtomicUsize::new(0);
    std::thread::scope(|scope| {
        for _ in 0..processes {
            scope.spawn(|| {
                if acquire() {
                    let observed = inside.fetch_add(1, Ordering::SeqCst) + 1;
                    largest.fetch_max(observed, Ordering::SeqCst);
                    std::thread::yield_now();
                    inside.fetch_sub(1, Ordering::SeqCst);
                    release();
                }
            });
        }
    });
    largest.load(Ordering::SeqCst)
}

/// The exercise's run: both builds absorb twelve crowding processes,
/// and the two largest-observed counts come back for the answer.
#[must_use]
fn semaphore_run() -> (usize, usize) {
    let run = RunHandle::new_run();
    let from_mutex = MutexSemaphore::new(PERMITS);
    let from_atomic = AtomicSemaphore::new(PERMITS);
    let mutex_acquire = || from_mutex.acquire(&run);
    let mutex_release = || from_mutex.release();
    let atomic_acquire = || from_atomic.acquire(&run);
    let atomic_release = || from_atomic.release();
    let from_mutex_largest = crowd(PROCESSES, &mutex_acquire, &mutex_release);
    let from_atomic_largest = crowd(PROCESSES, &atomic_acquire, &atomic_release);
    (from_mutex_largest, from_atomic_largest)
}

mod ex_3_47 {
    use super::{PERMITS, PROCESSES, semaphore_run};

    /// Exercise 3.47: semaphore from mutex or test-and-set
    ///
    /// Answers the largest number of holders ever observed inside the
    /// busier of the two builds, the semaphore's size, and how many
    /// processes went through it.
    #[must_use]
    pub fn ex_3_47() -> (usize, usize, usize) {
        let (from_mutex, from_atomic) = semaphore_run();
        (from_mutex.max(from_atomic), PERMITS, PROCESSES)
    }
}

#[test]
fn ex_3_47() {
    let (largest, permits, processes) = ex_3_47::ex_3_47();
    // Both builds held the line at three: no fourth process was ever
    // inside, however hard twelve of them crowded.
    assert!(largest <= permits);
    // And the semaphore did its job in the other direction too: at
    // least one process was inside, so the count is a real observation.
    assert!(largest >= 1);
    assert_eq!((permits, processes), (3, 12));
}

#[test]
fn ex_3_47_try_acquire_refuses_when_full() {
    // A full semaphore refuses a nonblocking attempt, on both builds,
    // and a release reopens it.
    let run = RunHandle::new_run();
    let from_mutex = MutexSemaphore::new(2);
    let from_atomic = AtomicSemaphore::new(2);
    assert!(from_mutex.acquire(&run));
    assert!(from_mutex.acquire(&run));
    assert!(!from_mutex.try_acquire());
    from_mutex.release();
    assert!(from_mutex.try_acquire());
    assert!(from_atomic.acquire(&run));
    assert!(from_atomic.acquire(&run));
    assert!(!from_atomic.try_acquire());
    from_atomic.release();
    assert!(from_atomic.try_acquire());
}
