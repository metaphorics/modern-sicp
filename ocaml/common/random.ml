(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

type t = { mutable state : Int64.t }

let multiplier = 0x2545F4914F6CDD1DL

let create seed =
  if Int64.equal seed 0L then Error Error.Zero_seed else Ok { state = seed }
;;

let next_u64 t =
  let x = Int64.logxor t.state (Int64.shift_right_logical t.state 12) in
  let x = Int64.logxor x (Int64.shift_left x 25) in
  let x = Int64.logxor x (Int64.shift_right_logical x 27) in
  t.state <- x;
  Int64.mul x multiplier
;;

let random t n =
  if n <= 0 then invalid_arg "Random.random: n must be positive";
  Int64.to_int (Int64.unsigned_rem (next_u64 t) (Int64.of_int n))
;;
