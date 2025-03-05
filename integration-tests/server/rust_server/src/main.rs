#![allow(dead_code)]

mod types;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::types::*;
    use chrono::TimeZone;
    use pretty_assertions::assert_eq;

    fn assert_response<T>(data: T, expected: &str)
    where
        T: serde::Serialize,
    {
        let json = serde_json::to_string(&Response::success(data)).unwrap();
        assert_eq!(json, expected);
    }

    fn uuid_from_str(s: &str) -> uuid::Uuid {
        uuid::Uuid::parse_str(s).unwrap()
    }

    #[test]
    fn ints() {
        // input
        let input_json = "42";
        let input: ints_test::Input = serde_json::from_str(input_json).unwrap();
        assert_eq!(input, 42);

        // output
        let output: ints_test::Output = 42;
        assert_response(output, r#"{"type":"success","value":42}"#);
    }

    #[test]
    fn strings() {
        // input
        let input_json = r#""George Washington""#;
        let input: strings_test::Input = serde_json::from_str(input_json).unwrap();
        assert_eq!(input, "George Washington");

        // output
        let output: strings_test::Output = "George Washington".to_string();
        assert_response(output, r#"{"type":"success","value":"George Washington"}"#);
    }

    #[test]
    fn bools() {
        // input
        let input_json = "false";
        let input: bools_test::Input = serde_json::from_str(input_json).unwrap();
        assert_eq!(input, false);

        // output
        let output: bools_test::Output = true;
        assert_response(output, r#"{"type":"success","value":true}"#);
    }

    #[test]
    fn floats() {
        // input
        let input_json = "42.3";
        let input: floats_test::Input = serde_json::from_str(input_json).unwrap();
        assert_eq!(input, 42.3);

        // output
        let output: floats_test::Output = 42.3;
        assert_response(output, r#"{"type":"success","value":42.3}"#);
    }

    #[test]
    fn uuids() {
        // input
        let input_json = r#""777c02a1-6555-4d08-94b5-827819b3f72c""#;
        let input: uuids_test::Input = serde_json::from_str(input_json).unwrap();
        assert_eq!(input, uuid_from_str("777c02a1-6555-4d08-94b5-827819b3f72c"));

        // output
        let output: uuids_test::Output = uuid_from_str("777c02a1-6555-4d08-94b5-827819b3f72c");
        assert_response(
            output,
            r#"{"type":"success","value":"777c02a1-6555-4d08-94b5-827819b3f72c"}"#,
        );
    }

    #[test]
    fn dates() {
        // input
        let input_json = r#""2014-07-08T09:10:11Z""#;
        let input: dates_test::Input = serde_json::from_str(input_json).unwrap();
        assert_eq!(
            input,
            chrono::Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap()
        );

        // output
        let output: dates_test::Output =
            chrono::Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap();
        assert_response(
            output,
            r#"{"type":"success","value":"2014-07-08T09:10:11Z"}"#,
        );
    }

    #[test]
    fn arrays() {
        // input
        let input_json = "[1,2,3]";
        let input: arrays_test::Input = serde_json::from_str(input_json).unwrap();
        assert_eq!(input, vec![1, 2, 3]);

        // output
        let output: arrays_test::Output = vec![
            uuid_from_str("65f4dfe8-ac5f-40bc-a64e-495727602302"),
            uuid_from_str("7364dd48-f837-4504-8706-98e42d2c0887"),
            uuid_from_str("f1b3b3b4-3b3b-4b3b-b3b3-b3b3b3b3b3b3"),
        ];
        assert_response(
            output,
            r#"{"type":"success","value":["65f4dfe8-ac5f-40bc-a64e-495727602302","7364dd48-f837-4504-8706-98e42d2c0887","f1b3b3b4-3b3b-4b3b-b3b3-b3b3b3b3b3b3"]}"#,
        );
    }

    #[test]
    fn optionals() {
        // input
        let input_json = "null";
        let input: optionals_test::Input = serde_json::from_str(input_json).unwrap();
        assert_eq!(input, None);

        let input_json = "42";
        let input: optionals_test::Input = serde_json::from_str(input_json).unwrap();
        assert_eq!(input, Some(42));

        // output
        let output: optionals_test::Output =
            Some(chrono::Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap());
        assert_response(
            output,
            r#"{"type":"success","value":"2014-07-08T09:10:11Z"}"#,
        );

        let output: optionals_test::Output = None;
        assert_response(output, r#"{"type":"success","value":null}"#);
    }

    #[test]
    fn compound_arrays_and_optionals() {
        // input
        let input_json = "[[1,2,3],null,[4,5,6]]";
        let input: compound_arrays_and_optionals_test::Input =
            serde_json::from_str(input_json).unwrap();
        assert_eq!(input, vec![Some(vec![1, 2, 3]), None, Some(vec![4, 5, 6]),]);

        let input_json = "[null,null,null]";
        let input: compound_arrays_and_optionals_test::Input =
            serde_json::from_str(input_json).unwrap();
        assert_eq!(input, vec![None, None, None]);

        // output
        let output: compound_arrays_and_optionals_test::Output = Some(vec![
            uuid_from_str("65f4dfe8-ac5f-40bc-a64e-495727602302"),
            uuid_from_str("7364dd48-f837-4504-8706-98e42d2c0887"),
            uuid_from_str("f1b3b3b4-3b3b-4b3b-b3b3-b3b3b3b3b3b3"),
        ]);
        assert_response(
            output,
            r#"{"type":"success","value":["65f4dfe8-ac5f-40bc-a64e-495727602302","7364dd48-f837-4504-8706-98e42d2c0887","f1b3b3b4-3b3b-4b3b-b3b3-b3b3b3b3b3b3"]}"#,
        );

        let output: compound_arrays_and_optionals_test::Output = None;
        assert_response(output, r#"{"type":"success","value":null}"#);
    }

    #[test]
    fn basic_structs() {
        // input
        let input_json = r#"{"id":"6bc9b6b8-5075-4296-af84-534e6fb5916d"}"#;
        let input: basic_structs_test::Input = serde_json::from_str(input_json).unwrap();
        assert_eq!(
            input,
            basic_structs_test::Input {
                id: uuid_from_str("6bc9b6b8-5075-4296-af84-534e6fb5916d")
            }
        );

        // output
        let output: basic_structs_test::Output = basic_structs_test::Output {
            name: "George Washington".to_string(),
            age: 42,
            birthday: chrono::Utc.with_ymd_and_hms(1732, 2, 22, 0, 0, 0).unwrap(),
            has_beard: true,
        };
        assert_response(
            output,
            r#"{"type":"success","value":{"name":"George Washington","age":42,"birthday":"1732-02-22T00:00:00Z","hasBeard":true}}"#,
        );
    }

    #[test]
    fn structs_with_compound_types() {
        // input
        let input_json = r#"{"foo":"lorem ipsum"}"#;
        let input: structs_with_compound_types_test::Input =
            serde_json::from_str(input_json).unwrap();
        assert_eq!(
            input,
            structs_with_compound_types_test::Input {
                foo: Some("lorem ipsum".to_string())
            }
        );

        let input_json = r#"{"foo":null}"#;
        let input: structs_with_compound_types_test::Input =
            serde_json::from_str(input_json).unwrap();
        assert_eq!(input, structs_with_compound_types_test::Input { foo: None });

        // output
        let output: structs_with_compound_types_test::Output =
            structs_with_compound_types_test::Output {
                bar: vec![Some(42), None, Some(1337)],
            };
        assert_response(
            output,
            r#"{"type":"success","value":{"bar":[42,null,1337]}}"#,
        );

        let output: structs_with_compound_types_test::Output =
            structs_with_compound_types_test::Output { bar: vec![] };
        assert_response(output, r#"{"type":"success","value":{"bar":[]}}"#);
    }

    #[test]
    fn nested_structs() {
        // no input

        // output
        let output: nested_structs_test::Output = nested_structs_test::Output {
            name: "George Washington".to_string(),
            birthday: chrono::Utc.with_ymd_and_hms(1732, 2, 22, 0, 0, 0).unwrap(),
            vehicle: nested_structs_test::Vehicle {
                color: "red".to_string(),
                year: 1776,
            },
        };
        assert_response(
            output,
            r#"{"type":"success","value":{"name":"George Washington","birthday":"1732-02-22T00:00:00Z","vehicle":{"color":"red","year":1776}}}"#,
        );
    }

    #[test]
    fn basic_enums() {
        // input
        let input_json = r#"{"type":"happy"}"#;
        let input: basic_enums_test::Input = serde_json::from_str(input_json).unwrap();
        assert_eq!(input, basic_enums_test::Input::Happy);

        let input_json = r#"{"type":"sad"}"#;
        let input: basic_enums_test::Input = serde_json::from_str(input_json).unwrap();
        assert_eq!(input, basic_enums_test::Input::Sad);

        // output
        let output: basic_enums_test::Output = basic_enums_test::Output::Foo;
        assert_response(output, r#"{"type":"success","value":{"type":"foo"}}"#);

        let output: basic_enums_test::Output = basic_enums_test::Output::Bar;
        assert_response(output, r#"{"type":"success","value":{"type":"bar"}}"#);

        let output: basic_enums_test::Output = basic_enums_test::Output::Baz;
        assert_response(output, r#"{"type":"success","value":{"type":"baz"}}"#);
    }

    #[test]
    fn enums_with_associated_values() {
        // no input

        // output
        let output: enums_with_associated_values_test::Output =
            enums_with_associated_values_test::Output::FirstOption(12);
        assert_response(
            output,
            r#"{"type":"success","value":{"type":"firstOption","value":12}}"#,
        );

        let output: enums_with_associated_values_test::Output =
            enums_with_associated_values_test::Output::SecondOption(vec![
                "foo".to_string(),
                "bar".to_string(),
            ]);
        assert_response(
            output,
            r#"{"type":"success","value":{"type":"secondOption","value":["foo","bar"]}}"#,
        );
    }

    #[test]
    fn composing_structs_and_enums() {
        // no input

        // output
        let output: composing_structs_and_enums_test::Output =
            composing_structs_and_enums_test::Person {
                name: "George Washington".to_string(),
                birthday: chrono::Utc.with_ymd_and_hms(1732, 2, 22, 0, 0, 0).unwrap(),
                id: uuid_from_str("6bc9b6b8-5075-4296-af84-534e6fb5916d"),
                car: composing_structs_and_enums_test::Car {
                    color: "red".to_string(),
                    gear: composing_structs_and_enums_test::Gear::Drive(
                        composing_structs_and_enums_test::Speed::Fifth,
                    ),
                },
            };
        assert_response(
            output,
            r#"{"type":"success","value":{"name":"George Washington","birthday":"1732-02-22T00:00:00Z","id":"6bc9b6b8-5075-4296-af84-534e6fb5916d","car":{"color":"red","gear":{"type":"drive","value":{"type":"fifth"}}}}}"#,
        );
    }

    #[test]
    fn array_of_structs_with_date() {
        // no input

        // output
        let output: array_of_structs_with_date_test::Output = vec![
            array_of_structs_with_date_test::Song {
                title: "Song 1".to_string(),
                composed: chrono::Utc.with_ymd_and_hms(1904, 3, 5, 0, 0, 0).unwrap(),
            },
            array_of_structs_with_date_test::Song {
                title: "Song 2".to_string(),
                composed: chrono::Utc.with_ymd_and_hms(1904, 3, 6, 0, 0, 0).unwrap(),
            },
        ];
        assert_response(
            output,
            r#"{"type":"success","value":[{"title":"Song 1","composed":"1904-03-05T00:00:00Z"},{"title":"Song 2","composed":"1904-03-06T00:00:00Z"}]}"#,
        );
    }
}
