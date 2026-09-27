(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.52:
   square limit variations, and the added 2.52a *)

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

let ex_2_52_wave = raise Sicp_common.Pending.Pending_solution
let ex_2_52_corner_split _painter _n = raise Sicp_common.Pending.Pending_solution
let ex_2_52_square_limit _painter _n = raise Sicp_common.Pending.Pending_solution
let ex_2_52a () = raise Sicp_common.Pending.Pending_solution
