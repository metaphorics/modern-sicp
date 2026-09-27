(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.50: flip-horiz and
   the rotations. *)

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

let make_vect x y = { x; y }
let xcor_vect v = v.x
let ycor_vect v = v.y
let add_vect a b = { x = a.x +. b.x; y = a.y +. b.y }
let sub_vect a b = { x = a.x -. b.x; y = a.y -. b.y }
let scale_vect s v = { x = s *. v.x; y = s *. v.y }
let make_frame origin edge1 edge2 = { origin; edge1; edge2 }
let origin_frame f = f.origin
let edge1_frame f = f.edge1
let edge2_frame f = f.edge2

let frame_coord_map frame =
  fun v ->
  add_vect
    (origin_frame frame)
    (add_vect
       (scale_vect (xcor_vect v) (edge1_frame frame))
       (scale_vect (ycor_vect v) (edge2_frame frame)))
;;

let transform_painter painter origin corner1 corner2 =
  fun frame ->
  let m = frame_coord_map frame in
  let new_origin = m origin in
  painter
    (make_frame
       new_origin
       (sub_vect (m corner1) new_origin)
       (sub_vect (m corner2) new_origin))
;;

(* Each transformation only has to describe the derived frame: origin,
   where edge1 ends, and where edge2 ends. *)
let ex_2_50_flip_horiz painter =
  transform_painter painter (make_vect 1.0 0.0) (make_vect 0.0 0.0) (make_vect 1.0 1.0)
;;

let ex_2_50_rotate_180 painter =
  transform_painter painter (make_vect 1.0 1.0) (make_vect 0.0 1.0) (make_vect 1.0 0.0)
;;

let ex_2_50_rotate_270 painter =
  transform_painter painter (make_vect 0.0 1.0) (make_vect 0.0 0.0) (make_vect 1.0 1.0)
;;
