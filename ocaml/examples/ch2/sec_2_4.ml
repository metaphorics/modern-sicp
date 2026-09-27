(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme programs of SICP section 2.4 *)

module Untagged = struct
  module Rectangular = struct
    type t = float * float

    let real_part (x, _) = x
    let imag_part (_, y) = y
    let magnitude (x, y) = sqrt ((x *. x) +. (y *. y))
    let angle (x, y) = atan2 y x
    let make_from_real_imag x y : t = x, y
    let make_from_mag_ang r a : t = r *. cos a, r *. sin a
  end

  module Polar = struct
    type t = float * float

    let magnitude (r, _) = r
    let angle (_, a) = a
    let real_part (r, a) = r *. cos a
    let imag_part (r, a) = r *. sin a
    let make_from_mag_ang r a : t = r, a
    let make_from_real_imag x y : t = sqrt ((x *. x) +. (y *. y)), atan2 y x
  end

  type 'a complex_ops =
    { real_part : 'a -> float
    ; imag_part : 'a -> float
    ; magnitude : 'a -> float
    ; angle : 'a -> float
    ; make_from_real_imag : float -> float -> 'a
    ; make_from_mag_ang : float -> float -> 'a
    }

  let rectangular_ops =
    { real_part = Rectangular.real_part
    ; imag_part = Rectangular.imag_part
    ; magnitude = Rectangular.magnitude
    ; angle = Rectangular.angle
    ; make_from_real_imag = Rectangular.make_from_real_imag
    ; make_from_mag_ang = Rectangular.make_from_mag_ang
    }
  ;;

  let polar_ops =
    { real_part = Polar.real_part
    ; imag_part = Polar.imag_part
    ; magnitude = Polar.magnitude
    ; angle = Polar.angle
    ; make_from_real_imag = Polar.make_from_real_imag
    ; make_from_mag_ang = Polar.make_from_mag_ang
    }
  ;;

  let add_complex ops z1 z2 =
    ops.make_from_real_imag
      (ops.real_part z1 +. ops.real_part z2)
      (ops.imag_part z1 +. ops.imag_part z2)
  ;;

  let sub_complex ops z1 z2 =
    ops.make_from_real_imag
      (ops.real_part z1 -. ops.real_part z2)
      (ops.imag_part z1 -. ops.imag_part z2)
  ;;

  let mul_complex ops z1 z2 =
    ops.make_from_mag_ang
      (ops.magnitude z1 *. ops.magnitude z2)
      (ops.angle z1 +. ops.angle z2)
  ;;

  let div_complex ops z1 z2 =
    ops.make_from_mag_ang
      (ops.magnitude z1 /. ops.magnitude z2)
      (ops.angle z1 -. ops.angle z2)
  ;;
end

type value =
  | Num of float
  | Pair of float * float
  | Tagged of tagged

and tagged =
  { tag : string
  ; contents : value
  }

let attach_tag tag contents = { tag; contents }
let type_tag t = t.tag
let contents_of t = t.contents
let is_rectangular t = String.equal (type_tag t) "rectangular"
let is_polar t = String.equal (type_tag t) "polar"

let real_part_rectangular = function
  | Pair (x, y) -> Untagged.Rectangular.real_part (x, y)
  | Num _ | Tagged _ -> invalid_arg "real_part_rectangular: expected a pair"
;;

let imag_part_rectangular = function
  | Pair (x, y) -> Untagged.Rectangular.imag_part (x, y)
  | Num _ | Tagged _ -> invalid_arg "imag_part_rectangular: expected a pair"
;;

let magnitude_rectangular = function
  | Pair (x, y) -> Untagged.Rectangular.magnitude (x, y)
  | Num _ | Tagged _ -> invalid_arg "magnitude_rectangular: expected a pair"
;;

let angle_rectangular = function
  | Pair (x, y) -> Untagged.Rectangular.angle (x, y)
  | Num _ | Tagged _ -> invalid_arg "angle_rectangular: expected a pair"
;;

let make_from_real_imag_rectangular x y =
  let px, py = Untagged.Rectangular.make_from_real_imag x y in
  attach_tag "rectangular" (Pair (px, py))
;;

let make_from_mag_ang_rectangular r a =
  let px, py = Untagged.Rectangular.make_from_mag_ang r a in
  attach_tag "rectangular" (Pair (px, py))
;;

let real_part_polar = function
  | Pair (r, a) -> Untagged.Polar.real_part (r, a)
  | Num _ | Tagged _ -> invalid_arg "real_part_polar: expected a pair"
