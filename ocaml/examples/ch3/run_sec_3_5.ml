(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Replay = Sicp_ch1.Replay
module Sec_3_5 = Sicp_ch3.Sec_3_5
module S = Sec_3_5.Streams
module I = Sec_3_5.Infinite
module Se = Sec_3_5.Series
module C = Sec_3_5.Convergence
module P = Sec_3_5.Pairs
module Si = Sec_3_5.Signals
module So = Sec_3_5.Solve
module R = Sec_3_5.Random_streams

let show_int n = string_of_int n
let show_float = Printf.sprintf "%.15g"
let show_pair (a, b) = Printf.sprintf "(%d, %d)" a b

(* The book's display-stream prints a newline before each element; a
   finite prefix stands in for the infinitely many the listings show. *)
let shown_lines show n s =
  String.concat "" (List.map (fun x -> "\n" ^ show x) (S.stream_take n s))
;;

let expect_int n shown = Replay.expect (show_int n) shown
let expect_float f shown = Replay.expect (show_float f) shown

let expect_pairs ps shown =
  Replay.expect (String.concat " " (List.map show_pair ps)) shown
;;

let expect_lines lines shown = Replay.expect lines shown

let () =
  (* The two sum-of-primes programs of the opening compute the same
     total over 2..100, and the stream filter reaches the second prime
     past 10,000 without enumerating the interval. *)
  let sum_primes_iter low high =
    let rec go count accum =
      if count > high
      then accum
      else if Sicp_ch1.Sec_1_2.Primality.is_prime count
      then go (count + 1) (accum + count)
      else go (count + 1) accum
    in
    go low 0
  in
  expect_int
    (sum_primes_iter 2 100)
    (string_of_int
       (List.fold_left
          ( + )
          0
          (List.filter
             Sicp_ch1.Sec_1_2.Primality.is_prime
             (S.stream_take 99 (S.stream_enumerate_interval 2 100)))));
  expect_int
    (S.stream_car
       (S.stream_cdr
          (S.stream_filter
             Sicp_ch1.Sec_1_2.Primality.is_prime
             (S.stream_enumerate_interval 10000 1000000))))
    "10009";
  (* The infinite streams of 3.5.2, by generator and by self-reference. *)
  expect_int (S.stream_ref I.no_sevens 100) "117";
  expect_int (S.stream_ref I.primes 50) "233";
  expect_int (S.stream_ref I.primes_alt 50) "233";
  Replay.expect
    (String.concat " " (List.map show_int (S.stream_take 10 I.fibs)))
    "0 1 1 2 3 5 8 13 21 34";
  Replay.expect
    (String.concat " " (List.map show_int (S.stream_take 10 I.fibs_implicit)))
    "0 1 1 2 3 5 8 13 21 34";
  Replay.expect
    (String.concat " " (List.map show_int (S.stream_take 8 I.integers_implicit)))
    "1 2 3 4 5 6 7 8";
  Replay.expect
    (String.concat " " (List.map show_int (S.stream_take 8 I.double)))
    "1 2 4 8 16 32 64 128";
  (* The power series of 3.5.3: the first coefficients of e^x. *)
  Replay.expect
    (String.concat " " (List.map show_float (S.stream_take 6 Se.exp_series)))
    "1 1 0.5 0.166666666666667 0.0416666666666667 0.00833333333333333";
  (* The square-root stream of guesses for 2. *)
  expect_lines
    (shown_lines show_float 5 (C.sqrt_stream 2.0))
    "\n1\n1.5\n1.41666666666667\n1.41421568627451\n1.41421356237469";
  (* The pi stream and its Euler acceleration. *)
  expect_lines
    (shown_lines show_float 8 C.pi_stream)
    "\n\
     4\n\
     2.66666666666667\n\
     3.46666666666667\n\
     2.8952380952381\n\
     3.33968253968254\n\
     2.97604617604618\n\
     3.28373848373848\n\
     3.01707181707182";
  expect_lines
    (shown_lines show_float 8 (C.euler_transform C.pi_stream))
    "\n\
     3.16666666666667\n\
     3.13333333333333\n\
     3.1452380952381\n\
     3.13968253968254\n\
     3.14271284271284\n\
     3.14088134088134\n\
     3.14207181707182\n\
     3.14125482360777";
  expect_lines
    (shown_lines show_float 8 (C.accelerated_sequence C.euler_transform C.pi_stream))
    "\n\
     4\n\
     3.16666666666667\n\
     3.14210526315789\n\
     3.141599357319\n\
     3.14159271403378\n\
     3.14159265397529\n\
     3.14159265359118\n\
     3.14159265358978";
  (* The stream of pairs above the diagonal. *)
  expect_pairs
    (S.stream_take 10 (P.pairs I.integers I.integers))
    "(1, 1) (1, 2) (2, 2) (1, 3) (2, 3) (1, 4) (3, 3) (1, 5) (2, 4) (1, 6)";
  (* The integrator over a constant unit signal. *)
  let rec unit_current = S.Cons (1.0, lazy unit_current) in
  Replay.expect
    (String.concat
       " "
       (List.map show_float (S.stream_take 5 (Si.integral unit_current 0.0 0.5))))
    "0 0.5 1 1.5 2";
  (* The delayed-integrand solve: y(1) of dy/dt = y, y(0) = 1, near e. *)
  expect_float (S.stream_ref (So.solve (fun y -> y) 1.0 0.001) 1000) "2.7169239322359";
  (* The random stream and the Monte Carlo pi sequence: deterministic
     under the fixed seed, and converging toward pi by the 10000th
     trial. *)
  expect_float (S.stream_ref R.pi_stream 9999) "3.1136183912267";
  let rec zeros = S.Cons (0.0, lazy zeros) in
  let amounts = S.cons_stream 25.0 (fun () -> S.cons_stream 40.0 (fun () -> zeros)) in
  Replay.expect
    (String.concat
       " "
       (List.map show_float (S.stream_take 3 (R.stream_withdraw 100.0 amounts))))
    "100 75 35"
;;
