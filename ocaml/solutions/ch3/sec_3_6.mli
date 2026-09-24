(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.1 exercise 3.6 *)

(** Exercise 3.6: a [rand] that can be reset to reproduce a sequence.
    Scheme's [((rand 'reset) new-value)] two-level application
    flattens into one [Reset new_value] message, since a variant
    payload carries the value [rand 'reset] would otherwise need a
    second application just to receive. *)

type rand_message =
  | Generate
  | Reset of int64

(** [make_rand seed] is a [rand] procedure seeded with [seed]: [rand
    Generate] is [Some] freshly drawn value; [rand (Reset new_seed)]
    reseeds the generator from [new_seed] and answers [None], since
    resetting has no useful value of its own. *)
val make_rand : int64 -> rand_message -> int option

(** [ex_3_06 ()] is a value generated, then generated again after a
    reset back to the original seed: the pair is equal, showing the
    reset reproduces the earlier sequence. *)
val ex_3_06 : unit -> int * int
