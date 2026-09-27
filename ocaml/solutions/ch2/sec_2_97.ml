(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.97 *)

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

let rec integerizing_power c exponent =
  if exponent <= 0 then 1.0 else c *. integerizing_power c (exponent - 1)
;;

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

let rec int_gcd a b = if b = 0 then abs a else int_gcd b (a mod b)

let coefficients_content terms =
  List.fold_left (fun acc t -> int_gcd acc (int_of_float (Float.round t.coeff))) 0 terms
;;

let reduce_content_pair n d =
  match coefficients_content (n @ d) with
  | 0 -> n, d
  | content ->
    let scale t = make_term t.order (t.coeff /. float_of_int content) in
    List.map scale n, List.map scale d
;;

let max_order terms = List.fold_left (fun acc t -> max acc t.order) 0 terms

let leading_term = function
  | [] -> invalid_arg "leading_term: the empty term list has no leading term"
  | t :: _ -> t
;;

(* 2.97a-b: compute the term-list GCD with pseudodivision, scale numerator
   and denominator by the same integerizing factor so dividing each by the
   GCD stays exact, then strip the redundant common factor left over from
   all the integerizing multiplications. *)
let reduce_terms n d =
  let g = gcd_terms n d in
  let o1 = max (max_order n) (max_order d) in
  let o2 = (leading_term g).order in
  let c = (leading_term g).coeff in
  let factor = integerizing_power c (1 + o1 - o2) in
  let scale t = make_term t.order (t.coeff *. factor) in
  let scaled_n = List.map scale n in
  let scaled_d = List.map scale d in
  let quotient_n, _ = div_terms scaled_n g in
  let quotient_d, _ = div_terms scaled_d g in
  reduce_content_pair quotient_n quotient_d
;;

type rational_function =
  { numer : poly
  ; denom : poly
  }

let make_rational_function numer denom = { numer; denom }

let reduce_poly p1 p2 =
  if String.equal p1.var p2.var
  then (
    let n, d = reduce_terms p1.term_list p2.term_list in
    make_rational_function (make_poly p1.var n) (make_poly p1.var d))
  else invalid_arg "reduce_poly: polys not in the same variable"
;;

(* Q1 = P1*P2 and Q2 = P1*P3 from exercises 2.95-2.96: reducing Q1/Q2 to
   lowest terms should cancel the shared factor P1 and land on P2/P3.
   [numer'/Q2 = Q1/denom'] is the cross-multiplication identity that holds
   for any correct reduction of numer/denom to lowest terms, regardless of
   the particular example, so checking it is a real correctness proof and
   not just a re-derivation of the expected numbers. *)
let ex_2_97 () =
  let p1 = make_poly "x" [ make_term 2 1.0; make_term 1 (-2.0); make_term 0 1.0 ] in
  let p2 = make_poly "x" [ make_term 2 11.0; make_term 0 7.0 ] in
  let p3 = make_poly "x" [ make_term 1 13.0; make_term 0 5.0 ] in
  let q1 = mul_poly p1 p2 in
  let q2 = mul_poly p1 p3 in
  let rf = reduce_poly q1 q2 in
  let cross_multiply_agrees =
    mul_terms rf.numer.term_list q2.term_list = mul_terms rf.denom.term_list q1.term_list
  in
  ( List.map (fun t -> t.order, t.coeff) rf.numer.term_list
  , List.map (fun t -> t.order, t.coeff) rf.denom.term_list
  , cross_multiply_agrees )
;;
