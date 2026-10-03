(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.10 *)

(* The pending scaffold of the solution with the same name under
   solutions/ch4: every entry raises the pending marker until the
   exercise is solved. *)

type term =
  | Lit of Sicp_common.Ast.scalar
  | Name of string
  | Fn of string list * term
  | Do of term list
  | Call of term * term list
  | Op of string * term * term
  | When of term * term * term
  | Rec of string * term * term

let from_new_syntax _ = raise Sicp_common.Pending.Pending_solution
let eval_term _ = raise Sicp_common.Pending.Pending_solution
let ex_4_10 _ = raise Sicp_common.Pending.Pending_solution
