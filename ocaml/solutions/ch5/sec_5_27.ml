(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

let ( let* ) = Result.bind

module Eval = Sicp_ch5.Sec_5_4

let recursive_source n =
  Printf.sprintf
    {|let rec factorial n = if n = 1 then 1 else factorial (n - 1) * n
let v = factorial %d
|}
    n
;;

let ex_5_27 () =
  let* points =
    Sec_5_26.measure
      ~controller:Eval.base_controller
      recursive_source
      [ 1; 2; 3; 4; 5; 6 ]
  in
  Ok
    (List.map (Sec_5_26.render_point "recursive factorial") points
     @ [ Sec_5_26.formula_line "maximum depth" (Sec_5_26.depths points)
       ; Sec_5_26.formula_line "total pushes" (Sec_5_26.pushes points)
       ])
;;
