(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.4 exercise 3.40 *)

(** Exercise 3.40: [x := !x * !x] races with [x := !x * !x * !x],
    fully unserialized. Each read of [x] on either side is its own
    event, since nothing stops the two processes from interleaving
    between them; the write on each side multiplies together whatever
    values that process actually saw. *)

(* One read or the closing write, tagged by which process it belongs
   to. Reads carry no data; the simulator just remembers how many a
   process has seen and what values they were. *)
type event =
  | R1
  | W1
  | R2
  | W2

let simulate schedule =
  let x = ref 10 in
  let seen1 = ref [] in
  let seen2 = ref [] in
  List.iter
    (function
      | R1 -> seen1 := !x :: !seen1
      | W1 -> x := List.fold_left ( * ) 1 !seen1
      | R2 -> seen2 := !x :: !seen2
      | W2 -> x := List.fold_left ( * ) 1 !seen2)
    schedule;
  !x
;;

(* Serialized, the two statements can no longer interleave with each
   other at all, so the outcome is whatever a sequential order gives;
   both orders agree here, since [(x^2)^3 = (x^3)^2 = x^6]. *)
let serialized_value () = 1_000_000

let unserialized_values () =
  Interleaving.interleavings [ [ R1; R1; W1 ]; [ R2; R2; R2; W2 ] ]
  |> List.map simulate
  |> List.sort_uniq compare
;;

let sample_unserialized_runs n =
  let one_run () =
    let x = ref 10 in
    ignore
      (Sicp_ch3.Sec_3_4.Parallel.parallel
         (fun _ -> x := !x * !x)
         (fun _ -> x := !x * !x * !x));
    !x
  in
  List.init n (fun _ -> one_run ()) |> List.sort_uniq compare
;;

let ex_3_40 () = unserialized_values (), [ serialized_value () ]
