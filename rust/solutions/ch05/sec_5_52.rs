// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.52: the compiler's C backend.
//!
//! The instruction sequences the 5.5 compiler emits are translated one
//! statement at a time into the body of a C `main`: registers are C
//! globals, labels are C labels (dashes become underscores), `continue`
//! holds a label address (the GNU computed-goto extension the system
//! compiler accepts), and every machine operation is one C function
//! mirroring the compiled operations table. Constants ride in globals
//! the initializer builds. Each top-level form of the adapted
//! metacircular source is compiled in turn and its value printed, so
//! the C program replays the book's session. The build uses the system
//! C compiler.

use ch05::sec_5_2::Fault;
use ch05::sec_5_5::{Linkage, METACIRCULAR, compile, default_config, new_state, top_cenv};
use sicp_runtime::{Value, read_program};

mod ex_5_52 {
    //! Exercise 5.52: compile the adapted metacircular source to C,
    //! build it, and run the object session through the C-compiled
    //! interpreter.

    use super::*;

    const RUNTIME_C: &str = include_str!("metacircular_backend_5_52.c");

    /// A C identifier for a controller label: dashes become underscores.
    fn c_label(name: &str) -> String {
        name.chars()
            .map(|c| if c == '-' { '_' } else { c })
            .collect()
    }

