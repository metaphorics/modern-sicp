(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.4 exercise 2.73 *)

type expr =
  | Const of int
  | Var of string
  | Compound of string * expr list

val operator : expr -> string
val operands : expr -> expr list

type deriv_rule = (expr -> string -> expr) -> expr list -> string -> expr
type table

val make_table : unit -> table
val install_sum_rule : table -> unit
val install_product_rule : table -> unit
val install_expt_rule : table -> unit
val deriv : table -> expr -> string -> expr
val make_sum : expr -> expr -> expr
val make_product : expr -> expr -> expr
val make_expt : expr -> int -> expr

(** [ex_2_73 ()] installs the sum, product, and power rules into a
    fresh table and differentiates [x + 3], [x * y], and [x**3], all
    with respect to [x]. *)
val ex_2_73 : unit -> expr * expr * expr
