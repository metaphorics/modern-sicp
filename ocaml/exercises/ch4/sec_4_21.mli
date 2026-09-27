(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.21: recursion without define. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [eval] is the base evaluator, run on programs whose procedures
      are plain lambdas. *)
val eval : Sicp_ch4.Sec_4_1.eval_t

(** [ex_4_21 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_21 : unit -> string list
