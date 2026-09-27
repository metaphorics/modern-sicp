(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The finished state after the exercise's second half: [Triangle] was
   added, and the compiler flagged the one [area] as the site that broke.
   The scaffold holds the two-constructor starting point. *)

type shape =
  | Circle of float
  | Rectangle of float * float
  | Triangle of float * float

let area = function
  | Circle r -> Float.pi *. r *. r
  | Rectangle (w, h) -> w *. h
  | Triangle (base, height) -> 0.5 *. base *. height
;;
