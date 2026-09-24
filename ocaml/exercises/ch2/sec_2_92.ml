(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.92 *)

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
let var_less _a0 _a1 = raise Sicp_common.Pending.Pending_solution

type coeff =
  | Num of float
  | Sub_poly of poly

let coeff_as_float _a0 = raise Sicp_common.Pending.Pending_solution
let coeff_as_poly _a0 = raise Sicp_common.Pending.Pending_solution
let add_across_variables _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let ex_2_92 () = raise Sicp_common.Pending.Pending_solution
