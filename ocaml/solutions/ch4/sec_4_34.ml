(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Check = Sicp_common.Check
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module Lazy_eval = Sicp_ch4.Sec_4_2

let ( let* ) = Result.bind
let print_budget = 10

(* The lazy-printing rule: a list prints as its elements, each forced
   only as it is printed, and the walk stops with an ellipsis once the
   budget of elements is spent. *)
let rec render ~self st v =
  let* v = Lazy_eval.actual_value ~self st v in
  match Value.view v with
  | Value.Nil | Value.Cons _ ->
    let* items = elements ~self st print_budget v in
    Ok ("[" ^ items ^ "]")
  | _ -> Ok (Value.to_string v)

and elements ~self st budget cell =
  match Value.view cell with
  | Value.Cons _ when budget = 0 -> Ok "..."
  | Value.Cons (head, tail) ->
    let* head = render ~self st head in
    let* tail = Lazy_eval.actual_value ~self st tail in
    (match Value.view tail with
     | Value.Nil -> Ok head
     | _ ->
       let* rest = elements ~self st (budget - 1) tail in
       Ok (head ^ "; " ^ rest))
  | _ -> Ok ""
;;

let definitions =
  "let cons x y = x :: y\n\
   let head_or default l = match l with x :: _ -> x | [] -> default\n\
   let rec ones n = cons n (ones n)\n"
;;

(* The modified driver: the program's last binding is forced and then
   printed by the lazy printer, which forces what it prints. *)
let run result =
  let st = Lazy_eval.state () in
  let self = Lazy_eval.fix (fun ~self -> Lazy_eval.open_eval ~self st) in
  let source = definitions ^ "let result = " ^ result ^ "\n" in
  let outcome =
    match Check.check_experiment ~experiment:Check.Lazy ~filename:"ex_4_34.ml" source with
    | Error d -> Ok ("rejected: " ^ Check.diagnostic_to_string d)
    | Ok program ->
      let* v = Lazy_eval.run_with ~self st ~emit:ignore program in
      render ~self st v
  in
  match outcome with
  | Ok printed -> printed
  | Error e -> "error: " ^ Eval_error.to_string e
;;

let ex_4_34 () =
  [ run "cons 1 (cons 2 [])"
  ; run "cons (cons 1 []) (cons (cons 2 []) [])"
  ; run "ones 1"
  ; run "head_or 0 (ones 1)"
  ; run "cons 1 (cons (1 / 0) [])"
  ]
;;
