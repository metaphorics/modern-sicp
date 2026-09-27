(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.53: [permanent-set!] under [if-fail]. The inner
    [prime-sum-pair] choice points backtrack through their
    alternatives while the trailing [(amb)] fails, so every prime-sum
    pair of the two lists is consed onto [pairs] and survives; when
    the combinations are spent, the failure reaches [if-fail], which
    succeeds with the accumulated list. The result is the three pairs
    in reverse discovery order, and the demonstration pins it together
    with the exhaustion that follows. *)

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
(define (an-element-of items)
  (require (not (null? items)))
  (amb (car items) (an-element-of (cdr items))))
(define (prime? n)
  (define (divides? d) (= 0 (remainder n d)))
  (define (find-divisor d)
    (if (> (* d d) n) n (if (divides? d) d (find-divisor (+ d 1)))))
  (if (< n 2) #f (= n (find-divisor 2))))
(define (quotient x y) (if (< x y) 0 (+ 1 (quotient (- x y) y))))
(define (remainder x y) (- x (* y (quotient x y))))
(define (prime-sum-pair list1 list2)
  (let ((a (an-element-of list1)) (b (an-element-of list2)))
    (require (prime? (+ a b)))
    (list a b)))|}
;;

let demo =
  {|(let ((pairs '()))
  (if-fail
   (let ((p (prime-sum-pair '(1 3 5 8) '(20 35 110))))
     (permanent-set! pairs (cons p pairs))
     (amb))
   pairs))|}
;;

let ex_4_53 () =
  let env = Eval.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) = Eval.run_program env program in
  let first = Eval.run env demo in
  let again = Eval.try_again () in
  List.map show [ first; again ]
;;
