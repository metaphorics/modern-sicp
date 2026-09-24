// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.74.

mod ex_2_74 {
    use std::rc::Rc;

    use ch02::sec_2_4::{contents, tag_list_key, type_tag};
    use sicp_runtime::{Handler, Key, OpTable, SchemeError, Value, cons_cell};

    /// Builds a north-division file entry: `(name . record)`, where a
    /// record is an alist keyed by field-name symbols.
    fn north_entry(name: &str, fields: Vec<(&str, Value)>) -> Value {
        let record = Value::list(
            fields
                .into_iter()
                .map(|(key, value)| Value::Pair(cons_cell(Value::sym(key), value)))
                .collect(),
        );
        Value::Pair(cons_cell(Value::sym(name), record))
    }

    /// Builds a south-division record: the flat property list
    /// `(name n field1 v1 field2 v2 ...)`, a genuinely different schema
    /// from the north division's alist.
    fn south_record(name: &str, fields: Vec<(&str, Value)>) -> Value {
        let mut items = vec![Value::sym("name"), Value::sym(name)];
        for (key, value) in fields {
            items.push(Value::sym(key));
            items.push(value);
        }
        Value::list(items)
    }

    fn assoc(key: &str, alist: &Value) -> Option<Value> {
        let mut cursor = alist.clone();
        while let Value::Pair(cell) = cursor {
            let entry = cell.car.borrow().clone();
            if let Value::Pair(field) = &entry {
                let field_key = field.car.borrow().clone();
                if field_key == Value::sym(key) {
                    return Some(field.cdr.borrow().clone());
                }
            }
            cursor = cell.cdr.borrow().clone();
        }
        None
    }

    fn property(key: &str, plist: &Value) -> Option<Value> {
        let mut cursor = plist.clone();
        loop {
            let Value::Pair(cell) = cursor else {
                return None;
            };
            let field_key = cell.car.borrow().clone();
            let rest = cell.cdr.borrow().clone();
            let Value::Pair(rest_cell) = rest else {
                return None;
            };
            let field_value = rest_cell.car.borrow().clone();
            if field_key == Value::sym(key) {
                return Some(field_value);
            }
            cursor = rest_cell.cdr.borrow().clone();
        }
    }

    /// North's `get-record`: scans the `(name . record)` alist.
    fn north_get_record() -> Handler {
        Rc::new(|args| {
            let [name, records] = args else {
                return Err(SchemeError::WrongArity {
                    procedure: "get-record".into(),
                    expected: "2".into(),
                    got: args.len(),
                });
            };
            let mut cursor = records.clone();
            while let Value::Pair(cell) = cursor {
                let entry = cell.car.borrow().clone();
                let rest = cell.cdr.borrow().clone();
                if let Value::Pair(pair) = &entry
                    && *pair.car.borrow() == *name
                {
                    return Ok(Value::tagged("north-division", pair.cdr.borrow().clone()));
                }
                cursor = rest;
            }
            Ok(Value::Bool(false))
        })
    }

    /// North's `get-salary`: an alist lookup on the tagged record's
    /// contents.
    fn north_get_salary() -> Handler {
        Rc::new(|args| {
            let [record] = args else {
                return Err(SchemeError::WrongArity {
                    expected: 1,
                    got: args.len(),
                });
            };
            assoc("salary", record)
                .ok_or_else(|| SchemeError::TypeMismatch("no salary field".into()))
        })
    }

    /// South's `get-record`: scans a flat list of property-list records
    /// for the one whose `name` property matches.
    fn south_get_record() -> Handler {
        Rc::new(|args| {
            let [name, records] = args else {
                return Err(SchemeError::WrongArity {
                    procedure: "get-record".into(),
                    expected: "2".into(),
                    got: args.len(),
                });
            };
            let mut cursor = records.clone();
            while let Value::Pair(cell) = cursor {
                let entry = cell.car.borrow().clone();
                let rest = cell.cdr.borrow().clone();
                if property("name", &entry).as_ref() == Some(name) {
                    return Ok(Value::tagged("south-division", entry));
                }
                cursor = rest;
            }
            Ok(Value::Bool(false))
        })
    }

