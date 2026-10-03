(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

let ( let* ) = Result.bind

module Eval = Sicp_ch5.Sec_5_4

type point =
  { n : int
  ; stats : Sec_5_23.stats
  }

let measure ~controller source ns =
  List.fold_right
    (fun n acc ->
       let* points = acc in
       let* stats = Sec_5_23.statistics ~controller (source n) in
       Ok ({ n; stats } :: points))
    ns
    (Ok [])
;;

let fit_linear samples =
  match samples, List.rev samples with
  | (n0, y0) :: _ :: _, (n1, y1) :: _ when n1 <> n0 && (y1 - y0) mod (n1 - n0) = 0 ->
    let a = (y1 - y0) / (n1 - n0) in
    Some (a, y0 - (a * n0))
  | _ -> None
;;

let holds (a, b) samples = List.for_all (fun (n, y) -> y = (a * n) + b) samples

let render_linear (a, b) =
  let slope = if a = 1 then "n" else Printf.sprintf "%dn" a in
  if b < 0 then Printf.sprintf "%s - %d" slope (-b) else Printf.sprintf "%s + %d" slope b
;;

let formula_line quantity samples =
  match fit_linear samples with
  | Some fit ->
    Printf.sprintf
      "%s = %s, holds on every measured n: %b"
      quantity
      (render_linear fit)
      (holds fit samples)
  | None -> quantity ^ " fits no linear formula"
;;

let render_point name { n; stats } =
  Printf.sprintf
    "%s n=%d: total-pushes = %d maximum-depth = %d"
    name
    n
    stats.Sec_5_23.pushes
    stats.depth
;;

let pushes points = List.map (fun { n; stats } -> n, stats.Sec_5_23.pushes) points
let depths points = List.map (fun { n; stats } -> n, stats.Sec_5_23.depth) points

let iterative_source n =
  Printf.sprintf
    {|let factorial n =
  let rec iter product counter =
    if counter > n then product else iter (counter * product) (counter + 1)
  in
  iter 1 1
let v = factorial %d
|}
    n
;;

let ex_5_26 () =
  let* points =
    measure ~controller:Eval.base_controller iterative_source [ 1; 2; 3; 4; 5; 6 ]
  in
  let depth = List.map snd (depths points) in
  let constant =
    match depth with
    | d :: rest when List.for_all (( = ) d) rest ->
      Printf.sprintf "maximum depth: %d, independent of n = true" d
    | _ -> "maximum depth: varies with n, independent of n = false"
  in
  Ok
    (List.map (render_point "iterative factorial") points
     @ [ constant; formula_line "total pushes" (pushes points) ])
;;
