(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.14 *)

(** Exercise 4.14: a host higher-order function installed as a primitive. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [eval] is the standard evaluator run in a global environment whose
    [List.map] is [louis_map]. *)
val eval : Sicp_ch4.Sec_4_1.eval_t

(** [ex_4_14 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_14 : unit -> string list

(** [ex_4_14a ()] sorts a list with a guest comparator through Louis's
    [List.sort], through Eva's guest insertion sort, and through the
    prelude's [List.sort]. *)
val ex_4_14a : unit -> string list
