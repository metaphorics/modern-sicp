(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** The named definitions behind the listings of section 3.2, grouped by
    subsection. Each value a listing displays is asserted by
    [run_sec_3_2] through [Replay], so the book's result comments are
    true by construction. *)

type withdraw_result =
  | Balance of int
  | Insufficient_funds

(** 3.2.1: a definition is sugar for binding the name to a [fun] value;
    both spellings bind [square] to the same kind of closure in the
    global frame. *)
module Square = struct
  let square x = x * x
  let square_from_fun = fun x -> x * x
end

(** 3.2.2: the three closures of the [f 5] walkthrough, each created in
    the global environment. *)
module F_and_sum_of_squares = struct
  let square x = x * x
  let sum_of_squares x y = square x + square y
  let f a = sum_of_squares (a + 1) (a * 2)
end

(** 3.2.3: [make_withdraw] and the closure-plus-cell shapes the section
    draws: one cell shared by [bump] and [peek], and one cell per
    one-closure counter. *)
module Local_state = struct
  let make_withdraw balance =
    let balance = ref balance in
    fun amount ->
      if !balance >= amount
      then (
        balance := !balance - amount;
        Balance !balance)
      else Insufficient_funds
  ;;

  let make_shared_counter () =
    let count = ref 0 in
    let bump () =
      count := !count + 1;
      !count
    in
    let peek () = !count in
    bump, peek
  ;;

  let make_counter () =
    let count = ref 0 in
    fun () ->
      count := !count + 1;
      !count
  ;;
end

(** 3.2.4: the block-structured square root whose internal definitions
    Figure 3.11 places in the frame of one [sqrt] call. *)
module Internal_definitions = struct
  let sqrt x =
    let good_enough guess = Float.abs ((guess *. guess) -. x) < 0.001 in
    let improve guess = (guess +. (x /. guess)) /. 2.0 in
    let rec sqrt_iter guess =
      if good_enough guess then guess else sqrt_iter (improve guess)
    in
    sqrt_iter 1.0
  ;;
end
