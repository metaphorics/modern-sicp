(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** The named definitions behind the listings of section 3.1, grouped by
    subsection. Each value a listing displays is asserted by
    [run_sec_3_1] through [Replay], so the book's result comments are
    true by construction. *)

type withdraw_result =
  | Balance of int
  | Insufficient_funds

module Global_withdraw = struct
  let balance = ref 100

  let withdraw amount =
    if !balance >= amount
    then (
      balance := !balance - amount;
      Balance !balance)
    else Insufficient_funds
  ;;
end

module New_withdraw = struct
  let new_withdraw =
    let balance = ref 100 in
    fun amount ->
      if !balance >= amount
      then (
        balance := !balance - amount;
        Balance !balance)
      else Insufficient_funds
  ;;
end

module Make_withdraw = struct
  let make_withdraw balance =
    let balance = ref balance in
    fun amount ->
      if !balance >= amount
      then (
        balance := !balance - amount;
        Balance !balance)
      else Insufficient_funds
  ;;
end

module Make_account = struct
  type account =
    { withdraw : int -> withdraw_result
    ; deposit : int -> int
    }

  let make_account balance =
    let balance = ref balance in
    let withdraw amount =
      if !balance >= amount
      then (
        balance := !balance - amount;
        Balance !balance)
      else Insufficient_funds
    in
    let deposit amount =
      balance := !balance + amount;
      !balance
    in
    { withdraw; deposit }
  ;;
end

module Rand = struct
  let random_init = 42L
  let bound = 1_000_000_000

  let generator =
    match Sicp_common.Random.create random_init with
    | Ok generator -> generator
    | Error Sicp_common.Error.Zero_seed -> failwith "the fixed seed 42 is nonzero"
  ;;

  let rand () = Sicp_common.Random.random generator bound
end

module Monte_carlo = struct
  let cesaro_test () = Sicp_ch1.Sec_1_2.Gcd.gcd (Rand.rand ()) (Rand.rand ()) = 1

  let monte_carlo trials experiment =
    let rec iter trials_remaining trials_passed =
      if trials_remaining = 0
      then float_of_int trials_passed /. float_of_int trials
      else if experiment ()
      then iter (trials_remaining - 1) (trials_passed + 1)
      else iter (trials_remaining - 1) trials_passed
    in
    iter trials 0
  ;;

  let estimate_pi trials = sqrt (6.0 /. monte_carlo trials cesaro_test)
end

module Random_gcd_test = struct
  (* The version that does not use [Rand.rand]'s captured state: the
     generator is threaded explicitly through every call that draws a
     number, instead of hiding behind a zero-argument closure. *)
  let random_gcd_test trials generator =
    let rec iter trials_remaining trials_passed =
      if trials_remaining = 0
      then float_of_int trials_passed /. float_of_int trials
      else (
        let x1 = Sicp_common.Random.random generator Rand.bound in
        let x2 = Sicp_common.Random.random generator Rand.bound in
        if Sicp_ch1.Sec_1_2.Gcd.gcd x1 x2 = 1
        then iter (trials_remaining - 1) (trials_passed + 1)
        else iter (trials_remaining - 1) trials_passed)
    in
    iter trials 0
  ;;

  let estimate_pi trials generator = sqrt (6.0 /. random_gcd_test trials generator)
end

module Make_simplified_withdraw = struct
  let make_simplified_withdraw balance =
    let balance = ref balance in
    fun amount ->
      balance := !balance - amount;
      !balance
  ;;
end

module Make_decrementer = struct
  let make_decrementer balance amount = balance - amount
end

module Factorial_imperative = struct
  let factorial n =
    let product = ref 1 in
    let counter = ref 1 in
    let rec iter () =
      if !counter > n
      then !product
      else (
        product := !counter * !product;
        counter := !counter + 1;
        iter ())
    in
    iter ()
  ;;
end
