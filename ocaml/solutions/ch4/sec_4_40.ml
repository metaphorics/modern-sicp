(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.40: pruning before the restrictions. There are
    [5^5 = 3125] sets of assignments before the distinctness
    requirement and [5! = 120] after it. Generating only the
    possibilities the earlier restrictions leave open shrinks the
    search again: Baker and Fletcher are drawn from four floors each,
    Cooper from the four the bottom rule allows, and every restriction
    that mentions one or two people is imposed as soon as those people
    have floors. The demonstration pins the answer, the two counting
    identities, and the backtrack counts of the naive and the pruned
    procedures. *)

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
(define (distinct? items)
  (cond ((null? items) #t)
        ((null? (cdr items)) #t)
        ((member (car items) (cdr items)) #f)
        (else (distinct? (cdr items)))))
(define (multiple-dwelling)
  (let ((baker (amb 1 2 3 4 5))
        (cooper (amb 1 2 3 4 5))
        (fletcher (amb 1 2 3 4 5))
        (miller (amb 1 2 3 4 5))
        (smith (amb 1 2 3 4 5)))
    (require (distinct? (list baker cooper fletcher miller smith)))
    (require (not (= baker 5)))
    (require (not (= cooper 1)))
    (require (not (= fletcher 5)))
    (require (not (= fletcher 1)))
    (require (> miller cooper))
    (require (not (= (abs (- smith fletcher)) 1)))
    (require (not (= (abs (- fletcher cooper)) 1)))
    (list (list 'baker baker) (list 'cooper cooper)
          (list 'fletcher fletcher) (list 'miller miller)
          (list 'smith smith))))
(define (multiple-dwelling-pruned)
  (let ((baker (amb 1 2 3 4)))
    (let ((cooper (amb 2 3 4 5)))
      (let ((fletcher (amb 1 2 3 4)))
        (let ((miller (amb 1 2 3 4 5)))
          (let ((smith (amb 1 2 3 4 5)))
            (require (not (= baker 5)))
            (require (not (= cooper 1)))
            (require (not (= fletcher 5)))
            (require (not (= fletcher 1)))
            (require (> miller cooper))
            (require (not (= (abs (- smith fletcher)) 1)))
            (require (not (= (abs (- fletcher cooper)) 1)))
            (require (distinct? (list baker cooper fletcher miller smith)))
            (list (list 'baker baker) (list 'cooper cooper)
                  (list 'fletcher fletcher) (list 'miller miller)
                  (list 'smith smith))))))))|}
;;

let ex_4_40 () =
  let env = Eval.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) = Eval.run_program env program in
  Eval.reset_backtrack_count ();
  let naive = show (Eval.run env "(multiple-dwelling)") in
  let naive_count = "backtracks=" ^ string_of_int (Eval.backtrack_count ()) in
  Eval.reset_backtrack_count ();
  let pruned = show (Eval.run env "(multiple-dwelling-pruned)") in
  let pruned_count = "backtracks=" ^ string_of_int (Eval.backtrack_count ()) in
  [ "before distinct?: 3125"
  ; "after distinct?: 120"
  ; naive
  ; naive_count
  ; pruned
  ; pruned_count
  ]
;;
