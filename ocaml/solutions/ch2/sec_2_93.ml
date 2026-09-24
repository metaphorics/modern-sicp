(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.93 *)

type term =
  { order : int
  ; coeff : float
  }

type poly =
  { var : string
  ; term_list : term list
  }

let make_term order coeff = { order; coeff }
let make_poly var term_list = { var; term_list }

let adjoin_term term term_list =
  if Float.equal term.coeff 0.0 then term_list else term :: term_list
;;

let rec add_terms l1 l2 =
  match l1, l2 with
  | [], l | l, [] -> l
  | t1 :: rest1, t2 :: rest2 ->
    if t1.order > t2.order
    then adjoin_term t1 (add_terms rest1 l2)
    else if t1.order < t2.order
    then adjoin_term t2 (add_terms l1 rest2)
    else adjoin_term (make_term t1.order (t1.coeff +. t2.coeff)) (add_terms rest1 rest2)
;;

let mul_term_by_all_terms t1 l =
  List.map (fun t2 -> make_term (t1.order + t2.order) (t1.coeff *. t2.coeff)) l
;;

let rec mul_terms l1 l2 =
  match l1 with
  | [] -> []
  | t1 :: rest1 -> add_terms (mul_term_by_all_terms t1 l2) (mul_terms rest1 l2)
;;

let add_poly p1 p2 =
  if String.equal p1.var p2.var
  then make_poly p1.var (add_terms p1.term_list p2.term_list)
  else invalid_arg "add_poly: polys not in same var"
;;

let mul_poly p1 p2 =
  if String.equal p1.var p2.var
  then make_poly p1.var (mul_terms p1.term_list p2.term_list)
  else invalid_arg "mul_poly: polys not in same var"
;;

type rational_function =
  { numer : poly
  ; denom : poly
  }

let make_rational_function numer denom = { numer; denom }

let add_rational_function rf1 rf2 =
  { numer = add_poly (mul_poly rf1.numer rf2.denom) (mul_poly rf2.numer rf1.denom)
  ; denom = mul_poly rf1.denom rf2.denom
  }
;;

let ex_2_93 () =
  let p1 = make_poly "x" [ make_term 2 1.0; make_term 0 1.0 ] in
  let p2 = make_poly "x" [ make_term 3 1.0; make_term 0 1.0 ] in
  let rf = make_rational_function p2 p1 in
  let sum = add_rational_function rf rf in
  ( List.map (fun t -> t.order, t.coeff) sum.numer.term_list
  , List.map (fun t -> t.order, t.coeff) sum.denom.term_list )
;;
