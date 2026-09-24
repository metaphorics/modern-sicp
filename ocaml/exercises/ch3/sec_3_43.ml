(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The pending scaffold of the solution with the same name under
   solutions/ch3: every entry raises the pending marker until the
   exercise is solved. *)

type raw_account = private
  { dispatch : Sicp_ch3.Sec_3_4.Account.account
  ; serializer : Sicp_ch3.Sec_3_4.Serializers.serializer
  }

let make_raw_account _ = raise Sicp_common.Pending.Pending_solution
let balance_of _ = raise Sicp_common.Pending.Pending_solution
let exchange _ _ = raise Sicp_common.Pending.Pending_solution
let serialized_exchange _ _ = raise Sicp_common.Pending.Pending_solution

let multiset_preserved_by_serialized_exchange _ _ =
  raise Sicp_common.Pending.Pending_solution
;;

let sum_preserved_by_plain_exchange _ _ = raise Sicp_common.Pending.Pending_solution
let ex_3_43 () = raise Sicp_common.Pending.Pending_solution
