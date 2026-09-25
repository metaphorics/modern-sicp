(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

(** Exercise 5.27: the recursive factorial on the monitored stack, for
    comparison with 5.26; both depth and pushes are linear in n, and
    the measured constants fill the book's table. *)

let ( >>= ) = Result.bind

module Eval = Sicp_ch5.Sec_5_4
module Measured = Sec_5_26

let recursive_source =
  {|(define (factorial n)
  (if (= n 1)
      1
      (* (factorial (- n 1)) n)))|}
;;

(** [ex_5_27 ()] measures the recursive factorial for n = 1 to 6, fits
    both counters, and verifies the linear formulas on every point.
    The maximum depth of 5.26's iterative version is constant, this
    one's grows by a fixed amount per n -- the book's table's two
    rows. *)
let ex_5_27 () =
  let ns = [ 1; 2; 3; 4; 5; 6 ] in
  Measured.measure recursive_source ns
  >>= fun stats ->
  let depths = List.map Measured.depth_of stats in
  let pushes = List.map Measured.pushes_of stats in
  match Measured.fit_linear ns depths, Measured.fit_linear ns pushes with
  | Some (ad, bd), Some (ap, bp) ->
    let depth_holds = List.for_all2 (fun n d -> (ad * n) + bd = d) ns depths in
    let pushes_holds = List.for_all2 (fun n p -> (ap * n) + bp = p) ns pushes in
    let table = List.map2 (Measured.render_stats "recursive factorial") ns stats in
    let formulas =
      [ Printf.sprintf
          "maximum depth = %dn %s %d, holds on every measured n: %b"
          ad
          (if bd < 0 then "-" else "+")
          (abs bd)
          depth_holds
      ; Printf.sprintf
          "total pushes = %dn %s %d, holds on every measured n: %b"
          ap
          (if bp < 0 then "-" else "+")
          (abs bp)
          pushes_holds
      ]
    in
    Ok (table @ formulas)
  | _ -> Error (Eval.Op_failed "expected at least two measurements")
;;
