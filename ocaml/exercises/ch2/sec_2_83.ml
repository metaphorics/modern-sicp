(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.83 *)

type value =
  | Int of int
  | Rat of int * int
  | Real of float
  | Cpx of float * float
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
let apply_generic _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let install_raise () = raise Sicp_common.Pending.Pending_solution
let raise_one_level _a0 = raise Sicp_common.Pending.Pending_solution
let ex_2_83 () = raise Sicp_common.Pending.Pending_solution
