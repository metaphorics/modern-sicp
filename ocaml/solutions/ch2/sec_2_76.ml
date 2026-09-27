(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.4 exercise 2.76 *)

(** Exercise 2.76: this edition reframes "generic operations with
    explicit dispatch, data-directed style, and message-passing
    style" as OCaml's own three ways to organize the same two
    representations and two operations. Each style below answers
    [real_part] and [magnitude] for a rectangular (3, 4) and a polar
    (5, 0); [ex_2_76] checks all three agree. The written comparison
    -- which style suits adding a type, which suits adding an
    operation -- is [ex_2_76.md]'s job, not this module's. *)

type explicit_z =
  | Rectangular of float * float
  | Polar of float * float

let real_part_explicit = function
  | Rectangular (x, _) -> x
  | Polar (r, a) -> r *. cos a
;;

let magnitude_explicit = function
  | Rectangular (x, y) -> sqrt ((x *. x) +. (y *. y))
  | Polar (r, _) -> r
;;

type dd_value =
  | Num of float
  | Pair of float * float

type dd_tagged =
  { tag : string
  ; contents : dd_value
  }

type dd_table = (string * string, dd_value -> dd_value) Hashtbl.t

let dd_make_table () : dd_table = Hashtbl.create 8

let dd_install_rectangular table =
  Hashtbl.replace table ("real_part", "rectangular") (function
    | Pair (x, _) -> Num x
    | Num _ -> invalid_arg "real_part: rectangular expects a pair");
  Hashtbl.replace table ("magnitude", "rectangular") (function
    | Pair (x, y) -> Num (sqrt ((x *. x) +. (y *. y)))
    | Num _ -> invalid_arg "magnitude: rectangular expects a pair")
;;

let dd_install_polar table =
  Hashtbl.replace table ("real_part", "polar") (function
    | Pair (r, a) -> Num (r *. cos a)
    | Num _ -> invalid_arg "real_part: polar expects a pair");
  Hashtbl.replace table ("magnitude", "polar") (function
    | Pair (r, _) -> Num r
    | Num _ -> invalid_arg "magnitude: polar expects a pair")
;;

let dd_apply_generic table op tagged =
  match Hashtbl.find_opt table (op, tagged.tag) with
  | Some proc ->
    (match proc tagged.contents with
     | Num n -> n
     | Pair _ -> invalid_arg (op ^ ": expected a number result"))
  | None -> invalid_arg (Printf.sprintf "%s: no method for %s" op tagged.tag)
;;

type mp_op =
  | Real_part
  | Magnitude

type mp_z = mp_op -> float

let mp_rectangular x y = function
  | Real_part -> x
  | Magnitude -> sqrt ((x *. x) +. (y *. y))
;;

let mp_polar r a = function
  | Real_part -> r *. cos a
  | Magnitude -> r
;;

type sample = float * float

let ex_2_76 () =
  let explicit_results =
    ( ( real_part_explicit (Rectangular (3.0, 4.0))
      , magnitude_explicit (Rectangular (3.0, 4.0)) )
    , (real_part_explicit (Polar (5.0, 0.0)), magnitude_explicit (Polar (5.0, 0.0))) )
  in
  let table = dd_make_table () in
  dd_install_rectangular table;
  dd_install_polar table;
  let dd_rect = { tag = "rectangular"; contents = Pair (3.0, 4.0) } in
  let dd_polar = { tag = "polar"; contents = Pair (5.0, 0.0) } in
  let dd_results =
    ( ( dd_apply_generic table "real_part" dd_rect
      , dd_apply_generic table "magnitude" dd_rect )
    , ( dd_apply_generic table "real_part" dd_polar
      , dd_apply_generic table "magnitude" dd_polar ) )
  in
  let mp_rect = mp_rectangular 3.0 4.0 in
  let mp_pol = mp_polar 5.0 0.0 in
  let mp_results =
    (mp_rect Real_part, mp_rect Magnitude), (mp_pol Real_part, mp_pol Magnitude)
  in
  explicit_results, dd_results, mp_results
;;
