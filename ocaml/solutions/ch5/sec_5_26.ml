(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

(** Exercise 5.26: the monitored stack explores the evaluator's
    tail-recursive property with the iterative factorial of 1.2.1.

    The monitored driver is the 5.4.4 variant: [print-result] performs
    [print-stack-statistics] before announcing the value, and the
    driver initializes the stack once per interaction, so every
    interaction's counters are its own.  The exercises of this unit
    that measure the stack (5.26 to 5.29) share this module's
    controller and helpers. *)

let ( >>= ) = Result.bind

module Eval = Sicp_ch5.Sec_5_4

(** The monitored driver fragment: stats printed before the value. *)
let monitored_driver =
  {|read-eval-print-loop
  (perform (op initialize-stack))
  (perform (op prompt-for-input))
  (assign exp (op read))
  (assign env (op get-global-environment))
  (assign continue (label print-result))
  (goto (label eval-dispatch))
print-result
  (perform (op print-stack-statistics))
  (perform (op announce-output))
  (perform (op user-print) (reg val))
  (goto (label read-eval-print-loop))|}
;;

(** The base controller with the monitored driver. *)
let controller =
  String.concat
    "\n"
    (List.map
       (fun (name, text) -> if name = "driver" then monitored_driver else text)
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

(** One interaction's measured counters. *)
type stats =
  { pushes : int
  ; depth : int
  }

let pushes_of { pushes; _ } = pushes
let depth_of { depth; _ } = depth

(** [parse_stats line] reads a [(total-pushes = P maximum-depth = D)]
    line the machine printed. *)
let parse_stats line =
  let number_at start =
    let len = String.length line in
    let rec go k =
      if k < len && line.[k] >= '0' && line.[k] <= '9' then go (k + 1) else k
    in
    let stop = go start in
    if stop = start
    then None
    else int_of_string_opt (String.sub line start (stop - start))
  in
  let total_pushes = String.length "(total-pushes = " in
  match () with
  | _ when String.starts_with ~prefix:"(total-pushes = " line ->
    (match number_at total_pushes with
     | Some p
       when String.starts_with
              ~prefix:(string_of_int p ^ " maximum-depth = ")
              (String.sub line total_pushes (String.length line - total_pushes)) ->
       let depth_at =
         total_pushes
         + String.length (string_of_int p)
         + String.length " maximum-depth = "
       in
       Option.map (fun d -> { pushes = p; depth = d }) (number_at depth_at)
     | _ -> None)
  | _ -> None
;;

(** [stats_of transcript] is the measured counters of every
    interaction that printed one, in order. *)
let stats_of transcript =
  List.filter_map
    (fun line ->
       if String.starts_with ~prefix:"(total-pushes" line then parse_stats line else None)
    transcript
;;

(** [fit_linear ns ps] is the [a], [b] of [p(n) = an + b] through the
    first and last point, or [None] for a single point. *)
let fit_linear ns ps =
  match ns, ps with
  | n0 :: _, p0 :: _ ->
    let n1 = List.nth ns (List.length ns - 1) in
    let p1 = List.nth ps (List.length ps - 1) in
    if n1 = n0
    then None
    else Some ((p1 - p0) / (n1 - n0), p0 - ((p1 - p0) / (n1 - n0) * n0))
  | _ -> None
;;

let iterative_source =
  {|(define (factorial n)
  (define (iter product counter)
    (if (> counter n)
        product
        (iter (* counter product)
              (+ counter 1))))
  (iter 1 1))|}
;;

(** [measure source ns] runs the program's calls [(factorial n)] for
    each [n] on a fresh machine and answers the counters. *)
let measure source ns =
  Eval.result_all
    (List.map
       (fun n ->
          run (source ^ "\n(factorial " ^ string_of_int n ^ ")")
          >>= fun lines ->
          match List.rev (stats_of lines) with
          | s :: _ -> Ok s
          | [] -> Error (Eval.Op_failed "the call printed no stack statistics"))
       ns)
;;

let render_stats name n { pushes; depth } =
  name
  ^ " n="
  ^ string_of_int n
  ^ ": total-pushes = "
  ^ string_of_int pushes
  ^ " maximum-depth = "
  ^ string_of_int depth
;;

(** [ex_5_26 ()] measures the iterative factorial for n = 1 to 6 and
    answers the table plus the two answers: the maximum depth is
    independent of n, and the pushes fit [an + b] with the fitted
    constants verified on every point. *)
let ex_5_26 () =
  let ns = [ 1; 2; 3; 4; 5; 6 ] in
  measure iterative_source ns
  >>= fun stats ->
  let depths = List.map (fun s -> s.depth) stats in
  let pushes = List.map (fun s -> s.pushes) stats in
  let depth_constant = List.for_all (fun d -> d = List.hd depths) depths in
  match fit_linear ns pushes with
  | Some (a, b) ->
    let linear_holds = List.for_all2 (fun n p -> (a * n) + b = p) ns pushes in
    let table = List.map2 (render_stats "iterative factorial") ns stats in
    let answers =
      [ "maximum depth: "
        ^ (if depth_constant then string_of_int (List.hd depths) else "not constant")
        ^ ", independent of n = "
        ^ string_of_bool depth_constant
      ; Printf.sprintf
          "total pushes = %dn %s %d, holds on every measured n: %b"
          a
          (if b < 0 then "-" else "+")
          (abs b)
          linear_holds
      ]
    in
    Ok (table @ answers)
  | None -> Error (Eval.Op_failed "expected at least two measurements")
;;
