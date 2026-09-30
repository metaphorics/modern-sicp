(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.18 *)

(** Exercise 4.18: an alternative scan-out strategy. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [scan_out_alternative e] is the alternative scanned-out form of the
    [let rec] group [e], or [e] itself when [e] is not one. *)
val scan_out_alternative : Sicp_common.Ast.expr -> Sicp_common.Ast.expr

(** [eval_alternative] is the evaluator of [Sec_4_16.scanning] with
    [scan_out_alternative]. *)
val eval_alternative : Sicp_ch4.Sec_4_1.eval_t

(** [ex_4_18 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_18 : unit -> string list
