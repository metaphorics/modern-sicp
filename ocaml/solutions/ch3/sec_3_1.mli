(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.1 exercise 3.1 *)

(** Exercise 3.1: an accumulator that keeps a running sum across calls,
    the same captured-[ref] idiom @ref{3.1.1} uses for
    [make_withdraw]. *)

(** [make_accumulator initial] is an accumulator that starts at
    [initial]; each call adds its argument to the running sum and
    returns the new sum. *)
val make_accumulator : int -> int -> int

(** [ex_3_01 ()] is the pair of sums the statement's two calls to
    [(A 10)] produce, in order. *)
val ex_3_01 : unit -> int * int
