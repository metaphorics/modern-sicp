(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.49: the four
   primitive painters. *)

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
let scale_vect s v = { x = s *. v.x; y = s *. v.y }
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

let segments_to_painter segment_list =
  fun frame ->
  let m = frame_coord_map frame in
  List.map (fun (start_v, end_v) -> m start_v, m end_v) segment_list
;;

(* (a) the frame outline; (b) the ``X''; (c) the diamond of side
   midpoints. *)
let ex_2_49_outline =
  segments_to_painter
    [ make_vect 0.0 0.0, make_vect 1.0 0.0
    ; make_vect 1.0 0.0, make_vect 1.0 1.0
    ; make_vect 1.0 1.0, make_vect 0.0 1.0
    ; make_vect 0.0 1.0, make_vect 0.0 0.0
    ]
;;

let ex_2_49_x =
  segments_to_painter
    [ make_vect 0.0 0.0, make_vect 1.0 1.0; make_vect 1.0 0.0, make_vect 0.0 1.0 ]
;;

let ex_2_49_diamond =
  segments_to_painter
    [ make_vect 0.5 0.0, make_vect 1.0 0.5
    ; make_vect 1.0 0.5, make_vect 0.5 1.0
    ; make_vect 0.5 1.0, make_vect 0.0 0.5
    ; make_vect 0.0 0.5, make_vect 0.5 0.0
    ]
;;

(* (d) a crude wave: head, front and back legs, tail, and back. *)
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

let ex_2_49_wave = segments_to_painter wave_segments