    /// A C function name for a machine operation.
    fn c_op(name: &str) -> String {
        let mut out = String::from("op_");
        for c in name.chars() {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => out.push(c),
                '?' => out.push_str("_p"),
                '!' => out.push_str("_x"),
                '*' => out.push_str("_star"),
                '+' => out.push_str("_plus"),
                '=' => out.push_str("_eq"),
                '<' => out.push_str("_lt"),
                '>' => out.push_str("_gt"),
                '/' => out.push_str("_slash"),
                _ => out.push('_'),
            }
        }
        out
    }

    /// Splits the inside of one parenthesized controller line into its
    /// top-level items, keeping nested structure and strings intact.
    fn split_top(line: &str) -> Result<Vec<String>, Fault> {
        let fail = || Fault::Parse(format!("the C backend rejects the line: {line}"));
        let inner = line
            .strip_prefix('(')
            .and_then(|rest| rest.strip_suffix(')'));
        let Some(inner) = inner else {
            return Err(fail());
        };
        let mut items = Vec::new();
        let bytes = inner.as_bytes();
        let mut depth = 0usize;
        let mut in_string = false;
        let mut escaped = false;
        let mut start = 0usize;
        let mut i = 0usize;
        while i <= bytes.len() {
            let end = i == bytes.len();
            let c = if end { b' ' } else { bytes[i] };
            if in_string {
                if escaped {
                    escaped = false;
                } else if c == b'\\' {
                    escaped = true;
                } else if c == b'"' {
                    in_string = false;
                }
            } else if c == b'"' {
                in_string = true;
            } else if c == b'(' {
                depth += 1;
            } else if c == b')' {
                depth = depth.checked_sub(1).ok_or_else(fail)?;
            }
            let boundary = end || (!in_string && depth == 0 && c.is_ascii_whitespace());
            if boundary {
                if start < i {
                    items.push(inner[start..i].to_owned());
                }
                start = i + 1;
            }
            i += 1;
        }
        if in_string || depth != 0 {
            return Err(fail());
        }
        Ok(items)
    }

    /// The operand of an `(op ...)` or assignment: a register, a
    /// constant slot, or a label address.
    fn operand_c(
        item: &str,
        consts: &mut Vec<String>,
        labels: &std::collections::HashSet<String>,
    ) -> Result<String, Fault> {
        let fail = || Fault::Parse(format!("the C backend rejects the operand: {item}"));
        if let Some(reg) = item.strip_prefix("(reg ") {
            let reg = reg.strip_suffix(')').ok_or_else(fail)?;
            return Ok(format!("R_{reg}"));
        }
        if let Some(label) = item.strip_prefix("(label ") {
            let label = label.strip_suffix(')').ok_or_else(fail)?;
            return Ok(format!("&&{}", c_label(label)));
        }
        if let Some(payload) = item.strip_prefix("(const ") {
            let payload = payload.strip_suffix(')').ok_or_else(fail)?;
            if labels.contains(payload) {
                return Ok(format!("&&{}", c_label(payload)));
            }
            if let Some(index) = consts.iter().position(|known| known == payload) {
                return Ok(format!("K{index}"));
            }
            consts.push(payload.to_owned());
            return Ok(format!("K{}", consts.len() - 1));
        }
        Err(fail())
    }

    /// One controller statement as C lines.
    #[expect(
        clippy::too_many_lines,
        reason = "Keeping this one-controller-instruction dispatch together mirrors the source machine."
    )]
    fn statement_c(
        line: &str,
        consts: &mut Vec<String>,
        labels: &std::collections::HashSet<String>,
        entry_open: &mut bool,
    ) -> Result<Vec<String>, Fault> {
        let fail = || Fault::Parse(format!("the C backend rejects the line: {line}"));
        if !line.starts_with('(') {
            return Ok(vec![format!("{}:;", c_label(line))]);
        }
        let items = split_top(line)?;
        let head = items.first().cloned().unwrap_or_default();
        match head.as_str() {
            "save" => {
                let reg = items.get(1).ok_or_else(fail)?;
                if reg == "continue" {
                    Ok(vec!["push_addr(R_continue);".to_owned()])
                } else {
                    Ok(vec![format!("push_val(R_{reg});")])
                }
            }
            "restore" => {
                let reg = items.get(1).ok_or_else(fail)?;
                if reg == "continue" {
                    Ok(vec!["R_continue = pop_addr();".to_owned()])
                } else {
                    Ok(vec![format!("R_{reg} = pop_val();")])
                }
            }
            "goto" => {
                let target = items.get(1).ok_or_else(fail)?;
                if let Some(label) = target
                    .strip_prefix("(label ")
                    .and_then(|rest| rest.strip_suffix(')'))
                {
                    Ok(vec![format!("goto {};", c_label(label))])
                } else if target == "(reg continue)" {
                    Ok(vec!["goto *R_continue;".to_owned()])
                } else if target == "(reg val)" && *entry_open {
                    Ok(vec!["goto *entry_addr;".to_owned()])
                } else {
                    Err(fail())
                }
            }
            "branch" => {
                let target = items.get(1).ok_or_else(fail)?;
                let Some(label) = target
                    .strip_prefix("(label ")
                    .and_then(|rest| rest.strip_suffix(')'))
                else {
                    return Err(fail());
                };
                Ok(vec![format!("if (flag) goto {};", c_label(label))])
            }
            "test" => {
                let op_item = items.get(1).ok_or_else(fail)?;
                let (name, args) = op_call(op_item, &items[2..], consts, labels)?;
                Ok(vec![format!("flag = is_true({name}({args}));")])
            }
            "perform" => {
                let op_item = items.get(1).ok_or_else(fail)?;
                let (name, args) = op_call(op_item, &items[2..], consts, labels)?;
                Ok(vec![format!("(void){name}({args});")])
            }
            "assign" => {
                let target = items.get(1).ok_or_else(fail)?;
                let source = items.get(2).ok_or_else(fail)?;
                if let Some(reg) = source
                    .strip_prefix("(reg ")
                    .and_then(|rest| rest.strip_suffix(')'))
                {
                    return Ok(vec![format!("R_{target} = R_{reg};")]);
                }
                if let Some(label) = source
                    .strip_prefix("(label ")
                    .and_then(|rest| rest.strip_suffix(')'))
                {
                    if target != "continue" {
                        return Err(fail());
                    }
                    return Ok(vec![format!("R_continue = &&{};", c_label(label))]);
                }
                if source.starts_with("(const ") {
                    let rendered = operand_c(source, consts, labels)?;
                    return Ok(vec![format!("R_{target} = {rendered};")]);
                }
                if source.starts_with("(op ") {
                    let name = source
                        .strip_prefix("(op ")
                        .and_then(|rest| rest.strip_suffix(')'))
                        .ok_or_else(fail)?;
                    if name == "make-compiled-procedure" {
                        let entry_item = items.get(3).ok_or_else(fail)?;
                        let env_item = items.get(4).ok_or_else(fail)?;
                        let entry = operand_c(entry_item, consts, labels)?;
                        let env = operand_c(env_item, consts, labels)?;
                        return Ok(vec![format!("R_{target} = mk_compiled({entry}, {env});")]);
                    }
                    if name == "compiled-procedure-entry" {
                        let proc_item = items.get(3).ok_or_else(fail)?;
                        let proc = operand_c(proc_item, consts, labels)?;
                        *entry_open = true;
                        return Ok(vec![format!("entry_addr = compiled_entry({proc});")]);
                    }
                    let mut arg_items = Vec::new();
                    for item in &items[3..] {
                        arg_items.push(operand_c(item, consts, labels)?);
                    }
                    return Ok(vec![format!(
                        "R_{target} = {}({});",
                        c_op(name),
                        arg_items.join(", ")
                    )]);
                }
                Err(fail())
            }
            _ => Err(fail()),
        }
    }

    /// An `(op name)` head plus its operand items as a C call.
    fn op_call(
        op_item: &str,
        rest: &[String],
        consts: &mut Vec<String>,
        labels: &std::collections::HashSet<String>,
    ) -> Result<(String, String), Fault> {
        let fail = || Fault::Parse(format!("the C backend rejects the call: {op_item}"));
        let name = op_item
            .strip_prefix("(op ")
            .and_then(|rest| rest.strip_suffix(')'))
            .ok_or_else(fail)?;
        if name == "make-compiled-procedure" || name == "compiled-procedure-entry" {
            return Err(fail());
        }
        let mut args = Vec::new();
        for item in rest {
            args.push(operand_c(item, consts, labels)?);
        }
        Ok((c_op(name), args.join(", ")))
    }

    /// A C expression building one compile-time constant value.
    fn const_c(value: &Value) -> Result<String, Fault> {
        let fail = || Fault::Parse(format!("the C backend rejects the constant: {value:?}"));
        match value {
            Value::Int(n) => Ok(format!("mk_int({n}LL)")),
            Value::Bool(true) => Ok("V_TRUE".to_owned()),
            Value::Bool(false) => Ok("V_FALSE".to_owned()),
            Value::Sym(name) => Ok(format!("mk_sym({})", c_string(name))),
            Value::Str(text) => Ok(format!("mk_str({})", c_string(text))),
            Value::Nil => Ok("V_NIL".to_owned()),
            Value::Pair(cell) => {
                let car = const_c(&cell.car.borrow())?;
                let cdr = const_c(&cell.cdr.borrow())?;
                Ok(format!("cons({car}, {cdr})"))
            }
            _ => Err(fail()),
        }
    }

    /// A C string literal for `text`, escaping quotes and backslashes.
    fn c_string(text: &str) -> String {
        let mut out = String::from("\"");
        for c in text.chars() {
            match c {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                _ => out.push(c),
            }
        }
        out.push('"');
        out
    }

    /// The value a `(const ...)` spelling names: the 5.2 machine's
    /// `const_value` unquotes one `quote` level and recurses into
    /// lists, so the backend does the same after parsing.
    fn unquote_const(value: &Value) -> Result<Value, Fault> {
        let fail = || Fault::Parse(format!("the C backend rejects the constant: {value:?}"));
        if let Value::Pair(_) = value {
            let items = value.list_items().map_err(|_| fail())?;
            if matches!(items.first(), Some(Value::Sym(tag)) if tag.as_ref() == "quote") {
                return items.get(1).cloned().ok_or_else(fail);
            }
            let values = items
                .iter()
                .map(unquote_const)
                .collect::<Result<Vec<_>, _>>()?;
            return Ok(Value::list(values));
        }
        Ok(value.clone())
    }

    /// The whole C file: the runtime, the constant initializer, and the
    /// compiled forms with a print between neighbors.
    fn compile_to_c(source: &str) -> Result<String, Fault> {
        let forms = read_program(source).map_err(|error| Fault::Parse(error.to_string()))?;
        let cfg = default_config();
        let state = new_state();
        let cenv = top_cenv();
        let mut consts: Vec<String> = Vec::new();
        let mut bodies: Vec<Vec<String>> = Vec::new();
        for form in &forms {
            let seq = compile(&cfg, &state, &cenv, form, "val", &Linkage::Next)?;
            bodies.push(seq.stmts);
        }
        let labels: std::collections::HashSet<String> = bodies
            .iter()
            .flatten()
            .filter(|line| !line.starts_with('('))
            .cloned()
            .collect();
        let mut program = Vec::new();
        for (index, stmts) in bodies.iter().enumerate() {
            let mut entry_open = false;
            program.push(format!("prog_entry_{index}:;"));
            for line in stmts {
                program.extend(statement_c(line, &mut consts, &labels, &mut entry_open)?);
            }
            program.push("user_print(R_val); emit_newline();".to_owned());
        }
        let mut initializers = Vec::new();
        for (index, spelling) in consts.iter().enumerate() {
            let forms = read_program(spelling).map_err(|error| Fault::Parse(error.to_string()))?;
            let value = forms
                .first()
                .ok_or_else(|| Fault::Parse(format!("the C backend rejects: {spelling}")))?;
            initializers.push(format!(
                "K{} = {};",
                index,
                const_c(&unquote_const(value)?)?
            ));
        }
        let declarations: Vec<String> = (0..consts.len())
            .map(|index| format!("static Val *K{index};"))
            .collect();
        Ok(format!(
            "{RUNTIME_C}\n{}\nstatic void init_constants(void) {{\n{}\n}}\nint main(void) {{\ninit_symbols();\ninit_global();\ninit_constants();\n{}\nreturn 0;\n}}\n",
            declarations.join("\n"),
            initializers.join("\n"),
            program.join("\n"),
        ))
    }

    /// Writes the C file, builds it with the system compiler, runs it,
    /// and answers its combined output.
    fn build_and_run(c_source: &str) -> Result<String, Fault> {
        let dir = std::env::temp_dir().join(format!("sicp_rust_5_52_{}", std::process::id()));
        std::fs::create_dir_all(&dir)
            .map_err(|error| Fault::Parse(format!("scratch dir: {error}")))?;
        let source = dir.join("compiled.c");
        let binary = dir.join("compiled");
        std::fs::write(&source, c_source)
            .map_err(|error| Fault::Parse(format!("write c: {error}")))?;
        let build = std::process::Command::new("cc")
            .arg("-O1")
            .arg("-o")
            .arg(&binary)
            .arg(&source)
            .output()
            .map_err(|error| Fault::Parse(format!("cc spawn: {error}")))?;
        if !build.status.success() {
            let text = String::from_utf8_lossy(&build.stderr).to_string();
            let _ = std::fs::remove_dir_all(&dir);
            return Err(Fault::Parse(format!(
                "the C backend failed to build: {text}"
            )));
        }
        let run = std::process::Command::new(&binary)
            .output()
            .map_err(|error| Fault::Parse(format!("run spawn: {error}")))?;
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&run.stdout),
            String::from_utf8_lossy(&run.stderr)
        );
        let _ = std::fs::remove_dir_all(&dir);
        if !run.status.success() {
            return Err(Fault::Parse(format!("the C program failed: {text}")));
        }
        Ok(text)
    }

    /// Compiles the adapted metacircular source to C, builds it, and
    /// runs the object session: factorial answers 120 and the tick
    /// combination answers its triple.
    pub fn ex_5_52() -> Result<Vec<String>, Fault> {
        let c_source = compile_to_c(METACIRCULAR)?;
        let output = build_and_run(&c_source)?;
        assert!(output.contains("120"), "{output}");
        assert!(output.contains("(tick tick tick)"), "{output}");
        Ok(vec![output])
    }

    #[test]
    fn ex_5_52_check() -> Result<(), Fault> {
        let lines = ex_5_52()?;
        assert!(lines[0].contains("120"));
        assert!(lines[0].contains("(tick tick tick)"));
        Ok(())
    }
}
