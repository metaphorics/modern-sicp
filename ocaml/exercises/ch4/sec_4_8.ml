(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.8 *)

(* The pending scaffold of the solution with the same name under
   solutions/ch4: every entry raises the pending marker until the
   exercise is solved. *)

type named_let =
  { name : string
  ; bindings : (string * Sicp_common.Ast.expr) list
  ; body : Sicp_common.Ast.expr
  }

type let_form =
  | Plain of Sicp_common.Ast.expr
  | Named of named_let

let let_to_combination _ = raise Sicp_common.Pending.Pending_solution
let eval_let_form _ = raise Sicp_common.Pending.Pending_solution
let fib _ = raise Sicp_common.Pending.Pending_solution
let ex_4_08 _ = raise Sicp_common.Pending.Pending_solution
