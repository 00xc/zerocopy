// Copyright 2019 The Fuchsia Authors
//
// Licensed under a BSD-style license <LICENSE-BSD>, Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0>, or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to
// those terms.

// See comment in `include.rs` for why we disable the prelude.
#![no_implicit_prelude]
#![allow(warnings)]

include!("include.rs");

// A struct is `FromBytes` if:
// - all fields are `FromBytes`

#[derive(imp::FromZeros, imp::FromBytes)]
struct Zst;

util_assert_impl_all!(Zst: imp::FromBytes);

#[derive(imp::FromZeros, imp::FromBytes)]
struct One {
    a: u8,
}

util_assert_impl_all!(One: imp::FromBytes);

#[derive(imp::FromZeros, imp::FromBytes)]
struct Two {
    a: u8,
    b: Zst,
}

util_assert_impl_all!(Two: imp::FromBytes);

#[derive(imp::FromZeros, imp::FromBytes)]
struct Unsized {
    a: [u8],
}

util_assert_impl_all!(Unsized: imp::FromBytes);

#[derive(imp::FromZeros, imp::FromBytes)]
struct TypeParams<'a, T: ?imp::Sized, I: imp::Iterator> {
    a: I::Item,
    b: u8,
    c: imp::PhantomData<&'a [::core::primitive::u8]>,
    d: imp::PhantomData<&'static ::core::primitive::str>,
    e: imp::PhantomData<imp::String>,
    f: T,
}

util_assert_impl_all!(TypeParams<'static, (), imp::IntoIter<()>>: imp::FromBytes);
util_assert_impl_all!(TypeParams<'static, util::AU16, imp::IntoIter<()>>: imp::FromBytes);
util_assert_impl_all!(TypeParams<'static, [util::AU16], imp::IntoIter<()>>: imp::FromBytes);

// Deriving `FromBytes` should work if the struct has bounded parameters.

#[derive(imp::FromZeros, imp::FromBytes)]
#[repr(transparent)]
struct WithParams<'a: 'b, 'b: 'a, T: 'a + 'b + imp::FromBytes, const N: usize>(
    [T; N],
    imp::PhantomData<&'a &'b ()>,
)
where
    'a: 'b,
    'b: 'a,
    T: 'a + 'b + imp::FromBytes;

util_assert_impl_all!(WithParams<'static, 'static, u8, 42>: imp::FromBytes);
