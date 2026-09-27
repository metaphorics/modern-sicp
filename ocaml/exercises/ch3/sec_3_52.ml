(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The pending scaffold of the solution with the same name under
   solutions/ch3: every entry raises the pending marker until the
   exercise is solved. *)

type 'a thunk_stream =
  | TCons of 'a * (unit -> 'a thunk_stream)
  | TEmpty

let thunk_map = raise Sicp_common.Pending.Pending_solution
let thunk_filter = raise Sicp_common.Pending.Pending_solution
let thunk_ref = raise Sicp_common.Pending.Pending_solution
let thunk_take = raise Sicp_common.Pending.Pending_solution
let ex_3_52 = raise Sicp_common.Pending.Pending_solution
