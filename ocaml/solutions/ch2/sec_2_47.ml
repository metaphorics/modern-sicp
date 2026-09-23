(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.47: two frame
   constructors. *)

type vect =
  { x : float
  ; y : float
  }

type frame1 = vect * vect * vect
type frame2 = vect * (vect * vect)

let make_frame1 origin edge1 edge2 = origin, edge1, edge2
let origin_frame1 (origin, _edge1, _edge2) = origin
let edge1_frame1 (_origin, edge1, _edge2) = edge1
let edge2_frame1 (_origin, _edge1, edge2) = edge2
let make_frame2 origin edge1 edge2 = origin, (edge1, edge2)
let origin_frame2 (origin, _edges) = origin
let edge1_frame2 (_origin, (edge1, _edge2)) = edge1
let edge2_frame2 (_origin, (_edge1, edge2)) = edge2

(* Both representations built from the same three corners of the unit
   square, so the selector pairs can be checked against each other:
   the constructor choice is invisible to everything that builds on
   the selectors. *)
let ex_2_47 () =
  let o = { x = 0.0; y = 0.0 } in
  let e1 = { x = 1.0; y = 0.0 } in
  let e2 = { x = 0.0; y = 1.0 } in
  make_frame1 o e1 e2, make_frame2 o e1 e2
;;
