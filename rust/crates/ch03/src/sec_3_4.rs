// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 3.4

//! Section 3.4: Concurrency: time is of the essence.
//!
//! The section keeps its lesson set (interleavings, serializers, deadlock)
//! and is taught with Rust's own mechanisms. Ownership is the first
//! serializer: the compiler refuses the data race the book's first example
//! races into, so every shared variable of this section lives behind
//! [`Arc`](std::sync::Arc) and [`Mutex`](std::sync::Mutex) -- a deliberate,
//! stated migration from the `Rc<RefCell<..>>` of section 3.3, which is not
//! [`Send`](std::marker::Send) and therefore cannot cross a thread.
//!
//! The book's `parallel-execute` becomes [`parallel_execute`]: a
//! [`std::thread::scope`] over two spawned closures that returns both
//! results. The control object that the book's footnote attaches to the
//! primitive is the returned [`RunHandle`]: its `halt` sets one shared
//! [`AtomicBool`] and every procedure polls `halted` between its own steps,
//! so a demonstration that deadlocks or loops can be asked to stop instead
//! of hanging the run.
//!
//! Every interleaving demo answers one value from a possible-outcome set;
//! the callers assert membership in the set, never one specific schedule.

use std::cell::Cell;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, PoisonError, TryLockError};

use crate::sec_3_1::Reply;

/// Reads a mutex-guarded value even if a panicked process left the mutex
/// poisoned: the guarded integers here are plain numbers with no invalid
/// state, so the value after a panic is still a number.
fn lock_shared<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

/// The two procedures of a run see the same handle.
pub struct RunHandle {
    halted: AtomicBool,
}

impl RunHandle {
    fn new() -> Self {
        Self {
            halted: AtomicBool::new(false),
        }
    }

    /// A fresh handle for a run assembled by hand: a demonstration that
    /// needs three processes at once (exercise 3.38) spawns its own scope
    /// and shares this handle.
    #[must_use]
    pub fn new_run() -> Self {
        Self::new()
    }

    /// Asks every procedure of the run to stop: sets the one shared flag
    /// that the book's footnote summarizes as what the control object can
    /// do to the processes it started.
    pub fn halt(&self) {
        self.halted.store(true, Ordering::Release);
    }

    /// The flag the workers poll between their own steps.
    #[must_use]
    pub fn halted(&self) -> bool {
        self.halted.load(Ordering::Acquire)
    }
}

/// Runs two procedures concurrently and returns both results with the
/// run's handle: the book's two-process `parallel-execute`. The first
/// procedure runs on a freshly spawned thread, the second on the calling
/// thread, and the call returns only after both have finished, so the two
/// procedures are genuinely in flight at the same time. Each procedure
/// receives the same [`RunHandle`], whose `halt` it observes through
/// `halted`; the demonstrations in this section let every procedure run to
/// completion and use the flag to end a run that would otherwise wait
/// forever.
///
/// # Panics
///
/// Panics if either process panics; the book's processes have no error
/// channel, and a crashed process is a crashed run.
pub fn parallel_execute<PA, PB, RA, RB>(pa: PA, pb: PB) -> (RA, RB, RunHandle)
where
    PA: FnOnce(&RunHandle) -> RA + Send,
    RA: Send,
    PB: FnOnce(&RunHandle) -> RB + Send,
    RB: Send,
{
    let handle = RunHandle::new();
    let (left, right) = std::thread::scope(|scope| {
        let first = scope.spawn(|| pa(&handle));
        let right = pb(&handle);
        let left = first.join().expect("parallel_execute: process panicked");
        (left, right)
    });
    (left, right, handle)
}

/// A serializer: the book's `make-serializer` value. All calls to one
/// serializer protect procedures in the same set, so protected bodies
/// cannot interleave with one another.
#[derive(Clone, Default)]
pub struct Serializer {
    mutex: Arc<Mutex<()>>,
}

impl Serializer {
    /// The book's `make-serializer`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            mutex: Arc::new(Mutex::new(())),
        }
    }

    /// Runs `p` under the serializer's mutex: acquires, runs, releases.
    /// This is the working serializer the section's accounts use; the
    /// section also builds its own serializer from a hand-rolled mutex,
    /// [`BookSerializer`], with the same contract.
    pub fn protect<T>(&self, p: impl FnOnce() -> T) -> T {
        let _guard = lock_shared(&self.mutex);
        p()
    }

    /// The probe the deadlock demonstrations need: protects `p` only if
    /// the mutex is free at the moment of the call, and answers `None`
    /// when some other process holds it -- or when a panicked process
    /// poisoned it, which for this purpose is held. A `protect` call
    /// would wait forever in exactly that situation.
    pub fn try_protect<T>(&self, p: impl FnOnce() -> T) -> Option<T> {
        match self.mutex.try_lock() {
            Ok(_guard) => Some(p()),
            Err(TryLockError::WouldBlock | TryLockError::Poisoned(_)) => None,
        }
    }
}

