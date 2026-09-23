(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.29:
   binary mobiles *)

type structure =
  | Weight of int
  | Hanging of mobile

and mobile = Mobile of branch * branch
and branch = Branch of int * structure

let left_branch _mobile = raise Sicp_common.Pending.Pending_solution
let right_branch _mobile = raise Sicp_common.Pending.Pending_solution
let branch_length _branch = raise Sicp_common.Pending.Pending_solution
let branch_structure _branch = raise Sicp_common.Pending.Pending_solution
let ex_2_29 _mobile = raise Sicp_common.Pending.Pending_solution
let total_weight _mobile = raise Sicp_common.Pending.Pending_solution
let balanced _mobile = raise Sicp_common.Pending.Pending_solution

module Cons_repr = struct
  type structure =
    | Weight of int
    | Hanging of mobile

  and mobile = branch * branch
  and branch = int * structure

  let total_weight _mobile = raise Sicp_common.Pending.Pending_solution
  let balanced _mobile = raise Sicp_common.Pending.Pending_solution
end
