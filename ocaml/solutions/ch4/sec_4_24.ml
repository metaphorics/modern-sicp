(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.1 exercise 4.24 *)

(** Exercise 4.24: the analyzed evaluator against the direct one. The
    benchmark program is the recursive [(fib n)]; the direct evaluator
    evaluates the call on each of [iterations] iterations, while the
    analyzed one analyzes the call once and runs the resulting execution
    procedure on the same number of iterations. Each side takes one
    untimed warm-up run and five measured runs, paired run by run; the
    reported time is the median, in CPU milliseconds read with
    [Sys.time]. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Reader = Sicp_common.Reader
module Value = Sicp_common.Value

(** The benchmark program and the size of its call: [(fib 11)] makes 287
    procedure calls, and [iterations] of them per measured run keep a
    run well above the clock's resolution while the five direct runs
    together stay in the few-hundred-millisecond range. *)
let fib_definition = "(define (fib n) (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2)))))"

let fib_n = 11
let iterations = 150

(** How many measured runs the median is taken over. *)
let runs = 5

(** The call both evaluators run: [(fib 11)]. *)
let call = Ast.application (Ast.variable "fib") [ Ast.int fib_n ]

(** [median samples] is the middle sample of the sorted [samples]. *)
let median samples =
  let sorted = List.sort compare samples in
  List.nth sorted (List.length sorted / 2)
;;

(** [time_loop run] runs [run ()] [iterations] times and answers the
    elapsed CPU time in milliseconds. *)
let time_loop run =
  let start = Sys.time () in
  for _ = 1 to iterations do
    let (_ : (Value.t, Eval_error.t) result) = run () in
    ()
  done;
  (Sys.time () -. start) *. 1000.0
;;

(** [timings ()] is [(direct_median, analyzed_median)] in milliseconds.
    The program is defined once per evaluator in its own fresh global
    environment -- the analyzed side must define [fib] through the
    analyzed evaluator, or its procedure body is never registered --
    then both sides warm up over one untimed run of the loop, and five
    direct-plus-analyzed run pairs follow; each side's median is the
    answer. The analyzed call is analyzed once, outside every timed
    region. *)
let timings () =
  let direct_env = Sicp_ch4.Sec_4_1.the_global_environment () in
  let analyzed_env = Sicp_ch4.Sec_4_1.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) =
    Sicp_ch4.Sec_4_1.run direct_env fib_definition
  in
  let (_ : (Value.t, Eval_error.t) result) =
    Reader.read fib_definition
    |> Result.map_error (fun e -> Eval_error.Invalid_form (Reader.to_string e))
    >>= fun exp -> Sicp_ch4.Sec_4_1.Analyze.eval exp analyzed_env
  in
  let analyzed =
    match Sicp_ch4.Sec_4_1.Analyze.analyze call with
    | Ok exec -> exec
    | Error e -> fun _ -> Error e
  in
  let direct_run () = Sicp_ch4.Sec_4_1.eval call direct_env in
  let analyzed_run () = analyzed analyzed_env in
  let (_ : float) = time_loop direct_run in
  let (_ : float) = time_loop analyzed_run in
  let pairs =
    List.init runs (fun _ ->
      let d = time_loop direct_run in
      d, time_loop analyzed_run)
  in
  let directs, analyzeds = List.split pairs in
  median directs, median analyzeds
;;

(** [ex_4_24 ()] is the demonstration: both medians to one decimal and
    the ratio by which the analyzed evaluator's execution beats the
    direct evaluator. *)
let ex_4_24 () =
  let direct, analyzed = timings () in
  [ Printf.sprintf "direct median: %.1f ms" direct
  ; Printf.sprintf "analyzed median: %.1f ms" analyzed
  ; Printf.sprintf "analyzed wins by %.1fx" (direct /. analyzed)
  ]
;;