    /// South's `get-salary`: a property-list lookup.
    fn south_get_salary() -> Handler {
        Rc::new(|args| {
            let [record] = args else {
                return Err(SchemeError::WrongArity {
                    expected: 1,
                    got: args.len(),
                });
            };
            property("salary", record)
                .ok_or_else(|| SchemeError::TypeMismatch("no salary field".into()))
        })
    }

    /// Part (a)/(b): installs north's package.
    fn install_north_division(table: &OpTable) {
        let tag = tag_list_key(&["north-division"]);
        table.put(Key::sym("get-record"), tag.clone(), north_get_record());
        table.put(Key::sym("get-salary"), tag, north_get_salary());
    }

    /// Installs south's package, structured differently from north's.
    fn install_south_division(table: &OpTable) {
        let tag = tag_list_key(&["south-division"]);
        table.put(Key::sym("get-record"), tag.clone(), south_get_record());
        table.put(Key::sym("get-salary"), tag, south_get_salary());
    }

    /// Part (d): a third division, acquired later. Headquarters' `get_record`,
    /// `get_salary`, and `find_employee_record` do not change; only this
    /// install is new.
    fn install_acquired_division(table: &OpTable) {
        let tag = tag_list_key(&["acquired-division"]);
        table.put(Key::sym("get-record"), tag.clone(), north_get_record());
        table.put(Key::sym("get-salary"), tag, north_get_salary());
    }

    fn make_north_file() -> Value {
        Value::tagged(
            "north-division",
            Value::list(vec![
                north_entry(
                    "Bitdiddle",
                    vec![
                        ("address", Value::string("Boston")),
                        ("salary", Value::int(60000)),
                    ],
                ),
                north_entry(
                    "Cratchet",
                    vec![
                        ("address", Value::string("Watertown")),
                        ("salary", Value::int(50000)),
                    ],
                ),
            ]),
        )
    }

    fn make_south_file() -> Value {
        Value::tagged(
            "south-division",
            Value::list(vec![
                south_record(
                    "Hacker",
                    vec![
                        ("address", Value::string("Cambridge")),
                        ("salary", Value::int(75000)),
                    ],
                ),
                south_record(
                    "Fect",
                    vec![
                        ("address", Value::string("Slumerville")),
                        ("salary", Value::int(65000)),
                    ],
                ),
            ]),
        )
    }

    fn make_acquired_file() -> Value {
        Value::tagged(
            "acquired-division",
            Value::list(vec![north_entry(
                "Reasoner",
                vec![
                    ("address", Value::string("Slumerville")),
                    ("salary", Value::int(80000)),
                ],
            )]),
        )
    }

    /// Part (a): headquarters' `get-record`, applicable to any division's
    /// file because the file itself carries the type tag that selects the
    /// division's handler. Answers `false` — the book's own miss value —
    /// when the file has no record for `name`.
    fn get_record(table: &OpTable, name: &str, file: &Value) -> Result<Value, SchemeError> {
        let tag = type_tag(file)?;
        let Some(handler) = table.get(&Key::sym("get-record"), &tag_list_key(&[tag.as_ref()]))
        else {
            return Err(SchemeError::UserRaised {
                message: "No method for these types: GET-RECORD".into(),
                irritants: vec![Value::Sym(tag)],
            });
        };
        handler(&[Value::sym(name), contents(file)?])
    }

