(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.80 *)

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
  put "=zero?" [ "scheme-number" ] (function
    | [ Num a ] -> Bool (Float.equal a 0.0)
    | _ -> invalid_arg "=zero?: scheme-number expects one number")
;;

let install_rational_package () =
  put "=zero?" [ "rational" ] (function
    | [ Ratpair (n, _) ] -> Bool (n = 0)
    | _ -> invalid_arg "=zero?: rational expects one pair")
;;

let install_complex_package () =
  put "=zero?" [ "complex" ] (function
    | [ Cpx (x, y) ] -> Bool (Float.equal x 0.0 && Float.equal y 0.0)
    | _ -> invalid_arg "=zero?: complex expects one pair")
;;

let is_zero v =
  match apply_generic "=zero?" [ v ] with
  | Bool b -> b
  | Num _ | Ratpair _ | Cpx _ | Tagged _ ->
    invalid_arg "=zero?: expected a boolean result"
;;

let ex_2_80 () =
  install_scheme_number_package ();
  install_rational_package ();
  install_complex_package ();
  let sn n = Tagged (attach_tag "scheme-number" (Num n)) in
  let rat n d = Tagged (attach_tag "rational" (Ratpair (n, d))) in
  let cpx x y = Tagged (attach_tag "complex" (Cpx (x, y))) in
  ( is_zero (sn 0.0)
  , is_zero (sn 3.0)
  , is_zero (rat 0 5)
  , is_zero (rat 1 5)
  , is_zero (cpx 0.0 0.0)
  , is_zero (cpx 0.0 1.0) )
;;
