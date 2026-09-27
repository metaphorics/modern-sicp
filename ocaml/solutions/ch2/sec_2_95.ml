(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.95 *)

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

let rec mul_terms l1 l2 =
  match l1 with
  | [] -> []
  | t1 :: rest1 -> add_terms (mul_term_by_all_terms t1 l2) (mul_terms rest1 l2)
;;

let mul_poly p1 p2 =
  if String.equal p1.var p2.var
  then make_poly p1.var (mul_terms p1.term_list p2.term_list)
  else invalid_arg "mul_poly: polys not in the same variable"
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

let remainder_terms l1 l2 = snd (div_terms l1 l2)

let rec gcd_terms a b =
  match b with
  | [] -> a
  | _ :: _ -> gcd_terms b (remainder_terms a b)
;;

let gcd_poly p1 p2 =
  if String.equal p1.var p2.var
  then make_poly p1.var (gcd_terms p1.term_list p2.term_list)
  else invalid_arg "gcd_poly: polys not in the same variable"
;;

let scalar_ratio_to reference terms =
  if List.length reference <> List.length terms
  then None
  else (
    let pairs = List.combine reference terms in
    if List.exists (fun (r, t) -> r.order <> t.order) pairs
    then None
    else (
      match pairs with
      | [] -> None
      | (r0, t0) :: _ ->
        let k0 = t0.coeff /. r0.coeff in
        if
          List.for_all (fun (r, t) -> Float.abs ((t.coeff /. r.coeff) -. k0) < 1e-6) pairs
        then Some k0
        else None))
;;

(* The book's P1, P2, P3 from the text, and Q1 = P1*P2, Q2 = P1*P3. Running
   exercise 2.94's naive [gcd_terms] on Q1 and Q2 introduces fractions at
   every division step; here that collapses the GCD down to a lone order-0
   constant term instead of a degree-2 polynomial proportional to P1,
   exactly the failure the book's footnote describes for a system whose
   coefficient division produces limited-precision results instead of exact
   rationals: [scalar_ratio_to] reports [None] because the two term lists
   do not even have matching shape, let alone a common scalar factor. *)
let ex_2_95 () =
  let p1 = make_poly "x" [ make_term 2 1.0; make_term 1 (-2.0); make_term 0 1.0 ] in
  let p2 = make_poly "x" [ make_term 2 11.0; make_term 0 7.0 ] in
  let p3 = make_poly "x" [ make_term 1 13.0; make_term 0 5.0 ] in
  let q1 = mul_poly p1 p2 in
  let q2 = mul_poly p1 p3 in
  let g = gcd_poly q1 q2 in
  ( List.map (fun t -> t.order, t.coeff) g.term_list
  , scalar_ratio_to p1.term_list g.term_list )
;;
