(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.5 *)

(* The pending scaffold of the solution with the same name under
   solutions/ch4: every entry raises the pending marker until the
   exercise is solved. *)

type clause =
  | Test of Sicp_common.Ast.expr * Sicp_common.Ast.expr
  | Arrow of Sicp_common.Ast.expr * Sicp_common.Ast.expr
  | Else of Sicp_common.Ast.expr

let cond_to_expr _ = raise Sicp_common.Pending.Pending_solution
let eval_cond _ = raise Sicp_common.Pending.Pending_solution
let ex_4_05 _ = raise Sicp_common.Pending.Pending_solution
