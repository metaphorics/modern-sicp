(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.49:
   the four primitive painters *)

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

val ex_2_49_outline : painter
val ex_2_49_x : painter
val ex_2_49_diamond : painter
val ex_2_49_wave : painter
