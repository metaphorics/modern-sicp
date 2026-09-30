(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.9 *)

(* The pending scaffold of the solution with the same name under
   solutions/ch4: every entry raises the pending marker until the
   exercise is solved. *)

type loop =
  | While of Sicp_common.Ast.expr * Sicp_common.Ast.expr
  | For of string * Sicp_common.Ast.expr * Sicp_common.Ast.expr * Sicp_common.Ast.expr

let loop_to_expr _ = raise Sicp_common.Pending.Pending_solution
let eval_loop _ = raise Sicp_common.Pending.Pending_solution
let summation _ = raise Sicp_common.Pending.Pending_solution
let ex_4_09 _ = raise Sicp_common.Pending.Pending_solution
