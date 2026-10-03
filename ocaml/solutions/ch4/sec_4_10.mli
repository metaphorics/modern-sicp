(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.10 *)

(** Exercise 4.10: a new syntax for an unchanged evaluator.

    The evaluator reads syntax only through [Ast.view], so a different
    surface syntax needs only a translation into the checked syntax;
    [eval] and [apply] stay as they are.  The new syntax here is a
    small uniform term language: every operator is named by a string,
    sequences are lists, and recursion has its own [Rec] form. *)

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

(** [factorial] is a term computing the factorial of 6 by recursion. *)
val factorial : term

(** [ex_4_10 ()] evaluates [factorial], a printing sequence, a direct
    function call, and a term with an unknown operator. *)
val ex_4_10 : unit -> string list
