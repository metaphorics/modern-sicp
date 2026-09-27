(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.44: up-split. *)

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

let below painter1 painter2 =
  let split_point = make_vect 0.0 0.5 in
  let paint_bottom =
    transform_painter painter1 (make_vect 0.0 0.0) (make_vect 1.0 0.0) split_point
  in
  let paint_top =
    transform_painter painter2 split_point (make_vect 1.0 0.5) (make_vect 0.0 1.0)
  in
  fun frame -> paint_bottom frame @ paint_top frame
;;

(* The mirror image of [right_split]: roles of [beside] and [below]
   switched. *)
let rec ex_2_44_up_split painter n =
  if n = 0
  then painter
  else (
    let smaller = ex_2_44_up_split painter (n - 1) in
    below painter (beside smaller smaller))
;;
