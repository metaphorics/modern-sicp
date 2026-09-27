(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.1 exercise 3.6 *)

(** Exercise 3.6: a [rand] that can be reset to reproduce a sequence.
    Scheme's [((rand 'reset) new-value)] two-level application
    flattens into one [Reset new_value] message, since a variant
    payload carries the value [rand 'reset] would otherwise need a
    second application just to receive. *)

type rand_message =
  | Generate
  | Reset of int64

let bound = 1_000_000_000

let fresh_generator seed =
  match Sicp_common.Random.create seed with
  | Ok generator -> generator
  | Error Sicp_common.Error.Zero_seed -> failwith "rand: seed must be nonzero"
;;

let make_rand seed =
  let generator = ref (fresh_generator seed) in
  fun message ->
    match message with
    | Generate -> Some (Sicp_common.Random.random !generator bound)
    | Reset new_seed ->
      generator := fresh_generator new_seed;
      None
;;

let ex_3_06 () =
  let rand = make_rand 7L in
  let draw () =
    match rand Generate with
    | Some value -> value
    | None -> assert false
  in
  let first = draw () in
  ignore (rand (Reset 99L));
  ignore (draw ());
  ignore (rand (Reset 7L));
  let third = draw () in
  first, third
;;
