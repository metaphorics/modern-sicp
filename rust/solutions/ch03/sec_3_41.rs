// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.41: Ben's serialized `balance`
//! against the text's unserialized one. Both are single atomic reads of
//! the same integer, so a reader running during a withdrawal reports
//! either the old or the new balance under either version: the forced
//! schedules leave identical observation sets, and no scenario tells
//! them apart.

use std::sync::{Arc, Mutex, PoisonError};

use ch03::sec_3_1::Reply;
use ch03::sec_3_4::{Account, Serializer, SharedInt, read_shared, shared_int};

/// Where a reader's observation is parked between the forced steps.
type Observation = Arc<Mutex<Vec<i128>>>;

/// The steps of one observation run: the withdrawal first, the balance
/// read second. The schedule decides how far the withdrawal gets before
/// the read runs.
type Steps = [Box<dyn FnOnce() + Send>; 2];

/// Ben's account: the account of the text with the balance message
/// answered through the serializer, the commented line of the exercise.
#[derive(Clone)]
struct BenAccount {
    balance: SharedInt,
    protected: Serializer,
}

impl BenAccount {
    /// The book's `make-account` with Ben's one change.
    fn new(initial: i128) -> Self {
        Self {
            balance: shared_int(initial),
            protected: Serializer::new(),
        }
    }

    /// Same as the text's account.
    fn withdraw(&self, amount: i128) -> Reply {
        self.protected
            .protect(|| ch03::sec_3_4::shared_withdraw(&self.balance, amount))
    }

    /// Ben's change: the read runs under the serializer.
    fn balance(&self) -> i128 {
        self.protected.protect(|| read_shared(&self.balance))
    }
}

/// The steps over Ben's account: the withdrawal and the read each own a
/// handle, so either can run on its own thread.
fn ben_observation_steps(observed: Observation) -> Steps {
    let account = BenAccount::new(100);
    let withdraw_account = account.clone();
    [
        Box::new(move || {
            let _ = withdraw_account.withdraw(50);
        }),
        Box::new(move || {
            observed
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .push(account.balance());
        }),
    ]
}

/// The same two steps over the text's account, whose balance message is
/// the plain unserialized read.
fn text_observation_steps(observed: Observation) -> Steps {
    let account = Account::new(100);
    let withdraw_account = account.clone();
    [
        Box::new(move || {
            let _ = withdraw_account.withdraw(50);
        }),
        Box::new(move || {
            observed
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .push(account.balance());
        }),
    ]
}

/// Forces the two steps on real threads with the reader's step first or
/// the banker's step first, per the two named callers.
fn forced_observation(steps: Steps, reader_first: bool) {
    let [withdraw, read] = steps;
    let reader = vec![read];
    let banker = vec![withdraw];
    let procs = if reader_first {
        vec![reader, banker]
    } else {
        vec![banker, reader]
    };
    ch03::sec_3_4::run_forced(procs, &[0, 1]);
}

/// Runs one observation with the reader forced before the withdrawal.
fn observe_reader_first(steps: Steps) {
    forced_observation(steps, true);
}

/// Runs one observation with the reader forced after the withdrawal.
fn observe_banker_first(steps: Steps) {
    forced_observation(steps, false);
}

/// Runs one observation into a fresh sink and answers what it saw.
fn observe_once(
    steps_for: &(dyn Fn(Observation) -> Steps + Sync),
    observe: fn(Steps),
) -> Vec<i128> {
    let sink: Observation = Arc::new(Mutex::new(Vec::new()));
    observe(steps_for(Arc::clone(&sink)));
    std::mem::take(&mut *sink.lock().unwrap_or_else(PoisonError::into_inner))
}

/// Observes the withdrawal under `steps_for`'s account version, with
/// the reader forced on both sides of it, and answers the sorted
/// observations.
fn sorted_observations(steps_for: &(dyn Fn(Observation) -> Steps + Sync)) -> Vec<i128> {
    let mut observations = observe_once(steps_for, observe_reader_first);
    observations.extend(observe_once(steps_for, observe_banker_first));
    observations.sort_unstable();
    observations
}

mod ex_3_41 {
    use super::{ben_observation_steps, sorted_observations, text_observation_steps};

    /// Exercise 3.41: should balance reads serialize
    ///
    /// Observes one balance read during a withdrawal under both
    /// versions, with the reader forced before and after the withdrawal,
    /// and answers the sorted observations of each version.
    #[must_use]
    pub fn ex_3_41() -> (Vec<i128>, Vec<i128>) {
        (
            sorted_observations(&text_observation_steps),
            sorted_observations(&ben_observation_steps),
        )
    }
}

#[test]
fn ex_3_41() {
    let (plain, ben) = ex_3_41::ex_3_41();
    // Either version, either schedule: the reader sees the balance
    // before the withdrawal (100) or after it (50). Both values are
    // balances the account really held; no torn or phantom value can
    // arise, because each version reads the integer in one atomic step.
    assert_eq!(plain, vec![50, 100]);
    assert_eq!(ben, vec![50, 100]);
    // The observation sets are identical: no scenario distinguishes
    // Ben's serialized read from the text's unserialized one.
    assert_eq!(plain, ben);
}
