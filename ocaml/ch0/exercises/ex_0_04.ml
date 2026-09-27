(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

type account =
  { deposit : float -> float
  ; withdraw : float -> float
  ; balance : unit -> float
  }

let make_account _initial = raise Sicp_common.Pending.Pending_solution
let make_shared_accounts _initial = raise Sicp_common.Pending.Pending_solution
