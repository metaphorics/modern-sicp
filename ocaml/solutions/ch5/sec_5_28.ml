(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

(** Exercise 5.28: the evaluator with its tail-recursion removed -- the
    naive [ev-sequence] of the 5.4.2 footnote, where every expression
    of a sequence is evaluated across a saved [continue] -- rerunning
    the measurements of 5.26 and 5.27 to show both factorial versions
    now demand space that grows with n. *)

let ( >>= ) = Result.bind

module Eval = Sicp_ch5.Sec_5_4
module Measured = Sec_5_26

(** The naive sequence evaluation of the 5.4.2 footnote: no expression
    is in tail position, so a tail call pushes. *)
let naive_ev_sequence =
  {|ev-sequence
  (test (op no-more-exps?) (reg unev))
  (branch (label ev-sequence-end))
  (assign exp (op first-exp) (reg unev))
  (save unev)
  (save env)
  (assign continue
          (label ev-sequence-continue))
  (goto (label eval-dispatch))
ev-sequence-continue
  (restore env)
  (restore unev)
  (assign unev
          (op rest-exps)
          (reg unev))
  (goto (label ev-sequence))
ev-sequence-end
  (restore continue)
  (goto (reg continue))|}
;;

(** The monitored controller with the naive sequence evaluation. *)
let controller =
  String.concat
    "\n"
    (List.map
       (fun (name, text) ->
          match name with
          | "driver" -> Measured.monitored_driver
          | "ev-sequence" -> naive_ev_sequence
          | _ -> text)
       Eval.controller_fragments)
;;

let run source =
  Eval.make_evaluator ~controller ~source ()
  >>= fun m ->
  let ended =
    match Eval.start m with
    | Ok () -> Ok ()
    | Error e when Eval.error_to_string e = "operation failed: " ^ Eval.input_exhausted ->
      Ok ()
    | Error e -> Error e
  in
  ended >>= fun () -> Ok (Eval.transcript m)
;;

(** [ex_5_28 ()] reruns the 5.26 and 5.27 experiments on the
    non-tail-recursive evaluator: the iterative factorial's maximum
    depth, constant under the book's evaluator, now grows linearly,
    and the recursive factorial's depth keeps growing too -- both rows
    of the book's demonstration. *)
let ex_5_28 () =
  let ns = [ 1; 2; 3; 4; 5 ] in
  let measure source =
    Eval.result_all
      (List.map
         (fun n ->
            run (source ^ "\n(factorial " ^ string_of_int n ^ ")")
            >>= fun lines ->
            match List.rev (Measured.stats_of lines) with
            | s :: _ -> Ok s
            | [] -> Error (Eval.Op_failed "the call printed no stack statistics"))
         ns)
  in
  Measured.iterative_source
  |> measure
  >>= fun iterative ->
  Sec_5_27.recursive_source
  |> measure
  >>= fun recursive ->
  let table name stats = List.map2 (fun n s -> Measured.render_stats name n s) ns stats in
  let depths = List.map Measured.depth_of iterative in
  let rec increasing = function
    | a :: (b :: _ as rest) -> b > a && increasing rest
    | _ -> true
  in
  let grows = increasing depths in
  Ok
    (table "non-tail iterative factorial" iterative
     @ table "non-tail recursive factorial" recursive
     @ [ "iterative maximum depth now grows with n: " ^ string_of_bool grows ])
;;
