#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
#[cfg(test)]
mod outlittests {
    use ifmt::{iformat, ipanic, iwrite, iwriteln};
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const simple_subst: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("outlittests::simple_subst"),
            ignore: false,
            allow_fail: false,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(simple_subst())),
    };
    fn simple_subst() {
        let x = 5.3;
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&x,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1(&["hello this is { a test "], args)
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&x,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1(&["hello this is { a test "], args)
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        let y = "420";
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&y,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe { ::core::fmt::Arguments::new_v1(&[""], args) },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&y,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe { ::core::fmt::Arguments::new_v1(&[""], args) },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&x, &y) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        } {
                            ref args => unsafe { ::core::fmt::Arguments::new_v1(&["", ""], args) },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&x, &y) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        } {
                            ref args => unsafe { ::core::fmt::Arguments::new_v1(&["", ""], args) },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const expr_subst: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("outlittests::expr_subst"),
            ignore: false,
            allow_fail: false,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(expr_subst())),
    };
    fn expr_subst() {
        let x = 5.3;
        let y = 29.0;
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&(x * y), &(x + y)) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1(&["atest ", " "], args)
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&(x * y), &(x + y)) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1(&["atest ", " "], args)
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const spec_subst: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("outlittests::spec_subst"),
            ignore: false,
            allow_fail: false,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(spec_subst())),
    };
    fn spec_subst() {
        let x = 5.3;
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&x,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &["hello this is { a test "],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::core::fmt::rt::v1::Count::Is(5usize),
                                            width: ::core::fmt::rt::v1::Count::Implied,
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&x,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &["hello this is { a test "],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::core::fmt::rt::v1::Count::Is(5usize),
                                            width: ::core::fmt::rt::v1::Count::Implied,
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        let y = "420";
        let z = Some("debugged");
        let k = 420.0;
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&y,) {
                            (arg0,) => {
                                [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)]
                            }
                        } {
                            ref args => unsafe { ::core::fmt::Arguments::new_v1(&[""], args) },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&y,) {
                            (arg0,) => {
                                [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)]
                            }
                        } {
                            ref args => unsafe { ::core::fmt::Arguments::new_v1(&[""], args) },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&z, &z) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                            ],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &["", ""],
                                    args,
                                    &[
                                        ::core::fmt::rt::v1::Argument {
                                            position: 0usize,
                                            format: ::core::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                flags: 0u32,
                                                precision: ::core::fmt::rt::v1::Count::Implied,
                                                width: ::core::fmt::rt::v1::Count::Implied,
                                            },
                                        },
                                        ::core::fmt::rt::v1::Argument {
                                            position: 1usize,
                                            format: ::core::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                flags: 4u32,
                                                precision: ::core::fmt::rt::v1::Count::Implied,
                                                width: ::core::fmt::rt::v1::Count::Implied,
                                            },
                                        },
                                    ],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&z, &z) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                            ],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &["", ""],
                                    args,
                                    &[
                                        ::core::fmt::rt::v1::Argument {
                                            position: 0usize,
                                            format: ::core::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                flags: 0u32,
                                                precision: ::core::fmt::rt::v1::Count::Implied,
                                                width: ::core::fmt::rt::v1::Count::Implied,
                                            },
                                        },
                                        ::core::fmt::rt::v1::Argument {
                                            position: 1usize,
                                            format: ::core::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                flags: 4u32,
                                                precision: ::core::fmt::rt::v1::Count::Implied,
                                                width: ::core::fmt::rt::v1::Count::Implied,
                                            },
                                        },
                                    ],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&((x * -k) as i32), &((x + k) as i32), &(x * k)) {
                            (arg0, arg1, arg2) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Octal::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::UpperHex::fmt),
                                ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Display::fmt),
                            ],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &["atest ", " ", " "],
                                    args,
                                    &[
                                        ::core::fmt::rt::v1::Argument {
                                            position: 0usize,
                                            format: ::core::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                flags: 0u32,
                                                precision: ::core::fmt::rt::v1::Count::Implied,
                                                width: ::core::fmt::rt::v1::Count::Implied,
                                            },
                                        },
                                        ::core::fmt::rt::v1::Argument {
                                            position: 1usize,
                                            format: ::core::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                flags: 0u32,
                                                precision: ::core::fmt::rt::v1::Count::Implied,
                                                width: ::core::fmt::rt::v1::Count::Implied,
                                            },
                                        },
                                        ::core::fmt::rt::v1::Argument {
                                            position: 2usize,
                                            format: ::core::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                flags: 0u32,
                                                precision: ::core::fmt::rt::v1::Count::Is(4usize),
                                                width: ::core::fmt::rt::v1::Count::Implied,
                                            },
                                        },
                                    ],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&((x * -k) as i32), &((x + k) as i32), &(x * k)) {
                            (arg0, arg1, arg2) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Octal::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::UpperHex::fmt),
                                ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Display::fmt),
                            ],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &["atest ", " ", " "],
                                    args,
                                    &[
                                        ::core::fmt::rt::v1::Argument {
                                            position: 0usize,
                                            format: ::core::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                flags: 0u32,
                                                precision: ::core::fmt::rt::v1::Count::Implied,
                                                width: ::core::fmt::rt::v1::Count::Implied,
                                            },
                                        },
                                        ::core::fmt::rt::v1::Argument {
                                            position: 1usize,
                                            format: ::core::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                flags: 0u32,
                                                precision: ::core::fmt::rt::v1::Count::Implied,
                                                width: ::core::fmt::rt::v1::Count::Implied,
                                            },
                                        },
                                        ::core::fmt::rt::v1::Argument {
                                            position: 2usize,
                                            format: ::core::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                flags: 0u32,
                                                precision: ::core::fmt::rt::v1::Count::Is(4usize),
                                                width: ::core::fmt::rt::v1::Count::Implied,
                                            },
                                        },
                                    ],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&420,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 1u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Is(4usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&420,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 1u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Is(4usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&420,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 2u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Is(4usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&420,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 2u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Is(4usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&"BBBBBBBBBBBBBBBBBBBB",) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Right,
                                            flags: 0u32,
                                            precision: ::core::fmt::rt::v1::Count::Is(4usize),
                                            width: ::core::fmt::rt::v1::Count::Is(10usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&("BBBBBBBBBBBBBBBBBBBB"),) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Right,
                                            flags: 0u32,
                                            precision: ::core::fmt::rt::v1::Count::Is(4usize),
                                            width: ::core::fmt::rt::v1::Count::Is(10usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&"B",) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: '0',
                                            align: ::core::fmt::rt::v1::Alignment::Right,
                                            flags: 0u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Is(5usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&("B"),) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: '0',
                                            align: ::core::fmt::rt::v1::Alignment::Right,
                                            flags: 0u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Is(5usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&"B",) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: 'L',
                                            align: ::core::fmt::rt::v1::Alignment::Right,
                                            flags: 0u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Is(5usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&("B"),) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: 'L',
                                            align: ::core::fmt::rt::v1::Alignment::Right,
                                            flags: 0u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Is(5usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&3.1415e9f32,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::core::fmt::rt::v1::Count::Is(3usize),
                                            width: ::core::fmt::rt::v1::Count::Implied,
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&3.1415e9f32,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::core::fmt::rt::v1::Count::Is(3usize),
                                            width: ::core::fmt::rt::v1::Count::Implied,
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&3.1415e9f32,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::core::fmt::rt::v1::Count::Is(3usize),
                                            width: ::core::fmt::rt::v1::Count::Is(11usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&3.1415e9f32,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::core::fmt::rt::v1::Count::Is(3usize),
                                            width: ::core::fmt::rt::v1::Count::Is(11usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&3.1415e9f32,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::core::fmt::rt::v1::Count::Is(3usize),
                                            width: ::core::fmt::rt::v1::Count::Is(11usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&3.1415e9f32,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::core::fmt::rt::v1::Count::Is(3usize),
                                            width: ::core::fmt::rt::v1::Count::Is(11usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&3.1415e9f32,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 1u32,
                                            precision: ::core::fmt::rt::v1::Count::Is(3usize),
                                            width: ::core::fmt::rt::v1::Count::Is(11usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&3.1415e9f32,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 1u32,
                                            precision: ::core::fmt::rt::v1::Count::Is(3usize),
                                            width: ::core::fmt::rt::v1::Count::Is(11usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&3.1415e9f32,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 1u32,
                                            precision: ::core::fmt::rt::v1::Count::Is(3usize),
                                            width: ::core::fmt::rt::v1::Count::Is(11usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&3.1415e9f32,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 1u32,
                                            precision: ::core::fmt::rt::v1::Count::Is(3usize),
                                            width: ::core::fmt::rt::v1::Count::Is(11usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&3.1415e9f32,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::LowerExp::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 1u32,
                                            precision: ::core::fmt::rt::v1::Count::Is(3usize),
                                            width: ::core::fmt::rt::v1::Count::Is(11usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&3.1415e9f32,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::LowerExp::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 1u32,
                                            precision: ::core::fmt::rt::v1::Count::Is(3usize),
                                            width: ::core::fmt::rt::v1::Count::Is(11usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&3.1415e9f32,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::LowerExp::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 1u32,
                                            precision: ::core::fmt::rt::v1::Count::Is(3usize),
                                            width: ::core::fmt::rt::v1::Count::Is(11usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&3.1415e9f32,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::LowerExp::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 1u32,
                                            precision: ::core::fmt::rt::v1::Count::Is(3usize),
                                            width: ::core::fmt::rt::v1::Count::Is(11usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&3.1415e9f32,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::UpperExp::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 1u32,
                                            precision: ::core::fmt::rt::v1::Count::Is(3usize),
                                            width: ::core::fmt::rt::v1::Count::Is(11usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&3.1415e9f32,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::UpperExp::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 1u32,
                                            precision: ::core::fmt::rt::v1::Count::Is(3usize),
                                            width: ::core::fmt::rt::v1::Count::Is(11usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&3.1415e9f32,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::LowerExp::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 1u32,
                                            precision: ::core::fmt::rt::v1::Count::Is(3usize),
                                            width: ::core::fmt::rt::v1::Count::Is(11usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&3.1415e9f32,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::LowerExp::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 1u32,
                                            precision: ::core::fmt::rt::v1::Count::Is(3usize),
                                            width: ::core::fmt::rt::v1::Count::Is(11usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&3.1415e9f32,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::UpperExp::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 1u32,
                                            precision: ::core::fmt::rt::v1::Count::Is(3usize),
                                            width: ::core::fmt::rt::v1::Count::Is(11usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&3.1415e9f32,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::UpperExp::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 1u32,
                                            precision: ::core::fmt::rt::v1::Count::Is(3usize),
                                            width: ::core::fmt::rt::v1::Count::Is(11usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&4.20,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Left,
                                            flags: 0u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Is(6usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&4.20,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Left,
                                            flags: 0u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Is(6usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&4.20,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 8u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Is(6usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&4.20,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 8u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Is(6usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&4.20,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Center,
                                            flags: 8u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Is(6usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&4.20,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Center,
                                            flags: 8u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Is(6usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&4.20,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 9u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Is(6usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&4.20,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 9u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Is(6usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&4.20,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Left,
                                            flags: 9u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Is(6usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&4.20,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Left,
                                            flags: 9u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Is(6usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&420,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::LowerHex::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 12u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Is(6usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&420,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::LowerHex::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 12u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Is(6usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&420,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::LowerHex::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Right,
                                            flags: 12u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Is(6usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&420,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::LowerHex::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Right,
                                            flags: 12u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Is(6usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&420,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::LowerHex::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Center,
                                            flags: 12u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Is(6usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&420,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::LowerHex::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Center,
                                            flags: 12u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Is(6usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        match match (&420,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::LowerHex::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Left,
                                            flags: 12u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Is(6usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        match match (&420,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::LowerHex::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1_formatted(
                                    &[""],
                                    args,
                                    &[::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Left,
                                            flags: 12u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Is(6usize),
                                        },
                                    }],
                                )
                            },
                        },
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const write: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("outlittests::write"),
            ignore: false,
            allow_fail: false,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(write())),
    };
    fn write() {
        use std::io::Write;
        let mut buffer = Vec::new();
        (&mut buffer)
            .write_fmt(
                match match () {
                    () => [],
                } {
                    ref args => unsafe { ::core::fmt::Arguments::new_v1(&["\n"], args) },
                },
            )
            .unwrap();
        (&mut buffer)
            .write_fmt(
                match match () {
                    () => [],
                } {
                    ref args => unsafe {
                        ::core::fmt::Arguments::new_v1(&["a boring string\n"], args)
                    },
                },
            )
            .unwrap();
        (&mut buffer)
            .write_fmt(
                match match (&(2 + 9),) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::LowerHex::fmt,
                    )],
                } {
                    ref args => unsafe {
                        ::core::fmt::Arguments::new_v1(&["some math 0x", "\n"], args)
                    },
                },
            )
            .unwrap();
        (&mut buffer)
            .write_fmt(
                match match (&(1 + 0),) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                } {
                    ref args => unsafe {
                        ::core::fmt::Arguments::new_v1(&["the ", "st end"], args)
                    },
                },
            )
            .unwrap();
        {
            ::std::io::_print(
                match match (&String::from_utf8(buffer[..].to_vec()).unwrap(),) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
                } {
                    ref args => unsafe { ::core::fmt::Arguments::new_v1(&["", "\n"], args) },
                },
            );
        };
        {
            match (
                &&buffer[..],
                &"\na boring string\nsome math 0xb\nthe 1st end".as_bytes(),
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const internal_lifetimes: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("outlittests::internal_lifetimes"),
            ignore: false,
            allow_fail: false,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(internal_lifetimes())),
    };
    fn internal_lifetimes() {
        let test = "a test";
        {
            match (&"dirty a test", &{
                let res = ::alloc::fmt::format(
                    match match (&{
                        let x: &'static str = test;
                        {
                            let y: &'static str = x;
                            y
                        }
                    },)
                    {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    } {
                        ref args => unsafe { ::core::fmt::Arguments::new_v1(&["dirty "], args) },
                    },
                );
                res
            }) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (&"dirty static str = test; { let y: &", &{
                let res = ::alloc::fmt::format(
                    match match (&{
                        let x = "static str = test; { let y: &";
                        x
                    },)
                    {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    } {
                        ref args => unsafe { ::core::fmt::Arguments::new_v1(&["dirty "], args) },
                    },
                );
                res
            }) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const internal_chars: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("outlittests::internal_chars"),
            ignore: false,
            allow_fail: false,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(internal_chars())),
    };
    fn internal_chars() {
        {
            match (&"an open brace: {", &{
                let res = ::alloc::fmt::format(
                    match match (&'{',) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    } {
                        ref args => unsafe {
                            ::core::fmt::Arguments::new_v1(&["an open brace: "], args)
                        },
                    },
                );
                res
            }) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (&"a close brace: }", &{
                let res = ::alloc::fmt::format(
                    match match (&'}',) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    } {
                        ref args => unsafe {
                            ::core::fmt::Arguments::new_v1(&["a close brace: "], args)
                        },
                    },
                );
                res
            }) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (&"an open brace: {", &{
                let res = ::alloc::fmt::format(
                    match match (&'\u{007b}',) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    } {
                        ref args => unsafe {
                            ::core::fmt::Arguments::new_v1(&["an open brace: "], args)
                        },
                    },
                );
                res
            }) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (&"a close brace: }", &{
                let res = ::alloc::fmt::format(
                    match match (&'\u{007D}',) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    } {
                        ref args => unsafe {
                            ::core::fmt::Arguments::new_v1(&["a close brace: "], args)
                        },
                    },
                );
                res
            }) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (&"dirty dan: '", &{
                let res = ::alloc::fmt::format(
                    match match (&{
                        '\'';
                        {
                            '\''
                        }
                    },)
                    {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    } {
                        ref args => unsafe {
                            ::core::fmt::Arguments::new_v1(&["dirty dan: "], args)
                        },
                    },
                );
                res
            }) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (&"pinhead: \"", &{
                let res = ::alloc::fmt::format(
                    match match (&{
                        '\"';
                        {
                            '\"'
                        }
                    },)
                    {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    } {
                        ref args => unsafe { ::core::fmt::Arguments::new_v1(&["pinhead: "], args) },
                    },
                );
                res
            }) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (&"gary: '", &{
                let res = ::alloc::fmt::format(
                    match match (&{
                        "\'";
                        {
                            "\'"
                        }
                    },)
                    {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    } {
                        ref args => unsafe { ::core::fmt::Arguments::new_v1(&["gary: "], args) },
                    },
                );
                res
            }) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (&"val kilmer: \"", &{
                let res = ::alloc::fmt::format(
                    match match (&{
                        "\"";
                        {
                            "\""
                        }
                    },)
                    {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    } {
                        ref args => unsafe {
                            ::core::fmt::Arguments::new_v1(&["val kilmer: "], args)
                        },
                    },
                );
                res
            }) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (&"lelouch: \"", &{
                let res = ::alloc::fmt::format(
                    match match (&{
                        '"';
                        {
                            '"'
                        }
                    },)
                    {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    } {
                        ref args => unsafe { ::core::fmt::Arguments::new_v1(&["lelouch: "], args) },
                    },
                );
                res
            }) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (&"gon: '", &{
                let res = ::alloc::fmt::format(
                    match match (&{
                        "'";
                        {
                            "'"
                        }
                    },)
                    {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    } {
                        ref args => unsafe { ::core::fmt::Arguments::new_v1(&["gon: "], args) },
                    },
                );
                res
            }) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const borrow: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("outlittests::borrow"),
            ignore: false,
            allow_fail: false,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(borrow())),
    };
    fn borrow() {
        let s = String::from("asdf");
        let v = <[_]>::into_vec(box [s]);
        {
            let res = ::alloc::fmt::format(
                match match (&v[0],) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                } {
                    ref args => unsafe { ::core::fmt::Arguments::new_v1(&["borrow: "], args) },
                },
            );
            res
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const no_alignment_leak: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("outlittests::no_alignment_leak"),
            ignore: false,
            allow_fail: false,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(no_alignment_leak())),
    };
    fn no_alignment_leak() {
        let s = "test";
        {
            match (&"test>", &{
                let res = ::alloc::fmt::format(
                    match match (&s,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    } {
                        ref args => unsafe { ::core::fmt::Arguments::new_v1(&["", ">"], args) },
                    },
                );
                res
            }) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        {
            match (&"test>", &{
                let res = ::alloc::fmt::format(
                    match match (&s,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    } {
                        ref args => unsafe { ::core::fmt::Arguments::new_v1(&["", ">"], args) },
                    },
                );
                res
            }) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const panic: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("outlittests::panic"),
            ignore: false,
            allow_fail: false,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::YesWithMessage("2B || !2b"),
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(panic())),
    };
    #[should_panic(expected = "2B || !2b")]
    fn panic() {
        let tobe = 0x2B;
        {
            ::std::rt::begin_panic_fmt(&match match (&tobe, &tobe) {
                (arg0, arg1) => [
                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::UpperHex::fmt),
                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::LowerHex::fmt),
                ],
            } {
                ref args => unsafe { ::core::fmt::Arguments::new_v1(&["", " || !"], args) },
            })
        };
    }
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[
        &simple_subst,
        &expr_subst,
        &spec_subst,
        &write,
        &internal_lifetimes,
        &internal_chars,
        &borrow,
        &no_alignment_leak,
        &panic,
    ])
}
