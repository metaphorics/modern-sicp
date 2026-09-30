(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.85 *)

type value =
  | Int of int
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
  | Int _ | Real _ | Cpx _ -> invalid_arg (op ^ ": apply_generic expects tagged arguments")
;;

let tower_level = function
  | "integer" -> 0
  | "real" -> 1
  | "complex" -> 2
  | tag -> invalid_arg ("tower_level: unknown type: " ^ tag)
;;

let install_raise_and_project () =
  put "raise" [ "integer" ] (function
    | [ Int n ] -> Tagged (attach_tag "real" (Real (float_of_int n)))
    | _ -> invalid_arg "raise: integer expects one integer");
  put "raise" [ "real" ] (function
    | [ Real x ] -> Tagged (attach_tag "complex" (Cpx (x, 0.0)))
    | _ -> invalid_arg "raise: real expects one number");
  put "project" [ "real" ] (function
    | [ Real x ] -> Tagged (attach_tag "integer" (Int (int_of_float (Float.round x))))
    | _ -> invalid_arg "project: real expects one number");
  put "project" [ "complex" ] (function
    | [ Cpx (x, _) ] -> Tagged (attach_tag "real" (Real x))
    | _ -> invalid_arg "project: complex expects one pair")
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

let raise_one_level v =
  let t = untag "raise_one_level" v in
  match get "raise" [ type_tag t ] with
  | Some proc -> proc [ contents_of t ]
  | None -> invalid_arg ("raise_one_level: no supertype for " ^ type_tag t)
;;

let rec apply_generic op args =
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
       then apply_generic op [ raise_one_level a1; a2 ]
       else if l2 < l1
       then apply_generic op [ a1; raise_one_level a2 ]
       else invalid_arg "apply_generic: no method for these types"
     | _ -> invalid_arg "apply_generic: no method for these types")
;;

let value_equal a b =
  match a, b with
  | Int x, Int y -> x = y
  | Real x, Real y -> Float.equal x y
  | Cpx (x1, y1), Cpx (x2, y2) -> Float.equal x1 x2 && Float.equal y1 y2
  | (Int _ | Real _ | Cpx _), (Int _ | Real _ | Cpx _) -> false
  | Tagged _, _ | _, Tagged _ -> invalid_arg "value_equal: expected untagged contents"
;;

let rec drop v =
  let t = untag "drop" v in
  match get "project" [ type_tag t ] with
  | None -> v
  | Some proj ->
    let projected = proj [ contents_of t ] in
    let raised_back = raise_one_level projected in
    let raised_back_contents = contents_of (untag "drop" raised_back) in
    if value_equal raised_back_contents (contents_of t) then drop projected else v
;;

let apply_generic_drop op args = drop (apply_generic op args)

let ex_2_85 () =
  install_raise_and_project ();
  install_homogeneous_add ();
  let cpx x y = Tagged (attach_tag "complex" (Cpx (x, y))) in
  let tag_of v = type_tag (untag "ex_2_85" v) in
  let sum = apply_generic_drop "add" [ cpx 2.0 3.0; cpx (-2.0) (-3.0) ] in
  ( tag_of (drop (cpx 1.5 0.0))
  , tag_of (drop (cpx 1.0 0.0))
  , tag_of (drop (cpx 2.0 3.0))
  , tag_of sum )
;;

(* Addition by this edition, extending SICP section 2.5 exercise 2.85 *)

type edge = string * string

type color =
  | White
  | Gray
  | Black

let has_cycle edges =
  let nodes =
    List.sort_uniq String.compare (List.concat_map (fun (a, b) -> [ a; b ]) edges)
  in
  let colors : (string, color) Hashtbl.t = Hashtbl.create 8 in
  List.iter (fun n -> Hashtbl.replace colors n White) nodes;
  let successors n =
    List.filter_map (fun (a, b) -> if String.equal a n then Some b else None) edges
  in
  let rec dfs n =
    match Hashtbl.find colors n with
    | Gray -> true
    | Black -> false
    | White ->
      Hashtbl.replace colors n Gray;
      let found = List.exists dfs (successors n) in
      Hashtbl.replace colors n Black;
      found
  in
  List.exists dfs nodes
;;

let ex_2_85a () =
  let tower_edges = [ "integer", "rational"; "rational", "real"; "real", "complex" ] in
  let self_coercion_edges = [ "real", "real" ] in
  let three_step_edges = [ "a", "b"; "b", "c"; "c", "a" ] in
  has_cycle tower_edges, has_cycle self_coercion_edges, has_cycle three_step_edges
;;
