(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Pairs = struct
  let x = 1, 2
  let car_x = fst x
  let cdr_x = snd x
  let nested_z = (1, 2), (3, 4)
  let car_car_z = fst (fst nested_z)
  let car_cdr_z = fst (snd nested_z)
end

module type Rational_number = sig
  type t

  val make_rat : int -> int -> t
  val numer : t -> int
  val denom : t -> int
end

module Rational_arithmetic (R : Rational_number) = struct
  let add_rat x y =
    R.make_rat ((R.numer x * R.denom y) + (R.numer y * R.denom x)) (R.denom x * R.denom y)
  ;;

  let sub_rat x y =
    R.make_rat ((R.numer x * R.denom y) - (R.numer y * R.denom x)) (R.denom x * R.denom y)
  ;;

  let mul_rat x y = R.make_rat (R.numer x * R.numer y) (R.denom x * R.denom y)
  let div_rat x y = R.make_rat (R.numer x * R.denom y) (R.denom x * R.numer y)
  let equal_rat x y = R.numer x * R.denom y = R.numer y * R.denom x
  let print_rat x = Printf.printf "%d/%d\n" (R.numer x) (R.denom x)
end

module Unreduced : Rational_number = struct
  type t = int * int

  let make_rat n d = n, d
  let numer (n, _) = n
  let denom (_, d) = d
end

module Unreduced_ops = Rational_arithmetic (Unreduced)

module Reduced : Rational_number = struct
  type t = int * int

  let make_rat n d =
    let g = Sicp_ch1.Sec_1_2.Gcd.gcd (abs n) (abs d) in
    n / g, d / g
  ;;

  let numer (n, _) = n
  let denom (_, d) = d
end

module Reduced_ops = Rational_arithmetic (Reduced)

module Rational_error = struct
  type t = Zero_denominator of { n : int }

  let to_string = function
    | Zero_denominator { n } -> Printf.sprintf "make %d 0: zero denominator" n
  ;;
end

module Rational : sig
  type t

  val make : int -> int -> (t, Rational_error.t) result
  val numer : t -> int
  val denom : t -> int
end = struct
  type t =
    { n : int
    ; d : int
    }

  let make n d =
    if d = 0
    then Error (Rational_error.Zero_denominator { n })
    else (
      let g = Sicp_ch1.Sec_1_2.Gcd.gcd (abs n) (abs d) in
      Ok { n = n / g; d = d / g })
  ;;

  let numer r = r.n
  let denom r = r.d
end

module Lazy_reduced : Rational_number = struct
  type t = int * int

  let make_rat n d = n, d
  let numer (n, d) = n / Sicp_ch1.Sec_1_2.Gcd.gcd (abs n) (abs d)
  let denom (n, d) = d / Sicp_ch1.Sec_1_2.Gcd.gcd (abs n) (abs d)
end

module Lazy_reduced_ops = Rational_arithmetic (Lazy_reduced)

module Procedural_pairs = struct
  let cons x y m =
    match m with
    | 0 -> x
    | 1 -> y
    | _ -> invalid_arg "Argument not 0 or 1: CONS"
  ;;

  let car z = z 0
  let cdr z = z 1
end

module type Interval_number = sig
  type t

  val make_interval : float -> float -> t
  val lower_bound : t -> float
  val upper_bound : t -> float
end

module Interval_arithmetic (I : Interval_number) = struct
  let add_interval x y =
    I.make_interval
      (I.lower_bound x +. I.lower_bound y)
      (I.upper_bound x +. I.upper_bound y)
  ;;

  let mul_interval x y =
    let p1 = I.lower_bound x *. I.lower_bound y in
    let p2 = I.lower_bound x *. I.upper_bound y in
    let p3 = I.upper_bound x *. I.lower_bound y in
    let p4 = I.upper_bound x *. I.upper_bound y in
    I.make_interval
      (Float.min (Float.min p1 p2) (Float.min p3 p4))
      (Float.max (Float.max p1 p2) (Float.max p3 p4))
  ;;

  let div_interval x y =
    mul_interval x (I.make_interval (1.0 /. I.upper_bound y) (1.0 /. I.lower_bound y))
  ;;
end

module Interval_center_width (I : Interval_number) = struct
  let make_center_width c w = I.make_interval (c -. w) (c +. w)
  let center i = (I.lower_bound i +. I.upper_bound i) /. 2.0
  let width i = (I.upper_bound i -. I.lower_bound i) /. 2.0
end

module Parallel_resistors (I : Interval_number) = struct
  module Ops = Interval_arithmetic (I)

  let par1 r1 r2 = Ops.div_interval (Ops.mul_interval r1 r2) (Ops.add_interval r1 r2)

  let par2 r1 r2 =
    let one = I.make_interval 1.0 1.0 in
    Ops.div_interval
      one
      (Ops.add_interval (Ops.div_interval one r1) (Ops.div_interval one r2))
  ;;
end