/// A shared integer: the book's variable that more than one process can
/// reach. Section 3.3 kept such state in `Rc<RefCell<..>>`; neither type
/// is `Send`, so a process -- a thread -- cannot take it along. `Arc`
/// shares the ownership across threads and `Mutex` makes each access
/// atomic.
pub type SharedInt = Arc<Mutex<i128>>;

/// Makes a shared integer holding `value`: the book's `(define x 10)`
/// written so a second process can reach it.
#[must_use]
pub fn shared_int(value: i128) -> SharedInt {
    Arc::new(Mutex::new(value))
}

/// Reads a shared integer: one locked access.
#[must_use]
pub fn read_shared(cell: &SharedInt) -> i128 {
    *lock_shared(cell)
}

/// Writes a shared integer: one locked access.
pub fn write_shared(cell: &SharedInt, value: i128) {
    *lock_shared(cell) = value;
}

/// The shared `x` of the section's first example, starting at 10.
#[must_use]
pub fn x_race_cell() -> SharedInt {
    shared_int(10)
}

/// The first process of the example: sets `x` to `x` times `x`, its two
/// accesses of `x` two separate locked reads, so the interleavings the
/// book enumerates stay possible.
pub fn square_x(x: &SharedInt) {
    let first = read_shared(x);
    let second = read_shared(x);
    write_shared(x, first * second);
}

/// The second process of the example: sets `x` to `x` plus 1.
pub fn increment_x(x: &SharedInt) {
    let value = read_shared(x);
    write_shared(x, value + 1);
}

/// The second process of exercise 3.40: sets `x` to `x` times `x` times
/// `x`, with three separate locked reads.
pub fn cube_x(x: &SharedInt) {
    let first = read_shared(x);
    let second = read_shared(x);
    let third = read_shared(x);
    write_shared(x, first * second * third);
}

/// The five values the book enumerates for the unserialized race.
pub const X_RACE_VALUES: [i128; 5] = [101, 121, 110, 11, 100];

/// The two values left when both processes are serialized.
pub const X_RACE_SERIALIZED_VALUES: [i128; 2] = [101, 121];

/// Runs the unserialized race once and answers whatever this particular
/// schedule left in `x`: one of [`X_RACE_VALUES`].
#[must_use]
pub fn run_x_race() -> i128 {
    let x = x_race_cell();
    let x2 = Arc::clone(&x);
    let _ = parallel_execute(|_run| square_x(&x), |_run| increment_x(&x2));
    read_shared(&x)
}

/// Runs the serialized race once and answers whatever this particular
/// schedule left in `x`: one of [`X_RACE_SERIALIZED_VALUES`].
#[must_use]
pub fn run_x_race_serialized() -> i128 {
    let x = x_race_cell();
    let x2 = Arc::clone(&x);
    let s = Serializer::new();
    let s2 = s.clone();
    let _ = parallel_execute(
        |_run| s.protect(|| square_x(&x)),
        |_run| s2.protect(|| increment_x(&x2)),
    );
    read_shared(&x)
}

/// The withdrawal of section 3.1.1 restated over shared state. The check
/// and the assignment are two separate locked accesses, so a concurrent
/// withdrawal can invalidate the check between them: the timing diagram
/// of Figure 3.29 is constructible, which is the section's subject.
pub fn shared_withdraw(balance: &SharedInt, amount: i128) -> Reply {
    let current = read_shared(balance);
    if current >= amount {
        let new_balance = current - amount;
        write_shared(balance, new_balance);
        Reply::Balance(new_balance)
    } else {
        Reply::Message("Insufficient funds")
    }
}

/// The bank account of section 3.1.1 with serialized deposits and
/// withdrawals. The 3.3 account held its balance in `Rc<Cell<i128>>`,
/// which is not `Send`; this account states the migration once: the
/// balance moves into `Arc<Mutex<..>>`, and one [`Serializer`] guards
/// every deposit and withdrawal.
#[derive(Clone)]
pub struct Account {
    balance: SharedInt,
    protected: Serializer,
}

