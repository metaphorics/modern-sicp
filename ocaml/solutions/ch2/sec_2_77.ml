(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.77 *)

type value =
  | Num of float
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
let counter = ref 0
let call_count () = !counter
let reset_call_count () = counter := 0

let untag op = function
  | Tagged t -> t
  | Num _ | Cpx _ -> invalid_arg (op ^ ": apply_generic expects tagged arguments")
;;

let apply_generic op args =
  incr counter;
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

let install_rectangular_package () =
  put "real_part" [ "rectangular" ] (function
    | [ Cpx (x, _) ] -> Num x
    | _ -> invalid_arg "real_part: rectangular expects one pair");
  put "imag_part" [ "rectangular" ] (function
    | [ Cpx (_, y) ] -> Num y
    | _ -> invalid_arg "imag_part: rectangular expects one pair");
  put "magnitude" [ "rectangular" ] (function
    | [ Cpx (x, y) ] -> Num (sqrt ((x *. x) +. (y *. y)))
    | _ -> invalid_arg "magnitude: rectangular expects one pair");
  put "angle" [ "rectangular" ] (function
    | [ Cpx (x, y) ] -> Num (atan2 y x)
    | _ -> invalid_arg "angle: rectangular expects one pair")
;;

let as_number = function
  | Num n -> n
  | Cpx _ | Tagged _ -> invalid_arg "expected a number result"
;;

let real_part z = as_number (apply_generic "real_part" [ z ])
let imag_part z = as_number (apply_generic "imag_part" [ z ])
let magnitude z = as_number (apply_generic "magnitude" [ z ])
let angle z = as_number (apply_generic "angle" [ z ])

let install_alyssa_complex_fix () =
  put "real_part" [ "complex" ] (function
    | [ z ] -> Num (real_part z)
    | _ -> invalid_arg "real_part: complex expects one value");
  put "imag_part" [ "complex" ] (function
    | [ z ] -> Num (imag_part z)
    | _ -> invalid_arg "imag_part: complex expects one value");
  put "magnitude" [ "complex" ] (function
    | [ z ] -> Num (magnitude z)
    | _ -> invalid_arg "magnitude: complex expects one value");
  put "angle" [ "complex" ] (function
    | [ z ] -> Num (angle z)
    | _ -> invalid_arg "angle: complex expects one value")
;;

let ex_2_77 () =
  install_rectangular_package ();
  let z =
    Tagged (attach_tag "complex" (Tagged (attach_tag "rectangular" (Cpx (3.0, 4.0)))))
  in
  reset_call_count ();
  let raised_before_fix =
    try
      ignore (magnitude z);
      false
    with
    | Invalid_argument _ -> true
  in
  install_alyssa_complex_fix ();
  reset_call_count ();
  let result = magnitude z in
  raised_before_fix, result, call_count ()
;;
