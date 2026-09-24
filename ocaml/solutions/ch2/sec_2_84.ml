(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.84 *)

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

let tower_level = function
  | "integer" -> 0
  | "rational" -> 1
  | "real" -> 2
  | "complex" -> 3
  | tag -> invalid_arg ("tower_level: unknown type: " ^ tag)
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

let raise_one_level v =
  let t = untag "raise_one_level" v in
  match get "raise" [ type_tag t ] with
  | Some proc -> proc [ contents_of t ]
  | None -> invalid_arg ("raise_one_level: no supertype for " ^ type_tag t)
;;

let install_homogeneous_add () =
  put "add" [ "integer"; "integer" ] (function
    | [ Int a; Int b ] -> Tagged (attach_tag "integer" (Int (a + b)))
    | _ -> invalid_arg "add: integer expects two integers");
  put "add" [ "complex"; "complex" ] (function
    | [ Cpx (x1, y1); Cpx (x2, y2) ] ->
      Tagged (attach_tag "complex" (Cpx (x1 +. x2, y1 +. y2)))
    | _ -> invalid_arg "add: complex expects two pairs")
;;

let rec apply_generic_tower op args =
  let tagged_args = List.map (untag op) args in
  let type_tags = List.map type_tag tagged_args in
  match get op type_tags with
  | Some proc -> proc (List.map contents_of tagged_args)
  | None ->
    (match args, type_tags with
     | [ a1; a2 ], [ t1; t2 ] ->
       let l1 = tower_level t1
       and l2 = tower_level t2 in
       if l1 < l2
       then apply_generic_tower op [ raise_one_level a1; a2 ]
       else if l2 < l1
       then apply_generic_tower op [ a1; raise_one_level a2 ]
       else invalid_arg "apply_generic_tower: no method for these types"
     | _ -> invalid_arg "apply_generic_tower: no method for these types")
;;

let ex_2_84 () =
  install_raise ();
  install_homogeneous_add ();
  let three = Tagged (attach_tag "integer" (Int 3)) in
  let z = Tagged (attach_tag "complex" (Cpx (2.0, 3.0))) in
  match apply_generic_tower "add" [ three; z ] with
  | Tagged { contents = Cpx (x, y); _ } -> x, y
  | _ -> invalid_arg "ex_2_84: expected a tagged complex"
;;
