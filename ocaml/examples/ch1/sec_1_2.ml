(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** The named definitions behind the listings of section 1.2, grouped by
    subsection. Each value a listing displays is asserted by
    [run_sec_1_2] through [Replay], so the book's result comments are
    true by construction. Listings that call a later definition keep the
    book's presentation order in the text; here every definition precedes
    its use, as OCaml requires. *)

(** The factorials of subsection 1.2.1: the linear recursive process, the
    linear iterative process carried by [fact_iter], and the
    block-structured variant the book's footnote hides inside
    [factorial]. [recursive] is exact through 20 and wraps at 21, the
    boundary the section's margin states. *)
module Factorial = struct
  let rec recursive n = if n = 1 then 1 else n * recursive (n - 1)

  let rec fact_iter product counter max_count =
    if counter > max_count
    then product
    else fact_iter (counter * product) (counter + 1) max_count
  ;;

  let iterative n = fact_iter 1 1 n

  let hidden n =
    let rec iter product counter =
      if counter > n then product else iter (counter * product) (counter + 1)
    in
    iter 1 1
  ;;
end

(** The Fibonacci procedures of subsection 1.2.2: the tree-recursive
    [recursive], the linear iteration carried by [fib_iter] whose
    self-call is its whole result, and the [count_change] tree the
    section counts a dollar with. *)
module Fibonacci = struct
  let rec recursive n =
    if n = 0 then 0 else if n = 1 then 1 else recursive (n - 1) + recursive (n - 2)
  ;;

  let rec fib_iter a b count = if count = 0 then b else fib_iter (a + b) a (count - 1)
  let iterative n = fib_iter 1 0 n
end

(** Counting change, subsection 1.2.2. [first_denomination] reads the
    number of kinds of coins; [cc] runs the reduction rule; the book's
    interaction is [count_change 100]. *)
module Counting_change = struct
  let first_denomination kinds_of_coins =
    match kinds_of_coins with
    | 1 -> 1
    | 2 -> 5
    | 3 -> 10
    | 4 -> 25
    | _ -> 50
  ;;

  let rec cc amount kinds_of_coins =
    if amount = 0
    then 1
    else if amount < 0 || kinds_of_coins = 0
    then 0
    else
      cc amount (kinds_of_coins - 1)
      + cc (amount - first_denomination kinds_of_coins) kinds_of_coins
  ;;

  let count_change amount = cc amount 5
end

(** Exponentiation, subsection 1.2.4: the linear recursive [expt], the
    linear iterative [expt_iter] behind [expt_iterative], and
    successive squaring in [fast_expt]. [is_even] is the book's
    [even?] over the remainder operator. *)
module Exponentiation = struct
  let square x = x * x
  let is_even n = n mod 2 = 0
  let rec expt b n = if n = 0 then 1 else b * expt b (n - 1)

  let rec expt_iter b counter product =
    if counter = 0 then product else expt_iter b (counter - 1) (b * product)
  ;;

  let expt_iterative b n = expt_iter b n 1

  let rec fast_expt b n =
    if n = 0
    then 1
    else if is_even n
    then square (fast_expt b (n / 2))
    else b * fast_expt b (n - 1)
  ;;
end

(** Euclid's Algorithm, subsection 1.2.5. The self-call is the entire
    result, so OCaml runs it in constant space. *)
module Gcd = struct
  let rec gcd a b = if b = 0 then a else gcd b (a mod b)
end

(** Testing for primality, subsection 1.2.6: trial division up to the
    square root, then the Fermat test over modular successive squaring.
    [fermat_test] and [fast_prime] take the edition's seeded generator
    as an explicit argument, so a fixed seed reproduces a run. *)
module Primality = struct
  let square x = x * x
  let divides a b = b mod a = 0
  let is_even n = n mod 2 = 0

  let rec find_divisor n test_divisor =
    if square test_divisor > n
    then n
    else if divides test_divisor n
    then test_divisor
    else find_divisor n (test_divisor + 1)
  ;;

  let smallest_divisor n = find_divisor n 2
  let is_prime n = n = smallest_divisor n

  let rec expmod base exp m =
    if exp = 0
    then 1
    else if is_even exp
    then square (expmod base (exp / 2) m) mod m
    else base * expmod base (exp - 1) m mod m
  ;;

  let fermat_test n gen =
    let try_it a = expmod a n n = a in
    try_it (1 + Sicp_common.Random.random gen (n - 1))
  ;;

  let rec fast_prime n times gen =
    if times = 0
    then true
    else if fermat_test n gen
    then fast_prime n (times - 1) gen
    else false
  ;;
end
