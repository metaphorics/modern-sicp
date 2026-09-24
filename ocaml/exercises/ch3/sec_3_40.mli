(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.4 exercise 3.40 *)

(** Exercise 3.40: all values [x] can take when one process sets it to
    [x * x] and another, concurrently, to [x * x * x], and what
    remains when both are serialized. *)

(** [unserialized_values ()] is the sorted list of every final value
    the two processes can produce, obtained by enumerating all
    interleavings of their reads and writes. *)
val unserialized_values : unit -> int list

(** [serialized_value ()] is the one value that remains when both
    processes run under the same serializer. *)
val serialized_value : unit -> int

(** [sample_unserialized_runs n] really runs the unserialized race [n]
    times and answers the sorted distinct finals observed. *)
val sample_unserialized_runs : int -> int list

(** [ex_3_40 ()] is the unserialized enumeration and the serialized
    singleton, in that order. *)
val ex_3_40 : unit -> int list * int list
