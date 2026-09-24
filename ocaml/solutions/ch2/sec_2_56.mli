(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.56 *)

(** An algebraic expression, extended with exponentiation. *)
type expr =
  | Const of int
  | Var of string
  | Sum of expr * expr
  | Prod of expr * expr
  | Power of expr * expr

val is_number : expr -> int -> bool
val same_variable : expr -> expr -> bool
val make_sum : expr -> expr -> expr
val make_product : expr -> expr -> expr

(** [base]/[exponent] extract the named part of a power.
    [invalid_arg] on any other expression. *)
val base : expr -> expr

val exponent : expr -> expr

(** [make_exponentiation base exponent] builds [base ^ exponent],
    simplified when the exponent is 0 or 1 or both parts are numbers. *)
val make_exponentiation : expr -> expr -> expr

(** [deriv exp var] is the derivative of [exp] with respect to the
    variable named [var]. *)
val deriv : expr -> string -> expr

(** [ex_2_56 ()] is the derivative of [x^3] with respect to [x]. *)
val ex_2_56 : unit -> expr
