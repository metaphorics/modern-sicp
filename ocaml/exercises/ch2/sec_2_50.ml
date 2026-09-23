(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.50:
   flip-horiz and the rotations *)

type vect =
  { x : float
  ; y : float
  }

type frame =
  { origin : vect
  ; edge1 : vect
  ; edge2 : vect
  }

type segment = vect * vect
type painter = frame -> segment list

let ex_2_50_flip_horiz _painter = raise Sicp_common.Pending.Pending_solution
let ex_2_50_rotate_180 _painter = raise Sicp_common.Pending.Pending_solution
let ex_2_50_rotate_270 _painter = raise Sicp_common.Pending.Pending_solution
