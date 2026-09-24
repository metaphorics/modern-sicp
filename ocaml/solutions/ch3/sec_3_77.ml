(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.77 *)

(** Exercise 3.77: [integral] in the integers-starting-from style,
    made to expect a delayed integrand so it can sit in feedback
    loops. The map class is [T]. *)

open Sicp_ch3.Sec_3_5

let rec integral delayed_integrand initial_value dt =
  Streams.Cons
    ( initial_value
    , lazy
        (let integrand = Lazy.force delayed_integrand in
         if Streams.stream_null integrand
         then Streams.the_empty_stream
         else
           integral
             (lazy (Streams.stream_cdr integrand))
             ((dt *. Streams.stream_car integrand) +. initial_value)
             dt) )
;;

let solve f y0 dt =
  let rec y_stream () = integral (lazy (Streams.stream_map f (y_stream ()))) y0 dt in
  y_stream ()
;;

let ex_3_77 () =
  let e_approximation = Streams.stream_ref (solve (fun y -> y) 1.0 0.001) 1000 in
  let over_empty =
    Streams.stream_take 2 (integral (lazy Streams.the_empty_stream) 5.0 0.1)
  in
  e_approximation, over_empty
;;
