(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program timed-prime-test in SICP section 1.2
   exercise 1.22 *)

(** Exercise 1.22: [timed_prime_test] returns [None] for a composite
    (mirroring the book's silent skip: nothing is printed) and
    [Some elapsed] for a prime, [elapsed] read from [Sys.time] around
    the test. [search_for_primes] walks candidates upward, one integer
    at a time, collecting the primality checks that succeed; timing the
    individual calls and reading whether the section's @math{{\Theta(\sqrt n)}}
    prediction holds is [ex_1_22.md]'s job, since a single call is
    usually too fast for [Sys.time] to see. *)

let square x = x * x

let rec find_divisor n test_divisor =
  if square test_divisor > n
  then n
  else if n mod test_divisor = 0
  then test_divisor
  else find_divisor n (test_divisor + 1)
;;

let smallest_divisor n = find_divisor n 2
let is_prime n = n > 1 && n = smallest_divisor n

let timed_prime_test n =
  let start = Sys.time () in
  if is_prime n then Some (Sys.time () -. start) else None
;;

let search_for_primes start count =
  let rec go candidate found =
    if List.length found = count
    then List.rev found
    else if is_prime candidate
    then go (candidate + 1) (candidate :: found)
    else go (candidate + 1) found
  in
  go start []
;;

let ex_1_22 () =
  List.map
    (fun start -> start, search_for_primes start 3)
    [ 1000; 10000; 100000; 1000000 ]
;;