    /// Part (b): headquarters' `get-salary`, dispatching on the tag the
    /// record itself carries — the record's structure is the division's
    /// business, not headquarters'.
    fn get_salary(table: &OpTable, record: &Value) -> Result<Value, SchemeError> {
        let tag = type_tag(record)?;
        let Some(handler) = table.get(&Key::sym("get-salary"), &tag_list_key(&[tag.as_ref()]))
        else {
            return Err(SchemeError::UserRaised {
                message: "No method for these types: GET-SALARY".into(),
                irritants: vec![Value::Sym(tag)],
            });
        };
        handler(&[contents(record)?])
    }

    /// Part (c): searches every division's file for `name`'s record.
    fn find_employee_record(
        table: &OpTable,
        name: &str,
        files: &[Value],
    ) -> Result<Option<Value>, SchemeError> {
        for file in files {
            let record = get_record(table, name, file)?;
            if record != Value::Bool(false) {
                return Ok(Some(record));
            }
        }
        Ok(None)
    }

    fn as_salary(v: &Value) -> Result<i128, SchemeError> {
        match v {
            Value::Int(n) => Ok(*n),
            other => Err(SchemeError::TypeMismatch(format!("not a salary: {other}"))),
        }
    }

    /// Exercise 2.74: `get-record`, `get-salary`, and `find-employee-record`
    /// over two differently structured division files, plus part (d): a
    /// third division installed with no change to headquarters' code.
    /// Returns the salaries found for Bitdiddle in the north file, for
    /// Hacker in the south file, and for Hacker across both files.
    pub fn ex_2_74() -> Result<(i128, i128, i128), SchemeError> {
        let table = OpTable::new();
        install_north_division(&table);
        install_south_division(&table);
        install_acquired_division(&table);

        let north = make_north_file();
        let south = make_south_file();
        let acquired = make_acquired_file();

        let bitdiddle_record = get_record(&table, "Bitdiddle", &north)?;
        let bitdiddle_salary = as_salary(&get_salary(&table, &bitdiddle_record)?)?;

        let hacker_record = get_record(&table, "Hacker", &south)?;
        let hacker_salary = as_salary(&get_salary(&table, &hacker_record)?)?;

        let found = find_employee_record(&table, "Hacker", &[north, south, acquired])?
            .ok_or_else(|| SchemeError::TypeMismatch("Hacker is in the south file".into()))?;
        let found_salary = as_salary(&get_salary(&table, &found)?)?;

        Ok((bitdiddle_salary, hacker_salary, found_salary))
    }

    #[cfg(test)]
    mod tests {
        use super::{
            as_salary, find_employee_record, get_record, get_salary, install_acquired_division,
            install_north_division, install_south_division, make_acquired_file, make_north_file,
            make_south_file,
        };
        use sicp_runtime::{OpTable, Value};

        #[test]
        fn ex_2_74() {
            assert_eq!(super::ex_2_74(), Ok((60000, 75000, 75000)));
        }

        #[test]
        fn missing_employee_answers_false_not_an_error() {
            let table = OpTable::new();
            install_north_division(&table);
            let north = make_north_file();
            let record = get_record(&table, "Nobody", &north).expect("north handler installed");
            assert_eq!(record, Value::Bool(false));
        }

        #[test]
        fn part_d_a_third_division_needs_only_an_install() {
            let table = OpTable::new();
            install_north_division(&table);
            install_south_division(&table);
            install_acquired_division(&table);
            let files = [make_north_file(), make_south_file(), make_acquired_file()];
            let record = find_employee_record(&table, "Reasoner", &files)
                .expect("lookup")
                .expect("Reasoner is in the acquired file");
            let salary = as_salary(&get_salary(&table, &record).expect("acquired handler"))
                .expect("salary is an int");
            assert_eq!(salary, 80000);
        }

        #[test]
        fn find_employee_record_returns_none_for_an_unknown_name() {
            let table = OpTable::new();
            install_north_division(&table);
            install_south_division(&table);
            let files = [make_north_file(), make_south_file()];
            assert_eq!(
                find_employee_record(&table, "Nobody", &files).expect("lookup"),
                None
            );
        }
    }
}
