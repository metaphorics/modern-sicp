(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.23 *)

(* The pending scaffold of the solution with the same name under
   solutions/ch4: every entry raises the pending marker until the
   exercise is solved. *)

type execution =
  Sicp_common.Env.t -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

type counts =
  { mutable analysis_steps : int
  ; mutable runtime_steps : int
  }

type style =
  | Book
  | Alyssa

let flatten _ = raise Sicp_common.Pending.Pending_solution
let book_sequence _ = raise Sicp_common.Pending.Pending_solution
let alyssa_sequence _ = raise Sicp_common.Pending.Pending_solution
let analyze_sequence _ = raise Sicp_common.Pending.Pending_solution
let measure _ = raise Sicp_common.Pending.Pending_solution
let ex_4_23 _ = raise Sicp_common.Pending.Pending_solution
