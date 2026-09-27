(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.1 exercise 4.21 *)

(** Exercise 4.21: recursion without [define] or [letrec]. Each
    procedure receives itself as an argument and calls that argument
    for the recursive step, so the plain base evaluator, which has no
    recursion support at all, runs both demonstrations. *)

let ( >>= ) = Result.bind

module Eval_error = Sicp_common.Eval_error
module Reader = Sicp_common.Reader
module Value = Sicp_common.Value
module SE = Sicp_ch4.Sec_4_1

(** [eval] is the base evaluator: the demonstrations are pure lambda
    applications the shared Reader parses directly. *)
let eval : SE.eval_t = SE.eval

let render = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

let run text =
  let env = SE.the_global_environment () in
  Reader.read_program text
  |> Result.map_error (fun e -> Eval_error.Invalid_form (Reader.to_string e))
  >>= fun exps ->
  let rec go = function
    | [] -> Ok (Value.symbol "ok")
    | [ exp ] -> eval exp env
    | exp :: rest -> eval exp env >>= fun _ -> go rest
  in
  go exps
;;

(** [ex_4_21 ()] evaluates the statement's self-application trick as a
      Fibonacci procedure on [10] -- part (a)'s analog of the
      factorial -- and part (b)'s mutually recursive even?/odd? pair
      without definitions, applied to [4]. *)
let ex_4_21 () =
  let fib =
    run
      {|
((lambda (n)
   ((lambda (fib) (fib fib n))
    (lambda (ft k)
      (if (< k 2)
          k
          (+ (ft ft (- k 1)) (ft ft (- k 2)))))))
 10)
|}
  in
  let parity =
    run
      {|
((lambda (even? odd?)
   (even? even? odd? 4))
 (lambda (ev? od? n)
   (if (= n 0) true (od? ev? od? (- n 1))))
 (lambda (ev? od? n)
   (if (= n 0) false (ev? ev? od? (- n 1)))))
|}
  in
  [ render fib; render parity ]
;;