;;

let imag_part_polar = function
  | Pair (r, a) -> Untagged.Polar.imag_part (r, a)
  | Num _ | Tagged _ -> invalid_arg "imag_part_polar: expected a pair"
;;

let magnitude_polar = function
  | Pair (r, _) -> r
  | Num _ | Tagged _ -> invalid_arg "magnitude_polar: expected a pair"
;;

let angle_polar = function
  | Pair (_, a) -> a
  | Num _ | Tagged _ -> invalid_arg "angle_polar: expected a pair"
;;

let make_from_real_imag_polar x y =
  let pr, pa = Untagged.Polar.make_from_real_imag x y in
  attach_tag "polar" (Pair (pr, pa))
;;

let make_from_mag_ang_polar r a = attach_tag "polar" (Pair (r, a))

(* Each generic selector checks the tag of its argument and calls the
   procedure that handles data of that type. *)
let real_part z =
  if is_rectangular z
  then real_part_rectangular (contents_of z)
  else if is_polar z
  then real_part_polar (contents_of z)
  else invalid_arg ("real_part: unknown type: " ^ type_tag z)
;;

let imag_part z =
  if is_rectangular z
  then imag_part_rectangular (contents_of z)
  else if is_polar z
  then imag_part_polar (contents_of z)
  else invalid_arg ("imag_part: unknown type: " ^ type_tag z)
;;

let magnitude z =
  if is_rectangular z
  then magnitude_rectangular (contents_of z)
  else if is_polar z
  then magnitude_polar (contents_of z)
  else invalid_arg ("magnitude: unknown type: " ^ type_tag z)
;;

let angle z =
  if is_rectangular z
  then angle_rectangular (contents_of z)
  else if is_polar z
  then angle_polar (contents_of z)
  else invalid_arg ("angle: unknown type: " ^ type_tag z)
;;

(* One reasonable choice: construct rectangular numbers whenever real
   and imaginary parts are given, polar numbers whenever a magnitude
   and angle are given. This edition states the choice before
   [add_complex], which calls it; the book can state it after, because
   Scheme resolves [make-from-real-imag] at call time, not at the
   point [add-complex] is defined. *)
let make_from_real_imag x y = make_from_real_imag_rectangular x y
let make_from_mag_ang r a = make_from_mag_ang_polar r a

let add_complex z1 z2 =
  make_from_real_imag (real_part z1 +. real_part z2) (imag_part z1 +. imag_part z2)
;;

let sub_complex z1 z2 =
  make_from_real_imag (real_part z1 -. real_part z2) (imag_part z1 -. imag_part z2)
;;

let mul_complex z1 z2 =
  make_from_mag_ang (magnitude z1 *. magnitude z2) (angle z1 +. angle z2)
;;

let div_complex z1 z2 =
  make_from_mag_ang (magnitude z1 /. magnitude z2) (angle z1 -. angle z2)
;;

