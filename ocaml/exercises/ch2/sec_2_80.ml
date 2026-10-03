(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.80 *)

type value =
  | Num of float
  | Ratpair of int * int
  | Cpx of float * float
  | Bool of bool
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
let install_real_package () = raise Sicp_common.Pending.Pending_solution
let install_rational_package () = raise Sicp_common.Pending.Pending_solution
let install_complex_package () = raise Sicp_common.Pending.Pending_solution
let is_zero _a0 = raise Sicp_common.Pending.Pending_solution
let ex_2_80 () = raise Sicp_common.Pending.Pending_solution
