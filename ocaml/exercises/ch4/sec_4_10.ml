(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

type term =
  | Lit of Sicp_common.Value.t
  | Name of string
  | Fn of string list * term
  | Do of term list
  | Call of term * term list

(* The pending scaffold of the solution with the same name under
   solutions/ch4: every entry raises the pending marker until the
   exercise is solved. *)

let from_new_syntax = raise Sicp_common.Pending.Pending_solution
let eval = raise Sicp_common.Pending.Pending_solution
let ex_4_10 = raise Sicp_common.Pending.Pending_solution
