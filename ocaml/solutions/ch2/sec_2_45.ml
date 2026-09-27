(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.45: the split
   combinator. *)

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

(* [right_split] and [up_split] differ only in which combiner goes on
   the outside, so the recursion is parameterized over the two. *)
let rec ex_2_45_split combine sub_combine painter n =
  if n = 0
  then painter
  else (
    let smaller = ex_2_45_split combine sub_combine painter (n - 1) in
    combine painter (sub_combine smaller smaller))
;;
