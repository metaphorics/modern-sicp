(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.83 *)

type value =
  | Int of int
  | Rat of int * int
  | Real of float
  | Cpx of float * float
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
  | Int _ | Rat _ | Real _ | Cpx _ ->
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

let install_raise () =
  put "raise" [ "integer" ] (function
    | [ Int n ] -> Tagged (attach_tag "rational" (Rat (n, 1)))
    | _ -> invalid_arg "raise: integer expects one integer");
  put "raise" [ "rational" ] (function
    | [ Rat (n, d) ] ->
      Tagged (attach_tag "real" (Real (float_of_int n /. float_of_int d)))
    | _ -> invalid_arg "raise: rational expects one pair");
  put "raise" [ "real" ] (function
    | [ Real x ] -> Tagged (attach_tag "complex" (Cpx (x, 0.0)))
    | _ -> invalid_arg "raise: real expects one number")
;;

let raise_one_level v = apply_generic "raise" [ v ]

let ex_2_83 () =
  install_raise ();
  let at_integer = Tagged (attach_tag "integer" (Int 3)) in
  let at_rational = raise_one_level at_integer in
  let at_real = raise_one_level at_rational in
  let at_complex = raise_one_level at_real in
  let final =
    match at_complex with
    | Tagged { contents = Cpx (x, y); _ } -> x, y
    | _ -> invalid_arg "ex_2_83: expected a tagged complex"
  in
  ( type_tag (untag "ex_2_83" at_rational)
  , type_tag (untag "ex_2_83" at_real)
  , type_tag (untag "ex_2_83" at_complex)
  , final )
;;
