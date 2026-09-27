(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.57 *)

(** Exercise 3.57: how many additions does the [fibs] definition
    perform, and how many without the memoized delay? The map class is
    [T]: the counts are measured, not argued, by running both stream
    implementations with a counting adder. *)

(** [memoized_additions n] extracts [n] elements of the book's fibs
    built over a counting adder and answers the number of additions:
    [n - 2], one per element past the first two, never repeated. *)
val memoized_additions : int -> int

(** [thunk_additions n] does the same over plain thunks, whose tails
    re-evaluate on every access: the additions grow exponentially. *)
val thunk_additions : int -> int

(** [ex_3_57 ()] is [n] (15), the memoized count, the thunk count, and
    whether the memoized count is [n - 2] exactly. *)
val ex_3_57 : unit -> int * int * int * bool