impl Account {
    /// The book's `make-account`, answering the same replies as the
    /// account of section 3.1.1.
    #[must_use]
    pub fn new(initial: i128) -> Self {
        Self {
            balance: shared_int(initial),
            protected: Serializer::new(),
        }
    }

    /// The book's `withdraw` message, protected by the account's
    /// serializer: two processes can never be withdrawing from or
    /// depositing into one account concurrently.
    #[must_use]
    pub fn withdraw(&self, amount: i128) -> Reply {
        self.protected
            .protect(|| shared_withdraw(&self.balance, amount))
    }

    /// The book's `deposit` message, protected the same way.
    #[must_use]
    pub fn deposit(&self, amount: i128) -> Reply {
        self.protected.protect(|| {
            let new_balance = read_shared(&self.balance) + amount;
            write_shared(&self.balance, new_balance);
            Reply::Balance(new_balance)
        })
    }

    /// The book's `balance` message, answering without the serializer,
    /// exactly as the main text's account does; exercise 3.41 asks
    /// whether Ben's serialized variant behaves differently.
    #[must_use]
    pub fn balance(&self) -> i128 {
        read_shared(&self.balance)
    }
}

/// Deposits one unit into `account`, `count` times, half of the deposits
/// on a spawned thread and half on the calling thread; the serialized
/// deposits conserve every unit.
#[must_use]
pub fn concurrent_deposits(account: &Account, count: usize) -> i128 {
    let half = count / 2;
    let account2 = account.clone();
    let _ = parallel_execute(
        |_run| {
            for _ in 0..half {
                // The book's deposit loop ignores the reply.
                let _ = account.deposit(1);
            }
        },
        |_run| {
            for _ in 0..(count - half) {
                let _ = account2.deposit(1);
            }
        },
    );
    account.balance()
}

/// The account of the text that exports its serializer: withdrawals and
/// deposits answer raw, and serializing them is each user's job. The
/// serializer is one value shared between the account and every handle
/// handed out, so protecting through it really does lock the account.
pub struct AccountAndSerializer {
    balance: SharedInt,
    serializer: Serializer,
}

impl AccountAndSerializer {
    /// The book's `make-account-and-serializer`.
    #[must_use]
    pub fn new(initial: i128) -> Self {
        Self {
            balance: shared_int(initial),
            serializer: Serializer::new(),
        }
    }

    /// The raw balance read; the book's `'balance` message.
    #[must_use]
    pub fn balance(&self) -> i128 {
        read_shared(&self.balance)
    }

    /// The raw, unprotected withdrawal the book's dispatch returns.
    #[must_use]
    pub fn withdraw(&self, amount: i128) -> Reply {
        shared_withdraw(&self.balance, amount)
    }

    /// The raw, unprotected deposit; it accepts negative amounts, as the
    /// book's footnote about `exchange` says out loud.
    #[must_use]
    pub fn deposit(&self, amount: i128) -> Reply {
        let new_balance = read_shared(&self.balance) + amount;
        write_shared(&self.balance, new_balance);
        Reply::Balance(new_balance)
    }

    /// The book's `'serializer` message: a handle on the account's own
    /// serializer, sharing its mutex with every other handle.
    #[must_use]
    pub fn serializer(&self) -> Serializer {
        self.serializer.clone()
    }
}

/// Deposits through the exported serializer: the book's protected
/// `deposit` of this account version, and the user-managed serialization
/// the text demonstrates. The new balance is read back after the
/// protected deposit completes.
#[must_use]
pub fn deposit_protected(account: &AccountAndSerializer, amount: i128) -> i128 {
    let serializer = account.serializer();
    serializer.protect(|| account.deposit(amount));
    account.balance()
}

/// Swaps the balances of two accounts: reads both, computes the
/// difference, withdraws it from one and deposits it into the other. The
/// reads and writes are separate operations, so a concurrent `exchange`
/// on a shared account can interleave between them; that is the section's
/// road to the violation of exercise 3.43 and to deadlock.
#[must_use]
pub fn exchange(account1: &AccountAndSerializer, account2: &AccountAndSerializer) -> i128 {
    let difference = account1.balance() - account2.balance();
    // The book's exchange discards both replies; the balances themselves
    // are the answer it reports.
    let _ = account1.withdraw(difference);
    let _ = account2.deposit(difference);
    difference
}

