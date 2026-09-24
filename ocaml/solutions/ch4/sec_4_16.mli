(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.16: scan out internal definitions. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [scan_out_defines body] is the body without internal
      definitions: each defined name becomes an unassigned let binding
      set by assignment. *)
val scan_out_defines
  :  Sicp_common.Ast.expr list
  -> (Sicp_common.Ast.expr list, Sicp_common.Eval_error.t) result

(** [eval] scans definitions out of every lambda body and rejects
      reads of the unassigned marker. *)
val eval : Sicp_ch4.Sec_4_1.eval_t

(** [ex_4_16 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_16 : unit -> string list
