// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.6: rand with generate and
//! reset messages.

use ch03::sec_3_1::RANDOM_INIT;

mod ex_3_06 {
    use ch03::sec_3_1::{RANDOM_INIT, rand_update};
    use sicp_runtime::SicpError;

    /// A message to the resettable generator: the book's `generate` and
    /// `reset` symbols became values.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Command {
        /// Produce and return the next random number.
        Generate,
        /// Reset the hidden state to this word.
        Reset(u64),
    }

    /// Exercise 3.6: rand with generate and reset messages
    ///
    /// The generator keeps its state in one word; [`MessageRand::handle`]
    /// dispatches on the message, so a reset from outside reaches the same
    /// state that [`Command::Generate`] advances.
    pub struct MessageRand {
        x: u64,
    }

    /// Starts the message-passing generator at `seed`.
    ///
    /// # Errors
    /// Returns [`SicpError::ZeroSeed`] when `seed` is zero, matching the
    /// section's generator.
    pub fn make_rand(seed: u64) -> Result<MessageRand, SicpError> {
        if seed == 0 {
            return Err(SicpError::ZeroSeed);
        }
        Ok(MessageRand { x: seed })
    }

    impl MessageRand {
        /// Handles one message: [`Command::Generate`] answers the next
        /// number, [`Command::Reset`] answers nothing and just moves the
        /// state.
        pub fn handle(&mut self, command: Command) -> Option<u64> {
            match command {
                Command::Generate => {
                    self.x = rand_update(self.x);
                    Some(self.x)
                }
                Command::Reset(new_value) => {
                    self.x = new_value;
                    None
                }
            }
        }
    }

    fn take_three(machine: &mut MessageRand) -> [u64; 3] {
        let mut drawn = [0_u64; 3];
        for slot in &mut drawn {
            if let Some(word) = machine.handle(Command::Generate) {
                *slot = word;
            }
        }
        drawn
    }

    /// Exercise 3.6: rand with generate and reset messages
    ///
    /// Returns whether three draws replay identically after a reset to the
    /// original state, and whether a reset to a different state produces a
    /// different continuation.
    #[must_use]
    pub fn ex_3_06() -> (bool, bool) {
        let mut machine = make_rand(RANDOM_INIT).expect("RANDOM_INIT is valid");
        let original = take_three(&mut machine);
        machine.handle(Command::Reset(RANDOM_INIT));
        let replayed = take_three(&mut machine);
        let replays = replayed == original;

        let mut machine = make_rand(RANDOM_INIT).expect("RANDOM_INIT is valid");
        let original = take_three(&mut machine);
        machine.handle(Command::Reset(42));
        let moved = take_three(&mut machine);
        let differs = moved != original;
        (replays, differs)
    }
}

#[test]
fn ex_3_06() {
    assert_eq!(ex_3_06::ex_3_06(), (true, true));

    // The reset reachability the exercise asks for: the first draw
    // after `Reset(x)` is `rand_update(x)`, the same value the original
    // run produced from that state.
    let mut machine = ex_3_06::make_rand(RANDOM_INIT).expect("RANDOM_INIT is valid");
    let first = machine.handle(ex_3_06::Command::Generate);
    machine.handle(ex_3_06::Command::Reset(RANDOM_INIT));
    assert_eq!(machine.handle(ex_3_06::Command::Generate), first);
}