module Data_directed = struct
  let table : (string * string list, value list -> value) Hashtbl.t = Hashtbl.create 16
  let put op type_tags proc = Hashtbl.replace table (op, type_tags) proc
  let get op type_tags = Hashtbl.find_opt table (op, type_tags)

  let install_rectangular_package () =
    put "real_part" [ "rectangular" ] (function
      | [ Pair (x, y) ] -> Num (Untagged.Rectangular.real_part (x, y))
      | _ -> invalid_arg "real_part: rectangular expects one pair");
    put "imag_part" [ "rectangular" ] (function
      | [ Pair (x, y) ] -> Num (Untagged.Rectangular.imag_part (x, y))
      | _ -> invalid_arg "imag_part: rectangular expects one pair");
    put "magnitude" [ "rectangular" ] (function
      | [ Pair (x, y) ] -> Num (Untagged.Rectangular.magnitude (x, y))
      | _ -> invalid_arg "magnitude: rectangular expects one pair");
    put "angle" [ "rectangular" ] (function
      | [ Pair (x, y) ] -> Num (Untagged.Rectangular.angle (x, y))
      | _ -> invalid_arg "angle: rectangular expects one pair");
    put "make_from_real_imag" [ "rectangular" ] (function
      | [ Num x; Num y ] -> Tagged (attach_tag "rectangular" (Pair (x, y)))
      | _ -> invalid_arg "make_from_real_imag: rectangular expects two numbers");
    put "make_from_mag_ang" [ "rectangular" ] (function
      | [ Num r; Num a ] ->
        let x, y = Untagged.Rectangular.make_from_mag_ang r a in
        Tagged (attach_tag "rectangular" (Pair (x, y)))
      | _ -> invalid_arg "make_from_mag_ang: rectangular expects two numbers")
  ;;

  let install_polar_package () =
    put "real_part" [ "polar" ] (function
      | [ Pair (r, a) ] -> Num (Untagged.Polar.real_part (r, a))
      | _ -> invalid_arg "real_part: polar expects one pair");
    put "imag_part" [ "polar" ] (function
      | [ Pair (r, a) ] -> Num (Untagged.Polar.imag_part (r, a))
      | _ -> invalid_arg "imag_part: polar expects one pair");
    put "magnitude" [ "polar" ] (function
      | [ Pair (r, _) ] -> Num r
      | _ -> invalid_arg "magnitude: polar expects one pair");
    put "angle" [ "polar" ] (function
      | [ Pair (_, a) ] -> Num a
      | _ -> invalid_arg "angle: polar expects one pair");
    put "make_from_mag_ang" [ "polar" ] (function
      | [ Num r; Num a ] -> Tagged (attach_tag "polar" (Pair (r, a)))
      | _ -> invalid_arg "make_from_mag_ang: polar expects two numbers");
    put "make_from_real_imag" [ "polar" ] (function
      | [ Num x; Num y ] ->
        let r, a = Untagged.Polar.make_from_real_imag x y in
        Tagged (attach_tag "polar" (Pair (r, a)))
      | _ -> invalid_arg "make_from_real_imag: polar expects two numbers")
  ;;

  let apply_generic op args =
    let untag = function
      | Tagged t -> t
      | Num _ | Pair _ -> invalid_arg (op ^ ": apply_generic expects tagged arguments")
    in
    let tagged_args = List.map untag args in
    let type_tags = List.map type_tag tagged_args in
    match get op type_tags with
    | Some proc -> proc (List.map contents_of tagged_args)
    | None ->
      invalid_arg
        (Printf.sprintf
           "apply_generic: no method for these types: %s (%s)"
           op
           (String.concat ", " type_tags))
  ;;

  let as_number = function
    | Num n -> n
    | Pair _ | Tagged _ -> invalid_arg "expected a number result"
  ;;

  let as_tagged = function
    | Tagged t -> t
    | Num _ | Pair _ -> invalid_arg "expected a tagged result"
  ;;

  let real_part z = as_number (apply_generic "real_part" [ Tagged z ])
  let imag_part z = as_number (apply_generic "imag_part" [ Tagged z ])
  let magnitude z = as_number (apply_generic "magnitude" [ Tagged z ])
  let angle z = as_number (apply_generic "angle" [ Tagged z ])

  (* [make_from_real_imag] and [make_from_mag_ang] are not generic
     selectors: their arguments are plain numbers, not tagged data, so
     they look their constructor up directly instead of going through
     [apply_generic]. *)
  let make_from_real_imag x y =
    match get "make_from_real_imag" [ "rectangular" ] with
    | Some proc -> as_tagged (proc [ Num x; Num y ])
    | None -> invalid_arg "make_from_real_imag: no rectangular constructor installed"
  ;;

  let make_from_mag_ang r a =
    match get "make_from_mag_ang" [ "polar" ] with
    | Some proc -> as_tagged (proc [ Num r; Num a ])
    | None -> invalid_arg "make_from_mag_ang: no polar constructor installed"
  ;;

  let add_complex z1 z2 =
    make_from_real_imag (real_part z1 +. real_part z2) (imag_part z1 +. imag_part z2)
  ;;

  let sub_complex z1 z2 =
    make_from_real_imag (real_part z1 -. real_part z2) (imag_part z1 -. imag_part z2)
  ;;

  let mul_complex z1 z2 =
    make_from_mag_ang (magnitude z1 *. magnitude z2) (angle z1 +. angle z2)
  ;;

  let div_complex z1 z2 =
    make_from_mag_ang (magnitude z1 /. magnitude z2) (angle z1 -. angle z2)
  ;;
end

module Message_passing = struct
  type op =
    | Real_part
    | Imag_part
    | Magnitude
    | Angle

  type t = op -> float

  let make_from_real_imag x y : t = function
    | Real_part -> x
    | Imag_part -> y
    | Magnitude -> sqrt ((x *. x) +. (y *. y))
    | Angle -> atan2 y x
  ;;

  let apply_generic op (z : t) = z op
end
