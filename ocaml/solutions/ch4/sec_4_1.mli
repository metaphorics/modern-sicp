(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.1: left-to-right and right-to-left operand evaluation. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [eval] is the base evaluator; the exercise pins both operand
      orders down explicitly. *)
val eval : Sicp_ch4.Sec_4_1.eval_t

(** [ex_4_01 ()] is [(left_to_right, right_to_left)]: the evaluation
    logs of one application's operands under each order, each log
    naming the operands in the order they were evaluated. *)
val ex_4_01 : unit -> string list * string list
