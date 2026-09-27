(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.4 exercise 2.76 *)

type explicit_z =
  | Rectangular of float * float
  | Polar of float * float

let real_part_explicit _a0 = raise Sicp_common.Pending.Pending_solution
let magnitude_explicit _a0 = raise Sicp_common.Pending.Pending_solution

type dd_value =
  | Num of float
  | Pair of float * float

type dd_tagged =
  { tag : string
  ; contents : dd_value
  }

type dd_table = unit

let dd_make_table () = raise Sicp_common.Pending.Pending_solution
let dd_install_rectangular _a0 = raise Sicp_common.Pending.Pending_solution
let dd_install_polar _a0 = raise Sicp_common.Pending.Pending_solution
let dd_apply_generic _a0 _a1 _a2 = raise Sicp_common.Pending.Pending_solution

type mp_op =
  | Real_part
  | Magnitude

type mp_z = mp_op -> float

let mp_rectangular _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let mp_polar _a0 _a1 = raise Sicp_common.Pending.Pending_solution

type sample = float * float

let ex_2_76 () = raise Sicp_common.Pending.Pending_solution