/// Serializes the whole exchange with both accounts' serializers: the fix
/// for the interleaving [`exchange`] allows, and -- when two processes
/// exchange the same two accounts in opposite orders -- the road to the
/// deadlock the text describes.
#[must_use]
pub fn serialized_exchange(
    account1: &AccountAndSerializer,
    account2: &AccountAndSerializer,
) -> i128 {
    let s1 = account1.serializer();
    let s2 = account2.serializer();
    s1.protect(|| s2.protect(|| exchange(account1, account2)))
}

/// The book's `test-and-set!` as an ordinary procedure over a plain
/// boolean cell: it reads the cell and then, if the cell was false, writes
/// it. Reading and writing are two separate memory operations, which is
/// exactly the window exercise 3.46 demonstrates.
pub fn test_and_set(cell: &Cell<bool>) -> bool {
    if cell.get() {
        true
    } else {
        cell.set(true);
        false
    }
}

/// The cell of the book's mutex, an `AtomicBool` so the test and the set
/// are one indivisible instruction: the hardware test-and-set the text
/// says multiprocessing computers provide, spelled `compare_exchange`.
#[derive(Default)]
pub struct TestAndSetCell(AtomicBool);

impl TestAndSetCell {
    /// A free cell.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The atomic `test-and-set!`: answers whether the cell was already
    /// set, setting it on the way when it was not, with no window between
    /// the test and the set for a second process to slip through.
    pub fn test_and_set(&self) -> bool {
        self.0
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
    }

    /// The book's `clear!`: frees the cell.
    pub fn clear(&self) {
        self.0.store(false, Ordering::Release);
    }
}

/// The book's mutex: a cell plus acquire and release, acquire retrying in
/// a loop while the cell reads set. The retry loop polls the run handle's
/// flag between attempts, so a demonstration whose acquire can never
/// succeed -- the deadlocks of exercises 3.45 and 3.49 -- can be asked to
/// give up instead of spinning forever; giving up answers `false` and
/// leaves the mutex as it found it.
#[derive(Default)]
pub struct BookMutex {
    cell: TestAndSetCell,
}

impl BookMutex {
    /// A free mutex.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Acquires the mutex, retrying while it is held; `false` when the
    /// run was halted first.
    pub fn acquire(&self, run: &RunHandle) -> bool {
        while self.cell.test_and_set() {
            if run.halted() {
                return false;
            }
            std::thread::yield_now();
        }
        true
    }

    /// A single non-blocking attempt: answers whether the mutex was free
    /// and is now held by the caller.
    #[must_use]
    pub fn try_acquire(&self) -> bool {
        !self.cell.test_and_set()
    }

    /// Releases the mutex.
    pub fn release(&self) {
        self.cell.clear();
    }
}

/// The serializer built the way the text builds it: one hand-rolled mutex
/// per serializer, protect acquires, runs, and releases. The working
/// [`Serializer`] wraps the standard mutex, which blocks instead of
/// spinning, and is what the rest of the section uses; this one is the
/// section's own construction made runnable.
#[derive(Clone, Default)]
pub struct BookSerializer {
    mutex: Arc<BookMutex>,
}

impl BookSerializer {
    /// The book's `make-serializer` over the book's mutex.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Protects `p`: acquires the mutex, runs `p`, releases. Answers
    /// `None` without running `p` when the run was halted while waiting.
    pub fn protect<T>(&self, run: &RunHandle, p: impl FnOnce() -> T) -> Option<T> {
        if !self.mutex.acquire(run) {
            return None;
        }
        let value = p();
        self.mutex.release();
        Some(value)
    }
}

/// A counting semaphore of size `n`: exercise 3.47's generalization of
/// the mutex, in its blocking form. Up to `n` processes hold it at once;
/// the rest wait for a release, on the condition variable rather than in
/// a spin.
pub struct Semaphore {
    permits: Mutex<usize>,
    available: Condvar,
}

impl Semaphore {
    /// A semaphore holding `permits` permits.
    #[must_use]
    pub fn new(permits: usize) -> Self {
        Self {
            permits: Mutex::new(permits),
            available: Condvar::new(),
        }
    }

    /// Acquires a permit, waiting on the condition variable -- with the
    /// permit count as the predicate, rechecked in a loop -- until one is
    /// released.
    pub fn acquire(&self) {
        let mut permits = lock_shared(&self.permits);
        while *permits == 0 {
            permits = self
                .available
                .wait(permits)
                .unwrap_or_else(PoisonError::into_inner);
        }
        *permits -= 1;
    }

