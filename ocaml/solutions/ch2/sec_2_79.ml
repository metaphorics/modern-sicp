(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.79 *)

type value =
  | Num of float
  | Ratpair of int * int
  | Cpx of float * float
  | Bool of bool
  | Tagged of tagged

and tagged =
  { tag : string
  ; contents : value
  }

let attach_tag tag contents = { tag; contents }
let type_tag t = t.tag
let contents_of t = t.contents
let table : (string * string list, value list -> value) Hashtbl.t = Hashtbl.create 8
let put op type_tags proc = Hashtbl.replace table (op, type_tags) proc
let get op type_tags = Hashtbl.find_opt table (op, type_tags)

let untag op = function
  | Tagged t -> t
  | Num _ | Ratpair _ | Cpx _ | Bool _ ->
    invalid_arg (op ^ ": apply_generic expects tagged arguments")
;;

let apply_generic op args =
  let tagged_args = List.map (untag op) args in
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

let install_scheme_number_package () =
  put "equ?" [ "scheme-number"; "scheme-number" ] (function
    | [ Num a; Num b ] -> Bool (Float.equal a b)
    | _ -> invalid_arg "equ?: scheme-number expects two numbers")
;;

let install_rational_package () =
  put "equ?" [ "rational"; "rational" ] (function
    | [ Ratpair (n1, d1); Ratpair (n2, d2) ] -> Bool (n1 * d2 = n2 * d1)
    | _ -> invalid_arg "equ?: rational expects two pairs")
;;

let install_complex_package () =
  put "equ?" [ "complex"; "complex" ] (function
    | [ Cpx (x1, y1); Cpx (x2, y2) ] -> Bool (Float.equal x1 x2 && Float.equal y1 y2)
    | _ -> invalid_arg "equ?: complex expects two pairs")
;;

let equ x y =
  match apply_generic "equ?" [ x; y ] with
  | Bool b -> b
  | Num _ | Ratpair _ | Cpx _ | Tagged _ -> invalid_arg "equ?: expected a boolean result"
;;

let ex_2_79 () =
  install_scheme_number_package ();
  install_rational_package ();
  install_complex_package ();
  let sn n = Tagged (attach_tag "scheme-number" (Num n)) in
  let rat n d = Tagged (attach_tag "rational" (Ratpair (n, d))) in
  let cpx x y = Tagged (attach_tag "complex" (Cpx (x, y))) in
  ( equ (sn 3.0) (sn 3.0)
  , equ (sn 3.0) (sn 4.0)
  , equ (rat 1 2) (rat 2 4)
  , equ (cpx 3.0 4.0) (cpx 3.0 4.0)
  , equ (cpx 3.0 4.0) (cpx 3.0 5.0) )
;;
