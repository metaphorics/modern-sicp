(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.25: [unless] breaks under applicative order. The lazy
    evaluator computes the unless factorial; the applicative-order
    contrast runs fuel-bounded. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [ex_4_25 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_25 : unit -> string list
