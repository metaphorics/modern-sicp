(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.38: the multiple-dwelling puzzle without the
    Smith-Fletcher clause. The modified puzzle has several solutions;
    the demonstration enumerates them with [try_again] and reports the
    count, which the independent enumeration puts at five. *)

module Eval = Sicp_ch4.Sec_4_3
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

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
    (require (not (= (abs (- fletcher cooper)) 1)))
    (list (list 'baker baker) (list 'cooper cooper)
          (list 'fletcher fletcher) (list 'miller miller)
          (list 'smith smith))))|}
;;

(** [enumerate env exp] walks every answer of one expression with
    [try_again] and answers the values and their count. *)
let enumerate env exp =
  let first = Eval.run env exp in
  let rec go count acc =
    match Eval.try_again () with
    | Ok v -> go (count + 1) (Value.to_string v :: acc)
    | Error _ -> count + 1, List.rev acc
  in
  let count, rest = go 0 [] in
  let first_string =
    match first with
    | Ok v -> Value.to_string v
    | Error e -> "Error: " ^ Eval_error.to_string e
  in
  first_string :: rest, count
;;

let ex_4_38 () =
  let env = Eval.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) = Eval.run_program env program in
  let values, count = enumerate env "(multiple-dwelling)" in
  values @ [ "solutions=" ^ string_of_int count ]
;;
