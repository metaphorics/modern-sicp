(* SPDX-License-Identifier: GPL-3.0-only *)

module Check = Sicp_common.Check
module Eval_error = Sicp_common.Eval_error
module Sec_4_1 = Sicp_ch4.Sec_4_1
module Sec_4_2 = Sicp_ch4.Sec_4_2
module Sec_4_3 = Sicp_ch4.Sec_4_3
module Sec_4_4 = Sicp_ch4.Sec_4_4
module Sec_5_2 = Sicp_ch5.Sec_5_2
module Sec_5_4 = Sicp_ch5.Sec_5_4

(** The OCaml edition's teaching conformance driver.

    Usage: [sicp_run --case CASE --engine ENGINE --source PATH].  One
    invocation reports exactly one observation on stdout as a single
    JSON object with the fields [termination] (one of [value], [error],
    [rejected]) and [stdout] (the exact observation transcript).  All
    diagnostics go to stderr.  Exit status is zero exactly when an
    observation is reported; a bad request or an internal failure exits
    nonzero.

    Engine names: [direct], [analyzed], [eceval], and [compiled] run a
    checked guest program of {source} through the four teaching engines;
    [lazy] and [search] run the named experiments of grammar section 10;
    [query] and [machine] run the domain fixtures of {source}; and
    [reference] runs the independent finite model beside the relevant
    teaching engine.  A rejected source produces no guest effect: its
    transcript is empty. *)
module Sec_5_5 = Sicp_ch5.Sec_5_5

let usage_error message =
  Printf.eprintf "sicp_run: %s\n" message;
  Printf.eprintf
    "usage: sicp_run --case CASE --engine ENGINE --source PATH [--work DIR]\n";
  exit 2
;;

let json_string s =
  let buf = Buffer.create (String.length s + 8) in
  Buffer.add_char buf '"';
  String.iter
    (fun c ->
       match c with
       | '"' -> Buffer.add_string buf "\\\""
       | '\\' -> Buffer.add_string buf "\\\\"
       | '\n' -> Buffer.add_string buf "\\n"
       | '\r' -> Buffer.add_string buf "\\r"
       | '\t' -> Buffer.add_string buf "\\t"
       | '\008' -> Buffer.add_string buf "\\b"
       | '\012' -> Buffer.add_string buf "\\f"
       | c when Char.code c < 0x20 ->
         Buffer.add_string buf (Printf.sprintf "\\u%04x" (Char.code c))
       | c -> Buffer.add_char buf c)
    s;
  Buffer.add_char buf '"';
  Buffer.contents buf
;;

let report ~termination ~stdout =
  Printf.printf
    "{\"termination\": %s, \"stdout\": %s}\n"
    (json_string termination)
    (json_string stdout);
  flush Stdlib.stdout
;;

let read_file path =
  let channel = open_in_bin path in
  Fun.protect
    ~finally:(fun () -> close_in channel)
    (fun () -> really_input_string channel (in_channel_length channel))
;;

let rejected detail =
  Printf.eprintf "sicp_run: rejected: %s\n" detail;
  report ~termination:"rejected" ~stdout:""
;;

let runtime_error transcript detail =
  Printf.eprintf "sicp_run: runtime error: %s\n" detail;
  report ~termination:"error" ~stdout:transcript
;;

let value transcript = report ~termination:"value" ~stdout:transcript

(* Checked guest-program engines share one admission and observation
   shape; only the evaluator differs. *)

let run_program ~check ~eval source_path =
  let source = read_file source_path in
  match check source with
  | Error diagnostic -> rejected (Check.diagnostic_to_string diagnostic)
  | Ok program ->
    let transcript = Buffer.create 256 in
    (match eval ~emit:(Buffer.add_string transcript) program with
     | Ok _ -> value (Buffer.contents transcript)
     | Error e -> runtime_error (Buffer.contents transcript) (Eval_error.to_string e))
;;

let run_core ~eval source_path =
  run_program ~check:(Check.check ~filename:source_path) ~eval source_path
;;

let run_lazy source_path =
  run_program
    ~check:(Check.check_experiment ~experiment:Lazy ~filename:source_path)
    ~eval:Sec_4_2.run
    source_path
;;

let run_search source_path =
  run_program
    ~check:(Check.check_experiment ~experiment:Search ~filename:source_path)
    ~eval:Sec_4_3.run
    source_path
;;

let run_query source_path =
  match Sec_4_4.read_fixture ~filename:source_path (read_file source_path) with
  | Error detail -> rejected detail
  | Ok commands ->
    let transcript = Buffer.create 256 in
    (match
       Sec_4_4.run ~emit:(Buffer.add_string transcript) (Sec_4_4.new_session ()) commands
     with
     | Ok () -> value (Buffer.contents transcript)
     | Error e -> runtime_error (Buffer.contents transcript) (Eval_error.to_string e))
;;

let run_machine source_path =
  match Sec_5_2.read_fixture ~filename:source_path (read_file source_path) with
  | Error detail -> rejected detail
  | Ok fixture ->
    let transcript = Buffer.create 256 in
    (match Sec_5_2.run_fixture ~emit:(Buffer.add_string transcript) fixture with
     | Ok () -> value (Buffer.contents transcript)
     | Error e -> runtime_error (Buffer.contents transcript) (Eval_error.to_string e))
;;

let reference_capability case =
  match String.split_on_char '/' case with
  | capability :: _ -> capability
  | [] -> usage_error "--case is empty"
;;

let run_reference ~case source_path =
  let transcript = Buffer.create 256 in
  match
    Driver_reference.run
      ~emit:(Buffer.add_string transcript)
      ~capability:(reference_capability case)
      source_path
  with
  | Ok () -> value (Buffer.contents transcript)
  | Error e -> runtime_error (Buffer.contents transcript) e
;;

let dispatch ~case ~engine source_path =
  match engine with
  | "direct" -> run_core ~eval:Sec_4_1.run source_path
  | "analyzed" -> run_core ~eval:Sec_4_1.run_analyzed source_path
  | "eceval" -> run_core ~eval:Sec_5_4.run source_path
  | "compiled" -> run_core ~eval:Sec_5_5.run source_path
  | "lazy" -> run_lazy source_path
  | "search" -> run_search source_path
  | "query" -> run_query source_path
  | "machine" -> run_machine source_path
  | "reference" -> run_reference ~case source_path
  | _ -> usage_error ("unknown engine " ^ engine)
;;

let main () =
  let case = ref "" in
  let engine = ref "" in
  let source = ref "" in
  let rec parse = function
    | [] -> ()
    | "--case" :: value :: rest ->
      case := value;
      parse rest
    | "--engine" :: value :: rest ->
      engine := value;
      parse rest
    | "--source" :: value :: rest ->
      source := value;
      parse rest
    | "--work" :: _ :: rest -> parse rest
    | flag :: _ -> usage_error ("unknown argument " ^ flag)
  in
  parse (List.tl (Array.to_list Sys.argv));
  if !case = "" then usage_error "--case is required";
  if !engine = "" then usage_error "--engine is required";
  if !source = "" then usage_error "--source is required";
  dispatch ~case:!case ~engine:!engine !source
;;

let () = main ()
