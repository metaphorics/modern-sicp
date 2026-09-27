(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.51: below, two
   ways. *)

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

let beside painter1 painter2 =
  let split_point = make_vect 0.5 0.0 in
  let paint_left =
    transform_painter painter1 (make_vect 0.0 0.0) split_point (make_vect 0.0 1.0)
  in
  let paint_right =
    transform_painter painter2 split_point (make_vect 1.0 0.0) (make_vect 0.5 1.0)
  in
  fun frame -> paint_left frame @ paint_right frame
;;

(* The direct construction, analogous to [beside]: bottom half and top
   half instead of left half and right half. *)
let ex_2_51_below painter1 painter2 =
  let split_point = make_vect 0.0 0.5 in
  let paint_bottom =
    transform_painter painter1 (make_vect 0.0 0.0) (make_vect 1.0 0.0) split_point
  in
  let paint_top =
    transform_painter painter2 split_point (make_vect 1.0 0.5) (make_vect 0.0 1.0)
  in
  fun frame -> paint_bottom frame @ paint_top frame
;;

let rotate_90 painter =
  transform_painter painter (make_vect 1.0 0.0) (make_vect 1.0 1.0) (make_vect 0.0 0.0)
;;

let rotate_270 painter =
  transform_painter painter (make_vect 0.0 1.0) (make_vect 0.0 0.0) (make_vect 1.0 1.0)
;;

(* The construction through rotations: a [beside] of the two 270-degree
   rotations, itself rotated by 90 degrees, carries the left half to
   the bottom and the right half to the top, while each 270-degree
   rotation composes with the 90-degree rotation to identity. *)
let ex_2_51_below_rotate painter1 painter2 =
  rotate_90 (beside (rotate_270 painter1) (rotate_270 painter2))
;;
