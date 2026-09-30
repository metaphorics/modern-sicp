(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.4 *)

(** Exercise 4.4: [&&] and [||] as special forms and as derived
    expressions.

    OCaml's connectives are binary and their operands are [bool], so a
    connective answers a Boolean rather than the value of its last
    operand.  Both treatments keep the short circuit: the right operand
    runs only when the left one does not decide.  The book's variadic
    forms, including the empty ones, are derived by [conjunction] and
    [disjunction]. *)

(** [eval_and eval left right env] evaluates [left], and [right] only
    when [left] is [true]. *)
val eval_and
  :  Sicp_ch4.Sec_4_1.eval_t
  -> Sicp_common.Ast.expr
  -> Sicp_common.Ast.expr
  -> Sicp_common.Env.t
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [eval_or eval left right env] evaluates [left], and [right] only
    when [left] is [false]. *)
val eval_or
  :  Sicp_ch4.Sec_4_1.eval_t
  -> Sicp_common.Ast.expr
  -> Sicp_common.Ast.expr
  -> Sicp_common.Env.t
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [eval_special] is the evaluator whose [&&] and [||] clauses are
    [eval_and] and [eval_or]. *)
val eval_special : Sicp_ch4.Sec_4_1.eval_t

(** [and_to_if a b] is [if a then b else false]. *)
val and_to_if : Sicp_common.Ast.expr -> Sicp_common.Ast.expr -> Sicp_common.Ast.expr

(** [or_to_if a b] is [if a then true else b]. *)
val or_to_if : Sicp_common.Ast.expr -> Sicp_common.Ast.expr -> Sicp_common.Ast.expr

(** [derive e] is [e] with every [&&] and [||] at every depth rewritten
    into a conditional. *)
val derive : Sicp_common.Ast.expr -> Sicp_common.Ast.expr

(** [eval_derived] evaluates [derive e] with the standard evaluator. *)
val eval_derived : Sicp_ch4.Sec_4_1.eval_t

(** [conjunction es] is the conjunction of [es] from left to right as
    nested conditionals; the empty conjunction is [true]. *)
val conjunction : Sicp_common.Ast.expr list -> Sicp_common.Ast.expr

(** [disjunction es] is the disjunction of [es] from left to right as
    nested conditionals; the empty disjunction is [false]. *)
val disjunction : Sicp_common.Ast.expr list -> Sicp_common.Ast.expr

(** [ex_4_04 ()] evaluates four connective expressions, two of which
    would divide by zero without the short circuit, each as
    ["special / derived"]; then the empty conjunction, the empty
    disjunction, and two variadic ones whose last operand would divide
    by zero. *)
val ex_4_04 : unit -> string list
