(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.4 exercise 2.75 *)

type op =
  | Real_part
  | Imag_part
  | Magnitude
  | Angle

type t = op -> float

let apply_generic _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let make_from_real_imag _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let ex_2_75 _a0 _a1 = raise Sicp_common.Pending.Pending_solution
