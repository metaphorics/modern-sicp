(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.87 *)

type coeff =
  | Flt of float
  | Poly of poly

and term =
  { order : int
  ; coeff : coeff
  }

and poly =
  { var : string
  ; term_list : term list
  }

let make_term _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let make_poly _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let is_zero_coeff _a0 = raise Sicp_common.Pending.Pending_solution
let adjoin_term _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let ex_2_87 () = raise Sicp_common.Pending.Pending_solution
