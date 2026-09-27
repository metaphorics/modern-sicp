(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.58 *)

(** A fully parenthesized or standard-notation infix expression. *)
type infix =
  | Num of int
  | Var of string
  | Plus of infix * infix
  | Times of infix * infix

val is_number : infix -> int -> bool
val make_sum : infix -> infix -> infix
val make_product : infix -> infix -> infix

(** [deriv exp var] is the derivative of the infix expression [exp]
    with respect to the variable named [var]. *)
val deriv : infix -> string -> infix

(** Part (a): [x + (3 * (x + (y + 2)))], fully parenthesized. *)
val ex_2_58_parenthesized_expr : infix

val ex_2_58_parenthesized : unit -> infix

(** One token of standard infix notation. *)
type tok =
  | TNum of int
  | TVar of string
  | TPlus
  | TTimes
  | TOpen
  | TClose

(** [parse_standard toks] is the [infix] tree [toks] names under the
    usual precedence, where [*] binds tighter than [+].
    [invalid_arg] on a malformed token list. *)
val parse_standard : tok list -> infix

(** Part (b): [x + 3 * (x + y + 2)], written without the parentheses
    part (a) requires around every sum. *)
val ex_2_58_standard_tokens : tok list

val ex_2_58_standard : unit -> infix
