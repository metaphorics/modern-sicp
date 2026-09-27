(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.93 *)

type term =
  { order : int
  ; coeff : float
  }

type poly =
  { var : string
  ; term_list : term list
  }

let make_term _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let make_poly _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let adjoin_term _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let add_terms _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let mul_terms _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let add_poly _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let mul_poly _a0 _a1 = raise Sicp_common.Pending.Pending_solution

type rational_function =
  { numer : poly
  ; denom : poly
  }

let make_rational_function _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let add_rational_function _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let ex_2_93 () = raise Sicp_common.Pending.Pending_solution
