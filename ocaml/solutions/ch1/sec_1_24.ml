(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program timed-prime-test in SICP section 1.2
   exercise 1.24 *)

(** Exercise 1.24: [fast_prime], applied to every prime exercise 1.22
    found, one fixed-seed generator threaded through all twelve calls.
    A true prime satisfies Fermat's Little Theorem for every witness,
    so every call answers [true] regardless of which values the
    generator draws; the timing comparison against exercise 1.22's
    @math{{\Theta(\sqrt n)}} search is [ex_1_24.md]'s job. *)

let is_even n = n mod 2 = 0
let square x = x * x

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

let twelve_primes =
  [ 1009
  ; 1013
  ; 1019
  ; 10007
  ; 10009
  ; 10037
  ; 100003
  ; 100019
  ; 100043
  ; 1000003
  ; 1000033
  ; 1000037
  ]
;;

let ex_1_24 () =
  match Sicp_common.Random.create 42L with
  | Error Sicp_common.Error.Zero_seed -> failwith "the fixed seed 42 is nonzero"
  | Ok gen -> List.map (fun n -> fast_prime n 20 gen) twelve_primes
;;
