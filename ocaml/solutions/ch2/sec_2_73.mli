(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.4 exercise 2.73 *)

(** An algebraic expression: a constant, a variable, or a compound
    expression built from an operator symbol and its operand list.
    [Compound] plays the role of the book's tagged list
    [(operator . operands)]: the operator string is the "type tag"
    the data-directed table dispatches [deriv] on. *)
type expr =
  | Const of int
  | Var of string
  | Compound of string * expr list

(** [operator exp] and [operands exp] are the operator symbol and
    operand list of a compound expression. [invalid_arg] on [Const]
    or [Var], which carry no operator to dispatch on -- the reason
    [number?] and [variable?] cannot be assimilated into the table. *)
val operator : expr -> string

val operands : expr -> expr list

(** One entry of the deriv table: given the current [deriv] function
    (so a rule can differentiate its own operands) and a variable
    name, a rule turns its operand list into a derivative. *)
type deriv_rule = (expr -> string -> expr) -> expr list -> string -> expr

(** The data-directed deriv table: one [deriv_rule] per operator
    string. *)
type table

val make_table : unit -> table

(** [install_sum_rule table] and [install_product_rule table] add the
    book's two differentiation rules to [table] -- part (b). *)
val install_sum_rule : table -> unit

val install_product_rule : table -> unit

(** [install_expt_rule table] adds the power rule for
    [Compound ("**", [base; Const n])], differentiating [base^n] as
    [n * base^(n - 1) * d(base)/dx] -- part (c), Exercise 2.56's rule
    installed as the "additional rule" the exercise asks for. *)
val install_expt_rule : table -> unit

(** [deriv table exp var] is the derivative of [exp] with respect to
    [var]. [Const] and [Var] are handled directly, exactly as the
    book's [cond] does before ever consulting the table; a [Compound]
    dispatches through [table] on its operator. [invalid_arg] when no
    rule is installed for that operator. *)
val deriv : table -> expr -> string -> expr

(** [make_sum]/[make_product]/[make_expt] build a [Compound]
    expression, folding in the same reductions ([x + 0 = x],
    [x * 1 = x], [x * 0 = 0], constant folding, [x^0 = 1], [x^1 = x])
    Exercise 2.56's constructors use. *)
val make_sum : expr -> expr -> expr

val make_product : expr -> expr -> expr
val make_expt : expr -> int -> expr

(** [ex_2_73 ()] installs the sum, product, and power rules into a
    fresh table and differentiates [x + 3], [x * y], and [x**3], all
    with respect to [x]. *)
val ex_2_73 : unit -> expr * expr * expr
