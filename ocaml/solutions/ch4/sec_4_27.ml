(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Check = Sicp_common.Check

(* The top level of the experiment forces every binding, so the
   interaction keeps the text's unforced [define] in a tuple component:
   a component holds the value [id (id 10)] evaluates to, not a forced
   one, exactly as the text's [define] binds [w]. *)
let interaction =
  "let count = ref 0\n\
   let id x = count := !count + 1; x\n\
   let defined = (id (id 10), ())\n\
   let () = print_int !count; print_newline ()\n\
   let () = match defined with (w, _) -> print_int w; print_newline ()\n\
   let () = print_int !count; print_newline ()\n\
   let () = match defined with (w, _) -> print_int w; print_newline ()\n\
   let () = print_int !count; print_newline ()\n"
;;

let ex_4_27 () =
  Sicp_ch4.Sec_4_1.transcript ~experiment:Check.Lazy Sicp_ch4.Sec_4_2.run interaction
  |> String.split_on_char '\n'
  |> List.filter (fun line -> line <> "")
;;
