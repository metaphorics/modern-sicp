(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program expmod in SICP section 1.2 exercise
   1.28 *)

(** Exercise 1.28: [miller_rabin_expmod] behaves as [expmod] except at
    every squaring step, where it also checks whether the value about
    to be squared is a nontrivial square root of 1 modulo [m] (not
    equal to 1 or [m - 1], yet its square is 1 modulo [m]); if so it
    returns 0, the signal the hint asks for, since 0 can never equal 1
    and so forces the outer test to reject. For a genuine prime no such
    root exists, so the signal never fires and the result is the
    ordinary Fermat power; for a Carmichael number the signal fires for
    at least half the witnesses, so the test rejects them where the
    plain Fermat test cannot. *)

let is_even n = n mod 2 = 0

let square_checking_root x m =
  if x <> 1 && x <> m - 1 && x * x mod m = 1 then 0 else x * x mod m
;;

let rec miller_rabin_expmod base exp m =
  if exp = 0
  then 1
  else if is_even exp
  then square_checking_root (miller_rabin_expmod base (exp / 2) m) m
  else base * miller_rabin_expmod base (exp - 1) m mod m
;;

let miller_rabin_test n gen =
  let a = 1 + Sicp_common.Random.random gen (n - 1) in
  miller_rabin_expmod a (n - 1) n = 1
;;

let rec miller_rabin_prime n times gen =
  if times = 0
  then true
  else if miller_rabin_test n gen
  then miller_rabin_prime n (times - 1) gen
  else false
;;

let carmichael_numbers = [ 561; 1105; 1729; 2465; 2821; 6601 ]

let ex_1_28 () =
  match Sicp_common.Random.create 42L with
  | Error Sicp_common.Error.Zero_seed -> failwith "the fixed seed 42 is nonzero"
  | Ok gen -> List.map (fun n -> n, miller_rabin_prime n 30 gen) carmichael_numbers
;;
