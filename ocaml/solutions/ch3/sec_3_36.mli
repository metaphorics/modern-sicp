(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.36 *)

(** Exercise 3.36, replaced for this edition: tracing the connector's
    closure call graph instead of drawing it. *)

(** [ex_3_36 ()] is [(the dispatch trace when a is set by the user, the
    trace when the user retracts)]: for the set, the constraint's me
    dispatch and its process_new_value, reached through
    for_each_except from set_my_value; for the forget, the
    I-lost-my-value dispatch and process_forget_value. *)
val ex_3_36 : unit -> string list * string list
