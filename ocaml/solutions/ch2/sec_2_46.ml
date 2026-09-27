(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.46: the vector
   abstraction. *)

type vect =
  { x : float
  ; y : float
  }

let make_vect x y = { x; y }
let xcor_vect v = v.x
let ycor_vect v = v.y
let add_vect a b = { x = a.x +. b.x; y = a.y +. b.y }
let sub_vect a b = { x = a.x -. b.x; y = a.y -. b.y }
let scale_vect s v = { x = s *. v.x; y = s *. v.y }

(* The statement's three operations on one fixed set of inputs, for the
   tests to check against. *)
let ex_2_46 () =
  ( add_vect (make_vect 1.0 2.0) (make_vect 3.0 4.0)
  , sub_vect (make_vect 3.0 4.0) (make_vect 1.0 2.0)
  , scale_vect 2.0 (make_vect 1.0 2.0) )
;;
