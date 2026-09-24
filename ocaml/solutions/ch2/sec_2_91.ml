(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.91 *)

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

let negate_terms l = List.map (fun t -> make_term t.order (-.t.coeff)) l
let sub_terms l1 l2 = add_terms l1 (negate_terms l2)

let mul_term_by_all_terms t1 l =
  List.map (fun t2 -> make_term (t1.order + t2.order) (t1.coeff *. t2.coeff)) l
;;

let rec div_terms l1 l2 =
  match l1 with
  | [] -> [], []
  | t1 :: _ ->
    let t2 =
      match l2 with
      | [] -> invalid_arg "div_terms: division by the empty term list"
      | t2 :: _ -> t2
    in
    if t2.order > t1.order
    then [], l1
    else (
      let new_term = make_term (t1.order - t2.order) (t1.coeff /. t2.coeff) in
      let quotient_rest, remainder =
        div_terms (sub_terms l1 (mul_term_by_all_terms new_term l2)) l2
      in
      adjoin_term new_term quotient_rest, remainder)
;;

let div_poly p1 p2 =
  if String.equal p1.var p2.var
  then (
    let q, r = div_terms p1.term_list p2.term_list in
    make_poly p1.var q, make_poly p1.var r)
  else invalid_arg "div_poly: polys not in same var"
;;

let ex_2_91 () =
  let p1 = make_poly "x" [ make_term 5 1.0; make_term 0 (-1.0) ] in
  let p2 = make_poly "x" [ make_term 2 1.0; make_term 0 (-1.0) ] in
  let q, r = div_poly p1 p2 in
  ( List.map (fun t -> t.order, t.coeff) q.term_list
  , List.map (fun t -> t.order, t.coeff) r.term_list )
;;
