(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.51 *)

(** Exercise 3.51: [show] prints its argument and returns it, and the
    transcript reveals when memoized [delay] evaluates what. The map
    class is [T]: printing becomes appending to a log the exercise
    answers, so the transcript is data rather than console noise. The
    tailored addition 3.51a counts force operations explicitly. *)

open Sicp_ch3.Sec_3_5

let ex_3_51 () =
  let log = ref [] in
  let show v =
    log := v :: !log;
    v
  in
  let x = Streams.stream_map show (Streams.stream_enumerate_interval 0 10) in
  let after_definition = List.rev !log in
  ignore (Streams.stream_ref x 5);
  let after_ref_5 = List.rev !log in
  ignore (Streams.stream_ref x 7);
  let after_ref_7 = List.rev !log in
  after_definition, after_ref_5, after_ref_7
;;

(** 3.51a: the same construction over an instrumented delay. Each tail
    is built with one counted memoized promise; [forces] counts every
    access and [bodies] counts how often a tail thunk actually ran.
    Memoization is the gap between the two. *)
type counters =
  { mutable forces : int
  ; mutable bodies : int
  }

let counted_cons ctr head tail =
  Streams.Cons
    ( head
    , lazy
        (ctr.bodies <- ctr.bodies + 1;
         tail ()) )
;;

let counted_cdr ctr = function
  | Streams.Cons (_, tail) ->
    ctr.forces <- ctr.forces + 1;
    Lazy.force tail
  | Streams.Empty -> invalid_arg "counted_cdr: the empty stream"
;;

let rec counted_ref ctr s n =
  if n = 0 then Streams.stream_car s else counted_ref ctr (counted_cdr ctr s) (n - 1)
;;

let ex_3_51a () =
  let ctr = { forces = 0; bodies = 0 } in
  let log = ref [] in
  let show v =
    log := v :: !log;
    v
  in
  let rec counted_map s =
    counted_cons
      ctr
      (show (Streams.stream_car s))
      (fun () -> counted_map (Streams.stream_cdr s))
  in
  let x = counted_map (Streams.stream_enumerate_interval 0 10) in
  let shows_after_definition = List.length !log in
  ignore (counted_ref ctr x 5);
  let shows_after_ref_5 = List.length !log in
  ignore (counted_ref ctr x 7);
  let shows_after_ref_7 = List.length !log in
  shows_after_definition, shows_after_ref_5, shows_after_ref_7, ctr.forces, ctr.bodies
;;
