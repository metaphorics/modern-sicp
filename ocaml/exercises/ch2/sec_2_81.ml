(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.81 *)

type value =
  | Num of float
  | Tagged of tagged

and tagged =
  { tag : string
  ; contents : value
  }

let attach_tag _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let type_tag _a0 = raise Sicp_common.Pending.Pending_solution
let contents_of _a0 = raise Sicp_common.Pending.Pending_solution
let put _a0 _a1 _a2 = raise Sicp_common.Pending.Pending_solution
let get _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let put_coercion _a0 _a1 _a2 = raise Sicp_common.Pending.Pending_solution
let get_coercion _a0 _a1 = raise Sicp_common.Pending.Pending_solution

exception Loop_detected of int

let apply_generic_loop _a0 _a1 _a2 = raise Sicp_common.Pending.Pending_solution
let apply_generic_fixed _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let install_louis_setup () = raise Sicp_common.Pending.Pending_solution
let ex_2_81_a () = raise Sicp_common.Pending.Pending_solution
let ex_2_81_c () = raise Sicp_common.Pending.Pending_solution
