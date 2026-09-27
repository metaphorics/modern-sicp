(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.19: the three scoping disciplines for internal definitions. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** The observation traces of the statement's program under the
      sequential rule, Alyssa's scan-out, and Eva's simultaneous
      rule. *)
val sequential_trace : unit -> string list

val alyssa_trace : unit -> string list
val eva_trace : unit -> string list

(** [ex_4_19 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_19 : unit -> string list
