#![allow(dead_code)]

use std::fmt::Debug;

use serde::Serialize;

const TEST_IO_ENV: &str = "HYLE_TEST_IO";

pub fn dump_debug<T>(case: &str, input: &str, output: &T)
where
    T: Debug,
{
    if !enabled() {
        return;
    }

    print_case(
        case,
        &[("input", input), ("output", &format!("{output:#?}"))],
    );
}

pub fn dump_json<T>(case: &str, input: &str, output: &T)
where
    T: Serialize,
{
    if !enabled() {
        return;
    }

    let output = serde_json::to_string_pretty(output).expect("test output should serialize");
    print_case(case, &[("input", input), ("output", &output)]);
}

pub fn dump_sections(case: &str, sections: &[(&str, String)]) {
    if !enabled() {
        return;
    }

    let borrowed = sections
        .iter()
        .map(|(label, value)| (*label, value.as_str()))
        .collect::<Vec<_>>();
    print_case(case, &borrowed);
}

fn enabled() -> bool {
    std::env::var(TEST_IO_ENV)
        .map(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
        .unwrap_or(false)
}

fn print_case(case: &str, sections: &[(&str, &str)]) {
    eprintln!("\n=== {case} ===");
    for (label, value) in sections {
        eprintln!("--- {label} ---");
        eprintln!("{value}");
    }
}
