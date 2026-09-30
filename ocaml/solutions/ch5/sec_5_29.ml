(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

let ( let* ) = Result.bind

module Eval = Sicp_ch5.Sec_5_4

let fib_source n =
  Printf.sprintf
    {|let rec fib n = if n < 2 then n else fib (n - 1) + fib (n - 2)
let v = fib %d
|}
    n
;;

let fib n =
  let rec go a b k = if k = 0 then a else go b (a + b) (k - 1) in
  go 0 1 n
;;

let recurrence_constants pushes =
  List.filter_map
    (fun (n, s) ->
       match List.assoc_opt (n - 1) pushes, List.assoc_opt (n - 2) pushes with
       | Some s1, Some s2 -> Some (s - s1 - s2)
       | _ -> None)
    pushes
;;

let depth_line depths =
  let steps =
    List.filter_map
      (fun (n, d) -> Option.map (fun d0 -> d - d0) (List.assoc_opt (n - 1) depths))
      depths
  in
  match steps, Sec_5_26.fit_linear depths with
  | step :: rest, Some fit ->
    Printf.sprintf
      "maximum depth = %s (every step %d), linear: %b"
      (Sec_5_26.render_linear fit)
      step
      (List.for_all (( = ) step) rest && Sec_5_26.holds fit depths)
  | _ -> "maximum depth: too few measurements"
;;

let recurrence_line pushes =
  match recurrence_constants pushes with
  | k :: rest ->
    Printf.sprintf
      "S(n) = S(n-1) + S(n-2) + %d, k constant: %b"
      k
      (List.for_all (( = ) k) rest)
  | [] -> "S(n): too few measurements"
;;

let closed_form_line pushes =
  let by_fib = List.map (fun (n, s) -> fib (n + 1), s) pushes in
  match Sec_5_26.fit_linear by_fib with
  | Some ((a, b) as fit) ->
    Printf.sprintf
      "S(n) = %d * Fib(n+1) %s %d, holds on every measured n: %b"
      a
      (if b < 0 then "-" else "+")
      (abs b)
      (Sec_5_26.holds fit by_fib)
  | None -> "S(n) = a * Fib(n+1) + b fits no integral a"
;;

let ex_5_29 () =
  let* points =
    Sec_5_26.measure
      ~controller:Eval.base_controller
      fib_source
      [ 2; 3; 4; 5; 6; 7; 8; 9 ]
  in
  let pushes = Sec_5_26.pushes points in
  Ok
    (List.map (Sec_5_26.render_point "fib") points
     @ [ depth_line (Sec_5_26.depths points)
       ; recurrence_line pushes
       ; closed_form_line pushes
       ])
;;
