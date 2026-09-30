(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.10 *)

(** Exercise 4.10: a new syntax for an unchanged evaluator. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** One term of the new syntax. *)
type term =
  | Lit of Sicp_common.Ast.scalar (** A literal. *)
  | Name of string (** A variable. *)
  | Fn of string list * term (** A function of one or more parameters. *)
  | Do of term list (** A nonempty sequence, answering its last term. *)
  | Call of term * term list (** An application to one or more arguments. *)
  | Op of string * term * term
  (** A binary operator named [+], [-], [*], [/], [=], [<], [>], or [^]. *)
  | When of term * term * term (** A conditional. *)
  | Rec of string * term * term
  (** [Rec (name, definition, body)] binds [name] recursively. *)

(** [from_new_syntax term] is the checked syntax of [term], or an
    [Invalid_form] error for an empty [Do], a [Fn] without parameters, a
    [Call] without arguments, or an unknown operator name. *)
val from_new_syntax : term -> (Sicp_common.Ast.expr, Sicp_common.Eval_error.t) result

(** [eval_term term env] translates [term] and evaluates it with the
    unmodified standard evaluator. *)
val eval_term
  :  term
  -> Sicp_common.Env.t
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [ex_4_10 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_10 : unit -> string list
