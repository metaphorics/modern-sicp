(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.16 *)

(** Exercise 4.16: scanning out internal definitions. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [is_unassigned v] is [true] when [v] is the unassigned marker. *)
val is_unassigned : Sicp_common.Value.t -> bool

(** [pattern_names p] is the variables [p] binds. *)
val pattern_names : Sicp_common.Ast.pattern -> string list

(** [deref_names names e] is [e] with every free occurrence of a name of
    [names] replaced by its dereference.  An occurrence under a binder
    that rebinds the name is left alone. *)
val deref_names : string list -> Sicp_common.Ast.expr -> Sicp_common.Ast.expr

(** [scan_out_let_rec e] is the scanned-out form of the [let rec] group
    [e], or [e] itself when [e] is not a [let rec]. *)
val scan_out_let_rec : Sicp_common.Ast.expr -> Sicp_common.Ast.expr

(** [scan_out_defines body] is the procedure body [body] with its
    internal definitions scanned out. *)
val scan_out_defines : Sicp_common.Ast.expr -> Sicp_common.Ast.expr

(** [scanning scan ~self] is one step of an evaluator that applies
    [scan] to the body of every procedure it makes and reports a
    dereference of the unassigned marker as an error; every other node
    falls back on the standard dispatch. *)
val scanning
  :  (Sicp_common.Ast.expr -> Sicp_common.Ast.expr)
  -> self:Sicp_ch4.Sec_4_1.eval_t
  -> Sicp_ch4.Sec_4_1.eval_t

(** [eval] is the fixed point of [scanning scan_out_defines]. *)
val eval : Sicp_ch4.Sec_4_1.eval_t

(** [ex_4_16 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_16 : unit -> (string list, Sicp_common.Eval_error.t) result
