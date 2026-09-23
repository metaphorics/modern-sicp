(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise
   SICP section 2.1 exercise 2.3 *)

type point = float * float

module type Rectangle = sig
  type t

  val width : t -> float
  val height : t -> float
end

module Perimeter_area (R : Rectangle) = struct
  let perimeter r = 2.0 *. (R.width r +. R.height r)
  let area r = R.width r *. R.height r
end

module Two_corners : sig
  type t

  val make : point -> point -> t
  val width : t -> float
  val height : t -> float
end = struct
  type t = point * point

  let make a b = a, b
  let width ((x1, _), (x2, _)) = Float.abs (x2 -. x1)
  let height ((_, y1), (_, y2)) = Float.abs (y2 -. y1)
end

module Corner_and_dims : sig
  type t

  val make : point -> float -> float -> t
  val width : t -> float
  val height : t -> float
end = struct
  type t = point * float * float

  let make corner w h = corner, w, h
  let width (_, w, _) = w
  let height (_, _, h) = h
end

module Two_corners_measure = Perimeter_area (Two_corners)
module Corner_and_dims_measure = Perimeter_area (Corner_and_dims)

let ex_2_03 () =
  let r1 = Two_corners.make (0.0, 0.0) (3.0, 4.0) in
  let r2 = Corner_and_dims.make (0.0, 0.0) 3.0 4.0 in
  ( (Two_corners_measure.perimeter r1, Two_corners_measure.area r1)
  , (Corner_and_dims_measure.perimeter r2, Corner_and_dims_measure.area r2) )
;;
