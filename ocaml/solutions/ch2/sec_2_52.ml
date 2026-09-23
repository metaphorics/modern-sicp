(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.52: square limit
   variations, and the added 2.52a. *)

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
let unit_frame = make_frame (make_vect 0.0 0.0) (make_vect 1.0 0.0) (make_vect 0.0 1.0)

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

let flip_vert painter =
  transform_painter painter (make_vect 0.0 1.0) (make_vect 1.0 1.0) (make_vect 0.0 0.0)
;;

let flip_horiz painter =
  transform_painter painter (make_vect 1.0 0.0) (make_vect 0.0 0.0) (make_vect 1.0 1.0)
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

let segments_to_painter segment_list =
  fun frame ->
  let m = frame_coord_map frame in
  List.map (fun (start_v, end_v) -> m start_v, m end_v) segment_list
;;

(* The section's wave painter, the base of the variations. *)
let wave_segments : segment list =
  [ make_vect 0.10 0.45, make_vect 0.30 0.50
  ; make_vect 0.30 0.50, make_vect 0.35 0.40
  ; make_vect 0.35 0.40, make_vect 0.40 0.30
  ; make_vect 0.40 0.30, make_vect 0.35 0.15
  ; make_vect 0.35 0.15, make_vect 0.40 0.00
  ; make_vect 0.60 0.00, make_vect 0.65 0.15
  ; make_vect 0.65 0.15, make_vect 0.60 0.30
  ; make_vect 0.60 0.30, make_vect 0.65 0.40
  ; make_vect 0.65 0.40, make_vect 0.85 0.35
  ; make_vect 0.85 0.35, make_vect 0.90 0.45
  ; make_vect 0.90 0.45, make_vect 0.70 0.55
  ; make_vect 0.70 0.55, make_vect 0.65 0.65
  ; make_vect 0.65 0.65, make_vect 0.60 0.75
  ; make_vect 0.60 0.75, make_vect 0.40 0.75
  ; make_vect 0.40 0.75, make_vect 0.35 0.65
  ; make_vect 0.35 0.65, make_vect 0.30 0.55
  ; make_vect 0.30 0.55, make_vect 0.10 0.45
  ; make_vect 0.45 0.60, make_vect 0.48 0.55
  ; make_vect 0.48 0.55, make_vect 0.55 0.55
  ; make_vect 0.55 0.55, make_vect 0.58 0.60
  ]
;;

let wave = segments_to_painter wave_segments

(* (a) lowest level: add a smile to the primitive painter. *)
let ex_2_52_wave =
  segments_to_painter
    (wave_segments
     @ [ make_vect 0.38 0.48, make_vect 0.43 0.44
       ; make_vect 0.43 0.44, make_vect 0.57 0.44
       ; make_vect 0.57 0.44, make_vect 0.62 0.48
       ])
;;

let rec up_split painter n =
  if n = 0
  then painter
  else (
    let smaller = up_split painter (n - 1) in
    below painter (beside smaller smaller))
;;

let rec right_split painter n =
  if n = 0
  then painter
  else (
    let smaller = right_split painter (n - 1) in
    beside painter (below smaller smaller))
;;

(* (b) middle level: one copy of each split image per corner instead
   of two. *)
let rec ex_2_52_corner_split painter n =
  if n = 0
  then painter
  else (
    let up = up_split painter (n - 1) in
    let right = right_split painter (n - 1) in
    beside (below painter up) (below right (ex_2_52_corner_split painter (n - 1))))
;;

(* (c) highest level: assemble the corners in a changed pattern. *)
let ex_2_52_square_limit painter n =
  let quarter = ex_2_52_corner_split painter n in
  let half = beside quarter (flip_horiz quarter) in
  below half (flip_vert half)
;;

module Svg = struct
  let line (a, b) =
    Printf.sprintf
      {|<line x1="%g" y1="%g" x2="%g" y2="%g"/>|}
      (xcor_vect a)
      (ycor_vect a)
      (xcor_vect b)
      (ycor_vect b)
  ;;

  let document ~view_box ~size body =
    Printf.sprintf
      {|<svg xmlns="http://www.w3.org/2000/svg" width="%d" height="%d" viewBox="%s"><g stroke="black" stroke-width="0.008" stroke-linecap="round" fill="none">%s</g></svg>|}
      size
      size
      view_box
      body
  ;;
end

(* Exercise 2.52a: the view-box transform. The view box is an explicit
   argument, so the same painted segments can be shown through any
   window of the plane that the caller names. *)
let render_view_box ~view_box ~size frame painter =
  let segments = painter frame in
  Svg.document ~view_box ~size (String.concat "" (List.map Svg.line segments))
;;

let ex_2_52a () =
  render_view_box
    ~view_box:"-0.1 -0.1 1.2 1.2"
    ~size:400
    unit_frame
    (ex_2_52_square_limit wave 4)
;;
