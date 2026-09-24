(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.3: data-directed dispatch in eval. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [put tag handler] installs the handler of one expression type;
      [get tag] is its handler, or [None]. *)
val put
  :  string
  -> Sicp_common.Ast.expr
  -> Sicp_common.Value.env
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result
  -> unit

val get
  :  string
  -> Sicp_common.Ast.expr
  -> Sicp_common.Value.env
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result option

(** [eval] dispatches through the table, applications last. *)
val eval : Sicp_ch4.Sec_4_1.eval_t

(** [ex_4_03 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_03 : unit -> string list
