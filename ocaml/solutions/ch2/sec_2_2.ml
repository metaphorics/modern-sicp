(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program print-point in SICP section 2.1
   exercise 2.2 *)

type point = float * float
type segment = point * point

let make_point x y : point = x, y
let x_point (p : point) = fst p
let y_point (p : point) = snd p
let make_segment (a : point) (b : point) : segment = a, b
let start_segment (s : segment) = fst s
let end_segment (s : segment) = snd s

let midpoint_segment s =
  let a = start_segment s in
  let b = end_segment s in
  make_point ((x_point a +. x_point b) /. 2.0) ((y_point a +. y_point b) /. 2.0)
;;

let print_point p = Printf.printf "(%g,%g)\n" (x_point p) (y_point p)
let ex_2_02 () = midpoint_segment (make_segment (make_point 0.0 0.0) (make_point 4.0 6.0))
