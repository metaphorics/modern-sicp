(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.47:
   two frame constructors *)

type vect =
  { x : float
  ; y : float
  }

type frame1 = vect * vect * vect
type frame2 = vect * (vect * vect)

let make_frame1 _o _e1 _e2 = raise Sicp_common.Pending.Pending_solution
let origin_frame1 _f = raise Sicp_common.Pending.Pending_solution
let edge1_frame1 _f = raise Sicp_common.Pending.Pending_solution
let edge2_frame1 _f = raise Sicp_common.Pending.Pending_solution
let make_frame2 _o _e1 _e2 = raise Sicp_common.Pending.Pending_solution
let origin_frame2 _f = raise Sicp_common.Pending.Pending_solution
let edge1_frame2 _f = raise Sicp_common.Pending.Pending_solution
let edge2_frame2 _f = raise Sicp_common.Pending.Pending_solution
let ex_2_47 () = raise Sicp_common.Pending.Pending_solution
