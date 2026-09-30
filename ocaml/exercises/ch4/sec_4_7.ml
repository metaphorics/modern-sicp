(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.7 *)

(* The pending scaffold of the solution with the same name under
   solutions/ch4: every entry raises the pending marker until the
   exercise is solved. *)

type let_star =
  { bindings : (string * Sicp_common.Ast.expr) list
  ; body : Sicp_common.Ast.expr
  }

let let_star_to_nested_lets _ = raise Sicp_common.Pending.Pending_solution
let eval_let_star _ = raise Sicp_common.Pending.Pending_solution
let ex_4_07 _ = raise Sicp_common.Pending.Pending_solution
