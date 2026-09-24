(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.74 *)

(** Exercise 3.74: Alyssa's zero crossings, completed with the
    generalized stream-map of exercise 3.50 mapping the detector over
    the signal and the signal shifted by one. The map class is [T]. *)

open Sicp_ch3.Sec_3_5

(* The sign of a 0 input is positive, as the statement requires. *)
let sign value = if value < 0.0 then -1 else 1

let sign_change_detector value last_value =
  if sign value = sign last_value then 0 else sign value
;;

let zero_crossings sense_data =
  Sec_3_50.stream_map_multi
    (function
      | [ value; last_value ] -> sign_change_detector value last_value
      | _ -> invalid_arg "zero_crossings: pairs of signal values")
    [ sense_data; Streams.cons_stream 0.0 (fun () -> sense_data) ]
;;

let rec ones_float = Streams.Cons (1.0, lazy ones_float)

(* The statement's sample signal, with a positive tail after the ellipsis. *)
let sense_data =
  List.fold_right
    (fun x acc -> Streams.cons_stream x (fun () -> acc))
    [ 1.0; 2.0; 1.5; 1.0; 0.5; -0.1; -2.0; -3.0; -2.0; -0.5; 0.2; 3.0; 4.0 ]
    ones_float
;;

let ex_3_74 () = Streams.stream_take 12 (zero_crossings sense_data)
