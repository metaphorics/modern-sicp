(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** The named definitions behind the listings of section 1.3, grouped by
    subsection. Each value a listing displays is asserted by
    [run_sec_1_3] through [Replay], so the book's result comments are
    true by construction. *)

(** The three template procedures of subsection 1.3.1, before
    abstraction. *)
module Sum_templates = struct
  let rec sum_integers a b = if a > b then 0 else a + sum_integers (a + 1) b
  let cube x = x * x * x
  let rec sum_cubes a b = if a > b then 0 else cube a + sum_cubes (a + 1) b

  let rec pi_sum a b =
    if a > b
    then 0.0
    else (1.0 /. (Float.of_int a *. Float.of_int (a + 2))) +. pi_sum (a + 4) b
  ;;
end

(** The single [sum] abstraction of subsection 1.3.1, kept in [float]
    throughout (index, term, and accumulator alike) so the same
    procedure serves integer-indexed sums and [integral]'s real-valued
    step without a second definition. *)
module Sum_abstraction = struct
  let rec sum term a next b = if a > b then 0.0 else term a +. sum term (next a) next b
  let inc x = x +. 1.0
  let cube x = x *. x *. x
  let sum_cubes a b = sum cube (Float.of_int a) inc (Float.of_int b)
  let identity x = x
  let sum_integers a b = sum identity (Float.of_int a) inc (Float.of_int b)

  let pi_sum a b =
    let pi_term x = 1.0 /. (x *. (x +. 2.0)) in
    let pi_next x = x +. 4.0 in
    sum pi_term (Float.of_int a) pi_next (Float.of_int b)
  ;;

  let integral f a b dx =
    let add_dx x = x +. dx in
    sum f (a +. (dx /. 2.0)) add_dx b *. dx
  ;;
end

module Lambda_and_let = struct
  let pi_sum a b =
    Sum_abstraction.sum
      (fun x -> 1.0 /. (x *. (x +. 2.0)))
      (Float.of_int a)
      (fun x -> x +. 4.0)
      (Float.of_int b)
  ;;

  let integral f a b dx =
    Sum_abstraction.sum f (a +. (dx /. 2.0)) (fun x -> x +. dx) b *. dx
  ;;

  let square x = x * x
  let plus4 x = x + 4
  let plus4_via_lambda = fun x -> x + 4
  let lambda_as_operator () = (fun x y z -> x + y + square z) 1 2 3

  let f_via_helper x y =
    let f_helper a b = (x *. (a *. a)) +. (y *. b) +. (a *. b) in
    f_helper (1.0 +. (x *. y)) (1.0 -. y)
  ;;

  let f_via_lambda x y =
    (fun a b -> (x *. (a *. a)) +. (y *. b) +. (a *. b)) (1.0 +. (x *. y)) (1.0 -. y)
  ;;

  let f_via_let x y =
    let a = 1.0 +. (x *. y) in
    let b = 1.0 -. y in
    (x *. (a *. a)) +. (y *. b) +. (a *. b)
  ;;

  let let_shadows_outer outer_x =
    (let x = 3.0 in
     x +. (x *. 10.0))
    +. outer_x
  ;;

  let let_inner_value () =
    let x = 3.0 in
    x +. (x *. 10.0)
  ;;

  let let_binds_from_outer_scope outer_x =
    let x = 3.0 in
    let y = outer_x +. 2.0 in
    x *. y
  ;;
end

module Numeric_error = struct
  type t =
    | Values_not_of_opposite_sign
    | Not_converged

  let pp ppf = function
    | Values_not_of_opposite_sign ->
      Format.fprintf ppf "the function values at the endpoints are not of opposite sign"
    | Not_converged ->
      Format.fprintf ppf "the search did not settle within its iteration cap"
  ;;
end

module Half_interval = struct
  let close_enough a b = Float.abs (a -. b) < 0.001

  let rec search f neg_point pos_point =
    let midpoint = (neg_point +. pos_point) /. 2.0 in
    if close_enough neg_point pos_point
    then midpoint
    else (
      let test_value = f midpoint in
      if test_value > 0.0
      then search f neg_point midpoint
      else if test_value < 0.0
      then search f midpoint pos_point
      else midpoint)
  ;;

  let half_interval_method f a b =
    let a_value = f a in
    let b_value = f b in
    if Float.compare a_value 0.0 < 0 && Float.compare b_value 0.0 > 0
    then Ok (search f a b)
    else if Float.compare b_value 0.0 < 0 && Float.compare a_value 0.0 > 0
    then Ok (search f b a)
    else Error Numeric_error.Values_not_of_opposite_sign
  ;;
end

module Fixed_point = struct
  let tolerance = 0.00001

  let fixed_point ?(max_iterations = 100_000) f first_guess =
    let close_enough v1 v2 = Float.abs (v1 -. v2) < tolerance in
    let rec try_guess guess n =
      if n > max_iterations
      then Error Numeric_error.Not_converged
      else (
        let next = f guess in
        if close_enough guess next then Ok next else try_guess next (n + 1))
    in
    try_guess first_guess 0
  ;;
end

module Average_damping = struct
  let average x y = (x +. y) /. 2.0
  let average_damp f x = average x (f x)
  let sqrt x = Fixed_point.fixed_point (average_damp (fun y -> x /. y)) 1.0
  let cube_root x = Fixed_point.fixed_point (average_damp (fun y -> x /. (y *. y))) 1.0
end

module Newtons_method = struct
  let dx = 0.00001
  let deriv g x = (g (x +. dx) -. g x) /. dx
  let cube x = x *. x *. x
  let newton_transform g x = x -. (g x /. deriv g x)
  let newtons_method g guess = Fixed_point.fixed_point (newton_transform g) guess
  let sqrt x = newtons_method (fun y -> (y *. y) -. x) 1.0
end

module First_class_procedures = struct
  let fixed_point_of_transform g transform guess =
    Fixed_point.fixed_point (transform g) guess
  ;;

  let sqrt_via_average_damp x =
    fixed_point_of_transform (fun y -> x /. y) Average_damping.average_damp 1.0
  ;;

  let sqrt_via_newton_transform x =
    fixed_point_of_transform (fun y -> (y *. y) -. x) Newtons_method.newton_transform 1.0
  ;;
end
