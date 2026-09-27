(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.77 *)

type value =
  | Num of float
  | Cpx of float * float
  | Tagged of tagged

and tagged =
  { tag : string
  ; contents : value
  }

let attach_tag _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let type_tag _a0 = raise Sicp_common.Pending.Pending_solution
let contents_of _a0 = raise Sicp_common.Pending.Pending_solution
let apply_generic _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let call_count () = raise Sicp_common.Pending.Pending_solution
let reset_call_count () = raise Sicp_common.Pending.Pending_solution
let install_rectangular_package () = raise Sicp_common.Pending.Pending_solution
let real_part _a0 = raise Sicp_common.Pending.Pending_solution
let imag_part _a0 = raise Sicp_common.Pending.Pending_solution
let magnitude _a0 = raise Sicp_common.Pending.Pending_solution
let angle _a0 = raise Sicp_common.Pending.Pending_solution
let install_alyssa_complex_fix () = raise Sicp_common.Pending.Pending_solution
let ex_2_77 () = raise Sicp_common.Pending.Pending_solution
