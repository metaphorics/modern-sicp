(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.16 *)

(** Exercise 4.16: scanning out internal definitions.

    A procedure's internal definitions are the [let rec] group that opens
    its body.  Scanning out turns the group into one [let] of fresh
    reference cells holding the unassigned marker, followed by an
    assignment per definition and then the body; every read of a scanned
    name becomes a dereference.  (a) A dereference that finds the marker
    is an "unassigned variable" error.  (b) [scan_out_defines] performs
    the transformation.  (c) The evaluator installs it where a procedure
    is made, the [fun] clause, so it runs once per closure rather than
    once per call.

    OCaml's admission refuses a group whose right-hand side computes
    with a name of the group, such as [let rec a = b + 1 and b = 2].  It
    does admit statically constructive values, such as the cyclic list
    [let rec xs = 1 :: ys and ys = 2 :: xs], which OCaml builds by
    backpatching.  Scanning out reads [ys] while assigning [xs], so that
    admitted group reaches the unassigned error. *)

(** [unassigned] is the expression of the unassigned marker. *)
val unassigned : Sicp_common.Ast.expr

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

(** [ex_4_16 ()] runs mutually recursive internal procedures, then a
    cyclic list, each under [eval] and under the standard evaluator;
    then a group admission rejects; and finally names the cells scanning
    creates for the procedures. *)
val ex_4_16 : unit -> (string list, Sicp_common.Eval_error.t) result
