(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** The named definitions behind the listings of section 1.1, grouped by
    subsection. Each value a listing displays is asserted by
    [run_sec_1_1] through [Replay], so the book's result comments are
    true by construction. *)

(** The interactions of subsection 1.1.1: arithmetic expressions over
    [int] and [float], nested combination trees, and the deep expression
    the book pretty-prints. *)
module Expressions = struct
  let forty_eight_six = 486
  let sum = 137 + 349
  let difference = 1000 - 334
  let product = 5 * 99
  let quotient = 10 / 5
  let mixed = 2.7 +. 10.0
  let chained_sum = 21 + 35 + 12 + 7
  let chained_product = 25 * 4 * 12
  let nested = (3 * 5) + (10 - 6)
  let essential_parens = 10 - (4 - 3)
  let deep = (3 * ((2 * 4) + (3 + 5))) + (10 - 7 + 6)
end

(** The named values of subsection 1.1.2: [size], then the circle
    quantities computed across the [int]/[float] boundary with an
    explicit [Float.of_int]. *)
module Naming = struct
  let size = 2
  let five_times_size = 5 * size
  let pi = 3.14159
  let radius = 10
  let area = pi *. Float.of_int (radius * radius)
  let circumference = 2.0 *. pi *. Float.of_int radius
end

(** The compound procedures of subsection 1.1.4: [square] as a building
    block under [sum_of_squares] and [f]. *)
module Compound = struct
  let square x = x * x
  let sum_of_squares x y = square x + square y
  let f a = sum_of_squares (a + 1) (a * 2)
end

(** The conditional forms of subsection 1.1.6: [abs] as a case analysis
    spelled three ways, and the two equivalent [greater_or_equal]
    definitions. *)
module Conditionals = struct
  let abs_cases x = if x > 0 then x else if x = 0 then 0 else -x
  let abs_two_way x = if x < 0 then -x else x

  let abs_match x =
    match x with
    | _ when x < 0 -> -x
    | _ -> x
  ;;

  let in_range x = x > 5 && x < 10
  let greater_or_equal x y = x > y || x = y
  let greater_or_equal_not x y = not (x < y)
end

(** The square-root program of subsection 1.1.7, flat first and then in
    block structure with internal definitions, as the book presents it.
    [square] here is the [float] version; it shadows the [int] [square]
    of [Compound], the same redefinition the book makes. *)
module Sqrt = struct
  let square x = x *. x
  let average x y = (x +. y) /. 2.0
  let improve guess x = average guess (x /. guess)
  let good_enough guess x = Float.abs (square guess -. x) < 0.001

  let rec sqrt_iter guess x =
    if good_enough guess x then guess else sqrt_iter (improve guess x) x
  ;;

  let sqrt x = sqrt_iter 1.0 x

  let sqrt_block x =
    let good_enough guess = Float.abs (square guess -. x) < 0.001 in
    let improve guess = average guess (x /. guess) in
    let rec sqrt_iter guess =
      if good_enough guess then guess else sqrt_iter (improve guess)
    in
    sqrt_iter 1.0
  ;;
end

(** The black-box point of subsection 1.1.8: two definitions of
    [square] on [float] with the same type and contract. They agree
    exactly on some inputs and differ by a final-bit rounding on
    others, which the section's prose now has to say. *)
module Black_box = struct
  let square x = x *. x
  let double x = x +. x
  let square_via_exp x = exp (double (log x))
end
