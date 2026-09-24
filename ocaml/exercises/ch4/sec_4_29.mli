(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.29: the memoization difference of 4.29; 4.29a adds the memoization toggle and the thunk counters. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [ex_4_29 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_29 : unit -> string list

(** [ex_4_29a ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_29a : unit -> string list
