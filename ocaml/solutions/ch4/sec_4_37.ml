(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.37: Ben's generator. Ben computes the square root after
    [j] and prunes with [hsq], so on the way to the first triple the
    evaluator delivers fewer [require] failures than the 4.35 order.
    The backtrack counter of 4.44a measures both runs on the same
    bounds; Ben is correct about the size of the search, and the
    demonstration pins the shared first triple and the two counts. *)

module Eval = Sicp_ch4.Sec_4_3
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

let show = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

let program =
  {|
(define (require p) (if (not p) (amb)))
(define (an-integer-between low high)
  (require (not (> low high)))
  (amb low (an-integer-between (+ low 1) high)))
(define (a-pythagorean-triple-between low high)
  (let ((i (an-integer-between low high)))
    (let ((j (an-integer-between i high)))
      (let ((k (an-integer-between j high)))
        (require (= (+ (* i i) (* j j)) (* k k)))
        (list i j k)))))
(define (a-pythagorean-triple-between-ben low high)
  (let ((i (an-integer-between low high)) (hsq (* high high)))
    (let ((j (an-integer-between i high)))
      (let ((ksq (+ (* i i) (* j j))))
        (require (>= hsq ksq))
        (let ((k (sqrt ksq)))
          (require (integer? k))
          (list i j k))))))|}
;;

let ex_4_37 () =
  let env = Eval.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) = Eval.run_program env program in
  Eval.reset_backtrack_count ();
  let plain = show (Eval.run env "(a-pythagorean-triple-between 1 20)") in
  let plain_count = "backtracks=" ^ string_of_int (Eval.backtrack_count ()) in
  Eval.reset_backtrack_count ();
  let ben = show (Eval.run env "(a-pythagorean-triple-between-ben 1 20)") in
  let ben_count = "backtracks=" ^ string_of_int (Eval.backtrack_count ()) in
  [ plain; plain_count; ben; ben_count ]
;;
