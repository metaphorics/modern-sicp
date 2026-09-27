(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.45:
   the split combinator *)

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

(** [ex_2_45_split combine sub_combine painter n] is the recursive
    splitter that places [painter] beside or below the twice-doubled
    smaller split, as [combine] and [sub_combine] direct. *)
val ex_2_45_split
  :  (painter -> painter -> painter)
  -> (painter -> painter -> painter)
  -> painter
  -> int
  -> painter
