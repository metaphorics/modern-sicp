(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.96 *)

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

(* [c ** exponent], read off the leading coefficient of the divisor as
   described in the text: multiplying the dividend by this factor before
   dividing guarantees the division introduces no fractions. *)
let rec integerizing_power c exponent =
  if exponent <= 0 then 1.0 else c *. integerizing_power c (exponent - 1)
;;

(* 2.96a: the same shape as [remainder_terms], except the dividend is first
   scaled by the integerizing factor c^(1+O1-O2). *)
let pseudoremainder_terms l1 l2 =
  match l1, l2 with
  | [], _ -> []
  | _, [] -> invalid_arg "pseudoremainder_terms: division by the empty term list"
  | t1 :: _, t2 :: _ ->
    let o1 = t1.order
    and o2 = t2.order
    and c = t2.coeff in
    let factor = integerizing_power c (1 + o1 - o2) in
    let scaled_dividend = List.map (fun t -> make_term t.order (t.coeff *. factor)) l1 in
    remainder_terms scaled_dividend l2
;;

let rec gcd_terms a b =
  match b with
  | [] -> a
  | _ :: _ -> gcd_terms b (pseudoremainder_terms a b)
;;

let gcd_poly p1 p2 =
  if String.equal p1.var p2.var
  then make_poly p1.var (gcd_terms p1.term_list p2.term_list)
  else invalid_arg "gcd_poly: polys not in the same variable"
;;

let rec int_gcd a b = if b = 0 then abs a else int_gcd b (a mod b)

(* 2.96b: the pseudo-GCD comes back with integer coefficients but a large
   redundant common factor; divide every coefficient by their (integer) GCD
   to strip it back down. *)
let coefficients_content terms =
  List.fold_left (fun acc t -> int_gcd acc (int_of_float (Float.round t.coeff))) 0 terms
;;

let reduce_content terms =
  match coefficients_content terms with
  | 0 -> terms
  | content ->
    List.map (fun t -> make_term t.order (t.coeff /. float_of_int content)) terms
;;

let gcd_terms_reduced a b = reduce_content (gcd_terms a b)

let gcd_poly_reduced p1 p2 =
  if String.equal p1.var p2.var
  then make_poly p1.var (gcd_terms_reduced p1.term_list p2.term_list)
  else invalid_arg "gcd_poly_reduced: polys not in the same variable"
;;

(* Re-run exercise 2.95's example: Q1 = P1*P2, Q2 = P1*P3. Part a:
   pseudodivision keeps every coefficient of the raw GCD an integer
   (unlike the naive gcd_terms from 2.95, which produced a fractional
   constant). Part b: dividing the raw GCD by its content leaves a
   coprime coefficient list, so its own content is 1. *)
let ex_2_96 () =
  let p1 = make_poly "x" [ make_term 2 1.0; make_term 1 (-2.0); make_term 0 1.0 ] in
  let p2 = make_poly "x" [ make_term 2 11.0; make_term 0 7.0 ] in
  let p3 = make_poly "x" [ make_term 1 13.0; make_term 0 5.0 ] in
  let q1 = mul_poly p1 p2 in
  let q2 = mul_poly p1 p3 in
  let raw = gcd_poly q1 q2 in
  let no_inexact_division =
    List.for_all (fun t -> Float.equal (Float.round t.coeff) t.coeff) raw.term_list
  in
  let reduced = gcd_poly_reduced q1 q2 in
  let reduced_content = coefficients_content reduced.term_list in
  no_inexact_division, reduced_content
;;
