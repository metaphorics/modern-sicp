(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.97 *)

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
let negate_terms _a0 = raise Sicp_common.Pending.Pending_solution
let sub_terms _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let mul_term_by_all_terms _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let mul_terms _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let mul_poly _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let div_terms _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let remainder_terms _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let integerizing_power _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let pseudoremainder_terms _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let gcd_terms _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let int_gcd _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let coefficients_content _a0 = raise Sicp_common.Pending.Pending_solution
let reduce_content_pair _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let max_order _a0 = raise Sicp_common.Pending.Pending_solution
let leading_term _a0 = raise Sicp_common.Pending.Pending_solution
let reduce_terms _a0 _a1 = raise Sicp_common.Pending.Pending_solution

type rational_function =
  { numer : poly
  ; denom : poly
  }

let make_rational_function _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let reduce_poly _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let ex_2_97 () = raise Sicp_common.Pending.Pending_solution
