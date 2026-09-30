(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.29 with the tailored addition 4.29a: memoization and the
    thunk counters.  The section evaluator's state carries the
    memoization toggle: with it off, a force never records its result,
    so every force of a delayed thunk recomputes it.  [square (id 10)]
    answers 100 either way, but [count] advances once with memoization
    and twice without; [cube (id 10)] recomputes its operand three
    times without memoization.  4.29a reads the experiment's counts of
    one [square (id 10)] run under each mode. *)

(** [ex_4_29 ()] answers the interaction's transcript with memoization,
    then without: [square (id 10)], [count], [cube (id 10)], [count],
    and the experiment's four thunk counts. *)
val ex_4_29 : unit -> string list

(** [ex_4_29a ()] answers, for memoization on and then off, the value
    of [square (id 10)] and the run's allocation, force,
    recomputation, and memo-hit counts. *)
val ex_4_29a : unit -> string list
