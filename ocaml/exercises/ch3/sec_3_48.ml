(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The pending scaffold of the solution with the same name under
   solutions/ch3: every entry raises the pending marker until the
   exercise is solved. *)

type numbered_account = private
  { id : int
  ; dispatch : Sicp_ch3.Sec_3_4.Account.account
  ; serializer : Sicp_ch3.Sec_3_4.Serializers.serializer
  }

let make_numbered_account _ _ = raise Sicp_common.Pending.Pending_solution
let balance_of _ = raise Sicp_common.Pending.Pending_solution
let ordered_serialized_exchange _ _ = raise Sicp_common.Pending.Pending_solution
let survives_reversed_concurrent_exchanges _ = raise Sicp_common.Pending.Pending_solution
let ex_3_48 () = raise Sicp_common.Pending.Pending_solution
