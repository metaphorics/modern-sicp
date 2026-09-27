(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

(** Exercise 5.29: the stack of the tree-recursive Fibonacci on the
    monitored evaluator, and the two formulas: the maximum depth in
    terms of n, and the total pushes as [a*Fib(n+1) + b], grown by the
    recurrence [S(n) = S(n-1) + S(n-2) + k]. *)

let ( >>= ) = Result.bind

module Eval = Sicp_ch5.Sec_5_4
module Measured = Sec_5_26

let fib_source =
  {|(define (fib n)
  (if (< n 2)
      n
      (+ (fib (- n 1)) (fib (- n 2)))))|}
;;

(** [measure_fib ns] runs [(fib n)] for each [n] on a fresh monitored
    machine and answers the counters. *)
let measure_fib ns =
  Eval.result_all
    (List.map
       (fun n ->
          Measured.run (fib_source ^ "\n(fib " ^ string_of_int n ^ ")")
          >>= fun lines ->
          match List.rev (Measured.stats_of lines) with
          | s :: _ -> Ok s
          | [] -> Error (Eval.Op_failed "the call printed no stack statistics"))
       ns)
;;

(** [fib n] is the n-th Fibonacci number, the oracle for the formula
    check. *)
let rec fib n = if n < 2 then n else fib (n - 1) + fib (n - 2)

(** [ex_5_29 ()] measures fib for n = 2 to 9, then:
    (a) the maximum depths grow by a fixed increment per n, the linear
    formula the book argues for from 1.2.2's space bound -- reported
    as [depth = n + c] with the increment verified;
    (b) the pushes satisfy [S(n) = S(n-1) + S(n-2) + k] with one
    constant k on every n, and [S(n) = a*Fib(n+1) + b] with the fitted
    a, b verified on every n. *)
let ex_5_29 () =
  let ns = [ 2; 3; 4; 5; 6; 7; 8; 9 ] in
  measure_fib ns
  >>= fun stats ->
  let depths = List.map Measured.depth_of stats in
  let pushes = List.map Measured.pushes_of stats in
  let rec consecutive_diffs = function
    | a :: (b :: _ as rest) -> (b - a) :: consecutive_diffs rest
    | _ -> []
  in
  let depth_steps = consecutive_diffs depths in
  let depth_increment = List.hd depth_steps in
  let depth_linear = List.for_all (fun s -> s = depth_increment) depth_steps in
  let arr = Array.of_list pushes in
  let ks =
    Array.to_list
      (Array.init (Array.length arr - 2) (fun i -> arr.(i + 2) - arr.(i + 1) - arr.(i)))
  in
  let k = List.hd ks in
  let k_constant = List.for_all (fun x -> x = k) ks in
  let a =
    (List.nth pushes (List.length pushes - 1) - List.nth pushes (List.length pushes - 2))
    / (fib 10 - fib 9)
  in
  let b = List.hd pushes - (a * fib 3) in
  let ab_holds = List.for_all2 (fun n p -> (a * fib (n + 1)) + b = p) ns pushes in
  let table = List.map2 (Measured.render_stats "fib") ns stats in
  let formulas =
    [ Printf.sprintf
        "maximum depth = %dn + %d (every step %d), linear: %b"
        depth_increment
        (List.hd depths - (depth_increment * List.hd ns))
        depth_increment
        depth_linear
    ; Printf.sprintf "S(n) = S(n-1) + S(n-2) + %d, k constant: %b" k k_constant
    ; Printf.sprintf
        "S(n) = %d * Fib(n+1) %s %d, holds on every measured n: %b"
        a
        (if b < 0 then "-" else "+")
        (abs b)
        ab_holds
    ]
  in
  Ok (table @ formulas)
;;
