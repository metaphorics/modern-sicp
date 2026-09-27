(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.34: printing lazy pairs. The representation is tagged --
    [(cons x y)] builds [(lambda (m) (m 'lazy-pair x y))] -- so the
    printer identifies a lazy pair by applying it to a selector that
    answers the tag, and reads an arm back with a selector that answers
    [p] or [q]; neither selector forces an element. The edition's
    lazy-printing rule: a lazy pair prints as its elements, each forced
    only as it is printed, the walk continuing through lazy tails up to
    a budget of ten elements; a list still going at the budget prints an
    ellipsis, so an infinite list prints a prefix and terminates. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Reader = Sicp_common.Reader
module Value = Sicp_common.Value
module Lazy_eval = Sicp_ch4.Sec_4_2

(** The budget of the lazy printer: elements shown before the walk
    answers that the list goes on. *)
let print_budget = 10

let selector arm = Ast.lambda [ "tag"; "p"; "q" ] [ Ast.variable arm ]

module Section = struct
  let eval = Lazy_eval.eval
end

module PC = Lazy_eval.Core (Section)

(** [apply_probe env v probe] applies the candidate pair [v] to one
    selector and forces the selector's answer. *)
let apply_probe env v probe =
  probe >>= fun probe -> PC.apply_procedure v [ probe ] env >>= Lazy_eval.force_value
;;

(** [is_lazy_pair env v] holds when [v] answers the [lazy-pair] tag. *)
let is_lazy_pair env v =
  match Value.view v with
  | Value.Compound_procedure _ ->
    (match apply_probe env v (selector "tag") with
     | Ok tag ->
       (match Value.view tag with
        | Value.Symbol "lazy-pair" -> true
        | _ -> false)
     | Error _ -> false)
  | _ -> false
;;

(** [to_string eval env budget v] renders the forced value [v] under the
    lazy-printing rule. *)
let rec to_string eval env budget v =
  Lazy_eval.force_value v
  >>= fun v ->
  if is_lazy_pair env v
  then render_elements eval env budget v >>= fun inner -> Ok ("(" ^ inner ^ ")")
  else Ok (Value.to_string v)

and render_elements eval env budget v =
  if budget <= 0
  then Ok "..."
  else
    apply_probe env v (selector "p")
    >>= fun head ->
    apply_probe env v (selector "q")
    >>= fun tail ->
    to_string eval env budget head
    >>= fun head_shown ->
    Lazy_eval.force_value tail
    >>= fun tail_value ->
    if is_lazy_pair env tail_value
    then
      render_elements eval env (budget - 1) tail_value
      >>= fun rest -> Ok (head_shown ^ " " ^ rest)
    else if Value.physical_equal tail_value Value.nil
    then Ok head_shown
    else
      to_string eval env budget tail_value
      >>= fun tail_shown -> Ok (head_shown ^ " . " ^ tail_shown)
;;

let render = function
  | Ok v -> v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

(** [run env text] is the modified driver loop of the statement: one
    form, evaluated and forced, then printed by the lazy printer. *)
let run env text =
  match Reader.read text with
  | Ok exp ->
    render
      (Lazy_eval.eval exp env >>= fun v -> to_string Lazy_eval.eval env print_budget v)
  | Error e -> "Error: " ^ Reader.to_string e
;;

let tagged_pairs =
  "(define (cons x y) (lambda (m) (m 'lazy-pair x y))) (define (car z) (z (lambda (tag p \
   q) p))) (define (cdr z) (z (lambda (tag p q) q))) (define ones (cons 1 ones))"
;;

(** [ex_4_34 ()] prints finite and infinite lazy lists under the rule:
    the finite pair and the nested pair print whole, the infinite [ones]
    prints its budgeted prefix with the ellipsis, and a scalar element
    still answers as its value. *)
let ex_4_34 () =
  let env = Lazy_eval.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) = Lazy_eval.run_program env tagged_pairs in
  [ run env "(cons 1 (cons 2 '()))"
  ; run env "(cons (cons 1 '()) (cons 2 '()))"
  ; run env "ones"
  ; run env "(car ones)"
  ]
;;
