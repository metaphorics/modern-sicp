(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.81 *)

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
let coercion_table : (string * string, value -> value) Hashtbl.t = Hashtbl.create 4
let put_coercion t1 t2 proc = Hashtbl.replace coercion_table (t1, t2) proc
let get_coercion t1 t2 = Hashtbl.find_opt coercion_table (t1, t2)

let untag op = function
  | Tagged t -> t
  | Num _ -> invalid_arg (op ^ ": apply_generic expects tagged arguments")
;;

exception Loop_detected of int

let max_depth = 1000

let rec apply_generic_loop depth op args =
  if depth > max_depth then raise (Loop_detected depth);
  let tagged_args = List.map (untag op) args in
  let type_tags = List.map type_tag tagged_args in
  match get op type_tags with
  | Some proc -> proc (List.map contents_of tagged_args)
  | None ->
    (match args, type_tags with
     | [ a1; a2 ], [ t1; t2 ] ->
       (match get_coercion t1 t2, get_coercion t2 t1 with
        | Some t1_to_t2, _ -> apply_generic_loop (depth + 1) op [ t1_to_t2 a1; a2 ]
        | None, Some t2_to_t1 -> apply_generic_loop (depth + 1) op [ a1; t2_to_t1 a2 ]
        | None, None -> invalid_arg "apply_generic_loop: no method for these types")
     | _ -> invalid_arg "apply_generic_loop: no method for these types")
;;

let rec apply_generic_fixed op args =
  let tagged_args = List.map (untag op) args in
  let type_tags = List.map type_tag tagged_args in
  match get op type_tags with
  | Some proc -> proc (List.map contents_of tagged_args)
  | None ->
    (match args, type_tags with
     | [ a1; a2 ], [ t1; t2 ] when not (String.equal t1 t2) ->
       (match get_coercion t1 t2, get_coercion t2 t1 with
        | Some t1_to_t2, _ -> apply_generic_fixed op [ t1_to_t2 a1; a2 ]
        | None, Some t2_to_t1 -> apply_generic_fixed op [ a1; t2_to_t1 a2 ]
        | None, None -> invalid_arg "apply_generic_fixed: no method for these types")
     | _ -> invalid_arg "apply_generic_fixed: no method for these types")
;;

let install_louis_setup () =
  put_coercion "real" "real" (fun n -> n);
  put_coercion "complex" "complex" (fun z -> z);
  put "exp" [ "real"; "real" ] (function
    | [ Num x; Num y ] -> Tagged (attach_tag "real" (Num (x ** y)))
    | _ -> invalid_arg "exp: real expects two numbers")
;;

let complex_sample = Tagged (attach_tag "complex" (Num 0.0))

let ex_2_81_a () =
  install_louis_setup ();
  try
    ignore (apply_generic_loop 0 "exp" [ complex_sample; complex_sample ]);
    invalid_arg "ex_2_81_a: apply_generic_loop unexpectedly terminated"
  with
  | Loop_detected depth -> depth
;;

let ex_2_81_c () =
  install_louis_setup ();
  try
    ignore (apply_generic_fixed "exp" [ complex_sample; complex_sample ]);
    false
  with
  | Invalid_argument _ -> true
;;