    /// A single non-blocking attempt: answers whether a permit was free
    /// and is now taken.
    #[must_use]
    pub fn try_acquire(&self) -> bool {
        let mut permits = lock_shared(&self.permits);
        if *permits == 0 {
            false
        } else {
            *permits -= 1;
            true
        }
    }

    /// Releases a permit and wakes one waiter.
    pub fn release(&self) {
        {
            let mut permits = lock_shared(&self.permits);
            *permits += 1;
        }
        self.available.notify_one();
    }
}

/// The spin budget of one handshake wait: after this many misses the
/// waiter panics instead of hanging a test forever. A correct driver
/// opens the gate long before the budget runs out; the budget only
/// converts a harness bug into a named failure.
const HANDSHAKE_SPIN_BUDGET: u32 = 4_000_000;

/// One handshake slot between the forced-interleaving driver and one
/// process, ticketed by step number: the driver publishes how many steps
/// the process may run, the process runs a step when its ticket covers
/// one and reports the number it completed. A process can never run two
/// steps on one ticket, and the driver observes every step in schedule
/// order. This is the mechanism of exercise 3.38a's harness, shared by
/// the forced interleavings of exercises 3.39 to 3.49.
#[derive(Default)]
pub struct Handshake {
    /// The number of steps the driver has let through.
    ticket: AtomicUsize,
    /// The number of steps the process has completed.
    completed: AtomicUsize,
}

impl Handshake {
    /// A slot with no steps let through.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Process side: waits until the driver's ticket covers step
    /// `completed + 1`, runs `step`, and reports it. `false` when the
    /// run was halted while waiting.
    ///
    /// # Panics
    ///
    /// Panics when the ticket never advances within the spin budget,
    /// which can only mean the driver's schedule ended too early.
    pub fn take_step(&self, run: &RunHandle, step: impl FnOnce()) -> bool {
        let mine = self.completed.load(Ordering::Acquire) + 1;
        let mut misses = 0u32;
        while misses <= HANDSHAKE_SPIN_BUDGET && self.ticket.load(Ordering::Acquire) < mine {
            if run.halted() {
                return false;
            }
            misses += 1;
            std::thread::yield_now();
        }
        assert!(
            misses <= HANDSHAKE_SPIN_BUDGET,
            "step {mine} never scheduled: the driver's schedule ended too early"
        );
        step();
        self.completed.store(mine, Ordering::Release);
        true
    }

    /// Driver side: raises the ticket by one and returns once the step
    /// behind that ticket has run and been reported.
    ///
    /// # Panics
    ///
    /// Panics when the step never runs within the spin budget.
    pub fn drive_step(&self, run: &RunHandle) {
        let next = self.ticket.load(Ordering::Acquire) + 1;
        self.ticket.store(next, Ordering::Release);
        let mut misses = 0u32;
        while misses <= HANDSHAKE_SPIN_BUDGET && self.completed.load(Ordering::Acquire) < next {
            if run.halted() {
                return;
            }
            misses += 1;
            std::thread::yield_now();
        }
        assert!(
            misses <= HANDSHAKE_SPIN_BUDGET,
            "scheduled step {next} never ran: the process is stuck"
        );
    }
}

/// Runs `procs` on real threads and forces the interleaving `schedule`:
/// the steps of `procs[p]` run in order on one thread, and `schedule`
/// names which process may take its next step. The schedule chooses
/// everything else about the interleaving. Every demonstration in the
/// exercises that must show one exact timing diagram -- the lost updates
/// of 3.38, the surviving outcomes of 3.39, the exchange races of 3.43,
/// and the deadlocks of 3.45 to 3.49 -- runs through here.
///
/// # Panics
///
/// Panics if a scheduled step never runs (see [`Handshake`]), or if a
/// step itself panics.
pub fn run_forced(procs: Vec<Vec<Box<dyn FnOnce() + Send>>>, schedule: &[usize]) {
    let run = RunHandle::new_run();
    let run_ref = &run;
    let hands: Vec<Handshake> = (0..procs.len()).map(|_| Handshake::new()).collect();
    let hands_ref = &hands;
    std::thread::scope(|scope| {
        for (p, steps) in procs.into_iter().enumerate() {
            scope.spawn(move || {
                for step in steps {
                    if !hands_ref[p].take_step(run_ref, step) {
                        return;
                    }
                }
            });
        }
        for &p in schedule {
            hands_ref[p].drive_step(run_ref);
        }
    });
}
