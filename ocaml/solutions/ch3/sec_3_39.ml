(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.4 exercise 3.39 *)

(** Exercise 3.39: which of the text's five outcomes remain when
    [x := s.protect (fun () -> !x * !x)] runs concurrently with the
    fully serialized [s.protect (fun () -> incr x)]. Both reads of [x]
    for the square happen inside the same critical section, so they
    can no longer be split by an interleaved write; but the
    assignment [x := ...] itself runs outside the serializer, so it
    can still race with the increment. *)

open Sicp_ch3.Sec_3_4

(* The four events, one read and one write per process. Both reads of
   [x] the squaring needs collapse into one [Read1], since they run
   inside the same critical section and always see the same value. *)
type op =
  | Read1
  | Write1
  | Read2
  | Write2

let simulate schedule =
  let x = ref 10 in
  let seen1 = ref 0 in
  let seen2 = ref 0 in
  List.iter
    (function
      | Read1 -> seen1 := !x
      | Write1 -> x := !seen1 * !seen1
      | Read2 -> seen2 := !x
      | Write2 -> x := !seen2 + 1)
    schedule;
  !x
;;

let possibilities_before = [ 101; 121; 110; 11; 100 ]

(* The only schedules the mutex allows: P1's read and P2's whole
   read-then-write pair cannot interleave with each other (both are
   inside the serializer), but P1's own unserialized write can land
   before, inside, or after P2's pair. *)
let schedules =
  [ [ Read1; Write1; Read2; Write2 ] (* P1 first, its write lands before P2 *)
  ; [ Read1; Read2; Write1; Write2 ] (* P1's write lands inside P2's pair *)
  ; [ Read1; Read2; Write2; Write1 ] (* P1's write lands after P2 *)
  ; [ Read2; Write2; Read1; Write1 ] (* P2 entirely first *)
  ]
;;

let possibilities_after () = List.map simulate schedules |> List.sort_uniq compare

let race () =
  let x = ref 10 in
  let s = Serializers.make_serializer () in
  ignore
    (Parallel.parallel
       (fun _ -> x := s.protect (fun () -> !x * !x))
       (fun _ -> s.protect (fun () -> incr x)));
  !x
;;

let sample_runs n = List.init n (fun _ -> race ()) |> List.sort_uniq compare
let ex_3_39 () = possibilities_after ()
