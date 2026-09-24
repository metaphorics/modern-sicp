(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.2: Louis Reasoner's reordered clauses and the call-prefix language. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [eval] is the call-prefixed evaluator of part (b): every
      application is written [(call f x ...)]. *)
val eval : Sicp_ch4.Sec_4_1.eval_t

(** [ex_4_02 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_02 : unit -> string list
