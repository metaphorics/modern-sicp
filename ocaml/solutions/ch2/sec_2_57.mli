(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.57 *)

(** An algebraic expression over sums and products of any number of
    terms. *)
type expr =
  | Const of int
  | Var of string
  | Sum of expr list
  | Prod of expr list

val is_number : expr -> int -> bool
val same_variable : expr -> expr -> bool
val make_sum : expr list -> expr
val make_product : expr list -> expr

(** [addend]/[augend] split a sum into its first term and the sum of
    the rest; [multiplier]/[multiplicand] do the same for a product.
    [invalid_arg] when the expression is of the other kind. *)
val addend : expr -> expr

val augend : expr -> expr
val multiplier : expr -> expr
val multiplicand : expr -> expr

(** [deriv exp var] is the derivative of [exp] with respect to the
    variable named [var]. *)
val deriv : expr -> string -> expr

(** [ex_2_57 ()] is the derivative of [x * y * (x + 3)] with respect
    to [x]. *)
val ex_2_57 : unit -> expr
