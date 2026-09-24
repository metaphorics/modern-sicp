(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The pending scaffold of the solution with the same name under
   solutions/ch3: every entry raises the pending marker until the
   exercise is solved. *)

type semaphore =
  { acquire : unit -> unit
  ; release : unit -> unit
  }

type bounded

let make_semaphore_via_mutexes _ = raise Sicp_common.Pending.Pending_solution
let make_semaphore_via_test_and_set _ = raise Sicp_common.Pending.Pending_solution

let max_concurrent_entries _ ~workers:_ ~rounds:_ =
  raise Sicp_common.Pending.Pending_solution
;;

let ex_3_47 () = raise Sicp_common.Pending.Pending_solution
let make_bounded _ = raise Sicp_common.Pending.Pending_solution
let acquire_bounded _ = raise Sicp_common.Pending.Pending_solution
let release_bounded _ = raise Sicp_common.Pending.Pending_solution
let try_acquire_bounded _ = raise Sicp_common.Pending.Pending_solution
let ex_3_47a () = raise Sicp_common.Pending.Pending_solution
