// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.9: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_09 {
    /// Adds one to its argument: the book's `inc`.
    fn inc(x: i64) -> i64 {
        x + 1
    }

    /// Subtracts one from its argument: the book's `dec`.
    fn dec(x: i64) -> i64 {
        x - 1
    }

    /// The first procedure: `inc` waits outside the recursive call, so
    /// each step defers one addition and the process is linear
    /// recursive.
    fn plus_deferred(a: i64, b: i64) -> i64 {
        if a == 0 {
            b
        } else {
            inc(plus_deferred(dec(a), b))
        }
    }

    /// The second procedure: the recursive call is the entire result of
    /// its branch, in tail position, so the process is linear iterative
    /// (in a language that reuses the frame; Rust's frame is real either
    /// way).
    fn plus_tail_shaped(a: i64, b: i64) -> i64 {
        if a == 0 {
            b
        } else {
            plus_tail_shaped(dec(a), inc(b))
        }
    }

    /// Exercise 1.9: the two addition methods, illustrated on `4 + 5`
    ///
    /// Returns the values the two procedures of the exercise produce for
    /// `(4, 5)`: the deferred-`inc` shape first, the tail-shaped shape
    /// second, together with an account of which process each generates.
    pub fn ex_1_09() -> [i64; 2] {
        [plus_deferred(4, 5), plus_tail_shaped(4, 5)]
    }
}

#[test]
fn ex_1_09() {
    let values = ex_1_09::ex_1_09();
    assert_eq!(values, [9, 9]);
}

/// Exercise 1.9a (this edition): measure how deep each spelling recurses
/// on a fixed small stack before its frames run out.
mod ex_1_09a {
    /// The stack budget within which a probe still counts as live: half
    /// the thread's stack, leaving headroom for the frame that discovers
    /// the budget is spent.
    fn budget_for(stack_bytes: usize) -> usize {
        stack_bytes / 2
    }

    /// How far the address of a fresh local has moved from `base`: the
    /// stack grows down, so this is the frame's depth in bytes.
    fn consumed(base: usize, probe: &u8) -> usize {
        let here = std::ptr::from_ref(probe) as usize;
        base.saturating_sub(here)
    }

    /// The deferred shape of exercise 1.9's first procedure: one frame
    /// stays live (via the `black_box` after the call) while its child
    /// recurses, exactly as the pending `inc` does.
    fn deferred_depth(base: usize, budget: usize) -> u64 {
        let probe = 0u8;
        if consumed(base, &probe) >= budget {
            0
        } else {
            let child = deferred_depth(base, budget);
            std::hint::black_box(&probe);
            1 + child
        }
    }

    /// The tail-shaped recursion of exercise 1.9's second procedure:
    /// Rust gives it no more of a frame discount than the first.
    fn tail_shape_depth(depth: u64, base: usize, budget: usize) -> u64 {
        let probe = 0u8;
        if consumed(base, &probe) >= budget {
            depth
        } else {
            tail_shape_depth(depth + 1, base, budget)
        }
    }

    /// Runs both probes on a thread whose stack is fixed at
    /// `stack_bytes`, and returns the frame depths reached.
    fn depths_on_stack(stack_bytes: usize) -> (u64, u64) {
        std::thread::Builder::new()
            .stack_size(stack_bytes)
            .spawn(move || {
                let anchor = 0u8;
                let base = std::ptr::from_ref(&anchor) as usize;
                let budget = budget_for(stack_bytes);
                (
                    deferred_depth(base, budget),
                    tail_shape_depth(0, base, budget),
                )
            })
            .expect("spawning a probe thread does not fail on this platform")
            .join()
            .expect("the probe thread never panics: it stops at the budget, not the guard page")
    }

    /// Runs the loop form of the second procedure of exercise 1.9 on the
    /// same small stack for a million steps, to show it needs none of
    /// the recursive versions' depth.
    fn loop_runs_unbounded(stack_bytes: usize, steps: i64) -> i64 {
        std::thread::Builder::new()
            .stack_size(stack_bytes)
            .spawn(move || {
                let (mut a, mut b) = (steps, 0);
                while a != 0 {
                    a -= 1;
                    b += 1;
                }
                b
            })
            .expect("spawning a probe thread does not fail on this platform")
            .join()
            .expect("the loop never panics")
    }

    /// Exercise 1.9a: measure the frame depth on a 64 KiB stack, confirm
    /// depth roughly doubles on a 256 KiB stack, and confirm the loop
    /// runs a million steps on the smaller stack untouched.
    pub fn ex_1_09a() -> (u64, u64) {
        const SMALL_STACK: usize = 64 * 1024;
        const LARGE_STACK: usize = 256 * 1024;

        let (small_deferred, small_tail) = depths_on_stack(SMALL_STACK);
        let (large_deferred, large_tail) = depths_on_stack(LARGE_STACK);

        // Frames per call are constant, so quadrupling the stack should
        // roughly quadruple the reachable depth; a loose band (2x to 8x,
        // checked with plain integer multiplication) absorbs
        // build-dependent frame sizes without a lossy float conversion.
        assert!(large_deferred >= small_deferred.saturating_mul(2));
        assert!(large_deferred <= small_deferred.saturating_mul(8));
        assert!(large_tail >= small_tail.saturating_mul(2));
        assert!(large_tail <= small_tail.saturating_mul(8));

        assert_eq!(loop_runs_unbounded(SMALL_STACK, 1_000_000), 1_000_000);

        (small_deferred, small_tail)
    }
}

#[test]
fn ex_1_09a() {
    let (deferred, tail_shaped) = ex_1_09a::ex_1_09a();
    assert!(deferred > 10);
    assert!(tail_shaped > 10);
}
