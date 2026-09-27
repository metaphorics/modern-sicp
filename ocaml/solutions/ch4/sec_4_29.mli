(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.29 with the tailored addition 4.29a: the memoization
    toggle and the thunk counters. The statement lives in the section;
    this signature is the pending exercise's public contract. *)

(** [ex_4_29 ()] runs the statement's interaction under both modes and
    answers the observable outcomes as printed strings. *)
val ex_4_29 : unit -> string list

(** [ex_4_29a ()] asserts the creation, forcing, and computation counts
    of one program under each mode, as printed strings. *)
val ex_4_29a : unit -> string list
