(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.82 *)

type value =
  | Num of float
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
let coercion_table : (string * string, value -> value) Hashtbl.t = Hashtbl.create 8
let put_coercion t1 t2 proc = Hashtbl.replace coercion_table (t1, t2) proc
let get_coercion t1 t2 = Hashtbl.find_opt coercion_table (t1, t2)

let untag op = function
  | Tagged t -> t
  | Num _ -> invalid_arg (op ^ ": apply_generic expects tagged arguments")
;;

let coerce_all_to target args =
  let target_tag = type_tag (untag "coerce_all_to" target) in
  let rec loop acc = function
    | [] -> Some (List.rev acc)
    | a :: rest ->
      let a_tag = type_tag (untag "coerce_all_to" a) in
      if String.equal a_tag target_tag
      then loop (a :: acc) rest
      else (
        match get_coercion a_tag target_tag with
        | Some f -> loop (f a :: acc) rest
        | None -> None)
  in
  loop [] args
;;

let apply_generic_n op args =
  let tagged_args = List.map (untag op) args in
  let type_tags = List.map type_tag tagged_args in
  match get op type_tags with
  | Some proc -> proc (List.map contents_of tagged_args)
  | None ->
    let rec try_candidates = function
      | [] -> invalid_arg "apply_generic_n: no method for these types"
      | candidate :: rest ->
        (match coerce_all_to candidate args with
         | Some coerced ->
           let coerced_tagged = List.map (untag op) coerced in
           let coerced_tags = List.map type_tag coerced_tagged in
           (match get op coerced_tags with
            | Some proc -> proc (List.map contents_of coerced_tagged)
            | None -> try_candidates rest)
         | None -> try_candidates rest)
    in
    try_candidates args
;;

let ex_2_82_a () =
  put "add3" [ "real"; "real"; "real" ] (function
    | [ Num a; Num b; Num c ] -> Tagged (attach_tag "real" (Num (a +. b +. c)))
    | _ -> invalid_arg "add3: expects three numbers");
  put_coercion "rational" "real" (function
    | Tagged t -> Tagged (attach_tag "real" t.contents)
    | Num _ -> invalid_arg "rational->real: expected a tagged value");
  let r = Tagged (attach_tag "rational" (Num 1.0)) in
  let two = Tagged (attach_tag "real" (Num 2.0)) in
  let three = Tagged (attach_tag "real" (Num 3.0)) in
  apply_generic_n "add3" [ r; two; three ]
;;

let ex_2_82_b () =
  put_coercion "a" "b" (function
    | Tagged t -> Tagged (attach_tag "b" t.contents)
    | Num _ -> invalid_arg "a->b: expected a tagged value");
  put_coercion "b" "c" (function
    | Tagged t -> Tagged (attach_tag "c" t.contents)
    | Num _ -> invalid_arg "b->c: expected a tagged value");
  put "mix2" [ "b"; "c" ] (function
    | [ Num x; Num y ] -> Tagged (attach_tag "b" (Num (x +. y)))
    | _ -> invalid_arg "mix2: expects two numbers");
  let x_a = Tagged (attach_tag "a" (Num 1.0)) in
  let x_c = Tagged (attach_tag "c" (Num 2.0)) in
  try
    ignore (apply_generic_n "mix2" [ x_a; x_c ]);
    false
  with
  | Invalid_argument _ -> true
;;
