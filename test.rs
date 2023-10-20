#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod tests {
    use std::sync::{Arc, RwLock};
    pub trait TestTrait: Send + Sync {
        fn test_method(&self, arg: u32) -> u32;
        fn test_method_2(&mut self, arg: u32) -> u32;
        fn clone_box(&self) -> Box<dyn TestTrait>;
    }
    #[allow(non_snake_case)]
    #[allow(missing_docs)]
    pub mod __mock_MockTestTrait {
        use super::*;
    }
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[allow(missing_docs)]
    pub struct MockTestTrait {
        TestTrait_expectations: MockTestTrait_TestTrait,
    }
    impl ::std::fmt::Debug for MockTestTrait {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), std::fmt::Error> {
            f.debug_struct("MockTestTrait").finish()
        }
    }
    impl ::std::default::Default for MockTestTrait {
        #[allow(clippy::default_trait_access)]
        fn default() -> Self {
            Self {
                TestTrait_expectations: Default::default(),
            }
        }
    }
    #[allow(non_snake_case)]
    #[allow(missing_docs)]
    pub mod __mock_MockTestTrait_TestTrait {
        use super::*;
        #[allow(missing_docs)]
        pub mod __test_method {
            use super::*;
            use ::mockall::CaseTreeExt;
            use ::std::{
                boxed::Box, mem, ops::{DerefMut, Range},
                sync::Mutex, vec::Vec,
            };
            #[allow(clippy::unused_unit)]
            enum Rfunc {
                Default,
                Expired,
                Mut(Box<dyn FnMut(u32) -> u32 + Send>),
                MutSt(::mockall::Fragile<Box<dyn FnMut(u32) -> u32>>),
                Once(Box<dyn FnOnce(u32) -> u32 + Send>),
                OnceSt(::mockall::Fragile<Box<dyn FnOnce(u32) -> u32>>),
                _Phantom(Box<dyn Fn() + Send>),
            }
            impl Rfunc {
                fn call_mut(
                    &mut self,
                    arg: u32,
                ) -> std::result::Result<u32, &'static str> {
                    match self {
                        Rfunc::Default => {
                            use ::mockall::ReturnDefault;
                            ::mockall::DefaultReturner::<u32>::return_default()
                        }
                        Rfunc::Expired => Err("called twice, but it returns by move"),
                        Rfunc::Mut(__mockall_f) => {
                            ::std::result::Result::Ok(__mockall_f(arg))
                        }
                        Rfunc::MutSt(__mockall_f) => {
                            ::std::result::Result::Ok((__mockall_f.get_mut())(arg))
                        }
                        Rfunc::Once(_) => {
                            if let Rfunc::Once(mut __mockall_f) = mem::replace(
                                self,
                                Rfunc::Expired,
                            ) {
                                ::std::result::Result::Ok(__mockall_f(arg))
                            } else {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                        }
                        Rfunc::OnceSt(_) => {
                            if let Rfunc::OnceSt(mut __mockall_f) = mem::replace(
                                self,
                                Rfunc::Expired,
                            ) {
                                ::std::result::Result::Ok((__mockall_f.into_inner())(arg))
                            } else {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                        }
                        Rfunc::_Phantom(_) => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
            }
            impl std::default::Default for Rfunc {
                fn default() -> Self {
                    Rfunc::Default
                }
            }
            enum Matcher {
                Always,
                Func(Box<dyn Fn(&u32) -> bool + Send>),
                FuncSt(::mockall::Fragile<Box<dyn Fn(&u32) -> bool>>),
                Pred(Box<(Box<dyn ::mockall::Predicate<u32> + Send>,)>),
                _Phantom(Box<dyn Fn() + Send>),
            }
            impl Matcher {
                #[allow(clippy::ptr_arg)]
                fn matches(&self, arg: &u32) -> bool {
                    match self {
                        Matcher::Always => true,
                        Matcher::Func(__mockall_f) => __mockall_f(arg),
                        Matcher::FuncSt(__mockall_f) => (__mockall_f.get())(arg),
                        Matcher::Pred(__mockall_pred) => {
                            [__mockall_pred.0.eval(arg)]
                                .iter()
                                .all(|__mockall_x| *__mockall_x)
                        }
                        _ => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
            }
            impl Default for Matcher {
                #[allow(unused_variables)]
                fn default() -> Self {
                    Matcher::Always
                }
            }
            impl ::std::fmt::Display for Matcher {
                fn fmt(
                    &self,
                    __mockall_fmt: &mut ::std::fmt::Formatter<'_>,
                ) -> ::std::fmt::Result {
                    match self {
                        Matcher::Always => {
                            __mockall_fmt.write_fmt(format_args!("<anything>"))
                        }
                        Matcher::Func(_) => {
                            __mockall_fmt.write_fmt(format_args!("<function>"))
                        }
                        Matcher::FuncSt(_) => {
                            __mockall_fmt
                                .write_fmt(format_args!("<single threaded function>"))
                        }
                        Matcher::Pred(__mockall_p) => {
                            __mockall_fmt.write_fmt(format_args!("{0}", __mockall_p.0))
                        }
                        _ => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
            }
            /// Holds the stuff that is independent of the output type
            struct Common {
                matcher: Mutex<Matcher>,
                seq_handle: Option<::mockall::SeqHandle>,
                times: ::mockall::Times,
            }
            impl std::default::Default for Common {
                fn default() -> Self {
                    Common {
                        matcher: Mutex::new(Matcher::default()),
                        seq_handle: None,
                        times: ::mockall::Times::default(),
                    }
                }
            }
            impl Common {
                fn call(&self, desc: &str) {
                    self.times
                        .call()
                        .unwrap_or_else(|m| {
                            let desc = {
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}", self.matcher.lock().unwrap()),
                                );
                                res
                            };
                            {
                                ::std::rt::panic_fmt(
                                    format_args!(
                                        "{0}: Expectation({1}) {2}",
                                        "MockTestTrait::test_method",
                                        desc,
                                        m,
                                    ),
                                );
                            };
                        });
                    self.verify_sequence(desc);
                    if ::mockall::ExpectedCalls::TooFew != self.times.is_satisfied() {
                        self.satisfy_sequence()
                    }
                }
                fn in_sequence(
                    &mut self,
                    __mockall_seq: &mut ::mockall::Sequence,
                ) -> &mut Self {
                    if !self.times.is_exact() {
                        {
                            ::std::rt::begin_panic(
                                "Only Expectations with an exact call count have sequences",
                            );
                        }
                    }
                    self.seq_handle = Some(__mockall_seq.next_handle());
                    self
                }
                fn is_done(&self) -> bool {
                    self.times.is_done()
                }
                #[allow(clippy::ptr_arg)]
                fn matches(&self, arg: &u32) -> bool {
                    self.matcher.lock().unwrap().matches(arg)
                }
                /// Forbid this expectation from ever being called.
                fn never(&mut self) {
                    self.times.never();
                }
                fn satisfy_sequence(&self) {
                    if let Some(__mockall_handle) = &self.seq_handle {
                        __mockall_handle.satisfy()
                    }
                }
                /// Expect this expectation to be called any number of times
                /// contained with the given range.
                fn times<MockallR>(&mut self, __mockall_r: MockallR)
                where
                    MockallR: Into<::mockall::TimesRange>,
                {
                    self.times.times(__mockall_r)
                }
                fn with<MockallMatcher0: ::mockall::Predicate<u32> + Send + 'static>(
                    &mut self,
                    arg: MockallMatcher0,
                ) {
                    let mut __mockall_guard = self.matcher.lock().unwrap();
                    *__mockall_guard
                        .deref_mut() = Matcher::Pred(Box::new((Box::new(arg),)));
                }
                fn withf<MockallF>(&mut self, __mockall_f: MockallF)
                where
                    MockallF: Fn(&u32) -> bool + Send + 'static,
                {
                    let mut __mockall_guard = self.matcher.lock().unwrap();
                    *__mockall_guard.deref_mut() = Matcher::Func(Box::new(__mockall_f));
                }
                fn withf_st<MockallF>(&mut self, __mockall_f: MockallF)
                where
                    MockallF: Fn(&u32) -> bool + 'static,
                {
                    let mut __mockall_guard = self.matcher.lock().unwrap();
                    *__mockall_guard
                        .deref_mut() = Matcher::FuncSt(
                        ::mockall::Fragile::new(Box::new(__mockall_f)),
                    );
                }
                fn verify_sequence(&self, desc: &str) {
                    if let Some(__mockall_handle) = &self.seq_handle {
                        __mockall_handle.verify(desc)
                    }
                }
            }
            impl Drop for Common {
                fn drop(&mut self) {
                    if !::std::thread::panicking() {
                        let desc = {
                            let res = ::alloc::fmt::format(
                                format_args!("{0}", self.matcher.lock().unwrap()),
                            );
                            res
                        };
                        match self.times.is_satisfied() {
                            ::mockall::ExpectedCalls::TooFew => {
                                {
                                    ::std::rt::panic_fmt(
                                        format_args!(
                                            "{0}: Expectation({1}) called {2} time(s) which is fewer than expected {3}",
                                            "MockTestTrait::test_method",
                                            desc,
                                            self.times.count(),
                                            self.times.minimum(),
                                        ),
                                    );
                                };
                            }
                            ::mockall::ExpectedCalls::TooMany => {
                                {
                                    ::std::rt::panic_fmt(
                                        format_args!(
                                            "{0}: Expectation({1}) called {2} time(s) which is more than expected {3}",
                                            "MockTestTrait::test_method",
                                            desc,
                                            self.times.count(),
                                            self.times.maximum(),
                                        ),
                                    );
                                };
                            }
                            _ => {}
                        }
                    }
                }
            }
            /// Expectation type for methods that return a `'static` type.
            /// This is the type returned by the `expect_*` methods.
            pub struct Expectation {
                common: Common,
                rfunc: Mutex<Rfunc>,
            }
            #[allow(clippy::unused_unit)]
            impl Expectation {
                /// Call this [`Expectation`] as if it were the real method.
                #[doc(hidden)]
                pub fn call(&self, arg: u32) -> u32 {
                    self.common
                        .call(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "MockTestTrait::test_method({0:?})",
                                        ::mockall::MaybeDebugger(&arg),
                                    ),
                                );
                                res
                            },
                        );
                    self.rfunc
                        .lock()
                        .unwrap()
                        .call_mut(arg)
                        .unwrap_or_else(|message| {
                            let desc = {
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}", self.common.matcher.lock().unwrap()),
                                );
                                res
                            };
                            {
                                ::std::rt::panic_fmt(
                                    format_args!(
                                        "{0}: Expectation({1}) {2}",
                                        "MockTestTrait::test_method",
                                        desc,
                                        message,
                                    ),
                                );
                            };
                        })
                }
                /// Return a constant value from the `Expectation`
                ///
                /// The output type must be `Clone`.  The compiler can't always
                /// infer the proper type to use with this method; you will
                /// usually need to specify it explicitly.  i.e.
                /// `return_const(42i32)` instead of `return_const(42)`.
                #[allow(unused_variables)]
                pub fn return_const<MockallOutput>(
                    &mut self,
                    __mockall_c: MockallOutput,
                ) -> &mut Self
                where
                    MockallOutput: Clone + Into<u32> + Send + 'static,
                {
                    self.returning(move |arg| __mockall_c.clone().into())
                }
                /// Single-threaded version of
                /// [`return_const`](#method.return_const).  This is useful for
                /// return types that are not `Send`.
                ///
                /// The output type must be `Clone`.  The compiler can't always
                /// infer the proper type to use with this method; you will
                /// usually need to specify it explicitly.  i.e.
                /// `return_const(42i32)` instead of `return_const(42)`.
                ///
                /// It is a runtime error to call the mock method from a
                /// different thread than the one that originally called this
                /// method.
                #[allow(unused_variables)]
                pub fn return_const_st<MockallOutput>(
                    &mut self,
                    __mockall_c: MockallOutput,
                ) -> &mut Self
                where
                    MockallOutput: Clone + Into<u32> + 'static,
                {
                    self.returning_st(move |arg| __mockall_c.clone().into())
                }
                /// Supply an `FnOnce` closure that will provide the return
                /// value for this Expectation.  This is useful for return types
                /// that aren't `Clone`.  It will be an error to call this
                /// method multiple times.
                pub fn return_once<MockallF>(
                    &mut self,
                    __mockall_f: MockallF,
                ) -> &mut Self
                where
                    MockallF: FnOnce(u32) -> u32 + Send + 'static,
                {
                    {
                        let mut __mockall_guard = self.rfunc.lock().unwrap();
                        *__mockall_guard
                            .deref_mut() = Rfunc::Once(Box::new(__mockall_f));
                    }
                    self
                }
                /// Single-threaded version of
                /// [`return_once`](#method.return_once).  This is useful for
                /// return types that are neither `Send` nor `Clone`.
                ///
                /// It is a runtime error to call the mock method from a
                /// different thread than the one that originally called this
                /// method.  It is also a runtime error to call the method more
                /// than once.
                pub fn return_once_st<MockallF>(
                    &mut self,
                    __mockall_f: MockallF,
                ) -> &mut Self
                where
                    MockallF: FnOnce(u32) -> u32 + 'static,
                {
                    {
                        let mut __mockall_guard = self.rfunc.lock().unwrap();
                        *__mockall_guard
                            .deref_mut() = Rfunc::OnceSt(
                            ::mockall::Fragile::new(Box::new(__mockall_f)),
                        );
                    }
                    self
                }
                /// Supply a closure that will provide the return value for this
                /// `Expectation`.  The method's arguments are passed to the
                /// closure by value.
                pub fn returning<MockallF>(&mut self, __mockall_f: MockallF) -> &mut Self
                where
                    MockallF: FnMut(u32) -> u32 + Send + 'static,
                {
                    {
                        let mut __mockall_guard = self.rfunc.lock().unwrap();
                        *__mockall_guard.deref_mut() = Rfunc::Mut(Box::new(__mockall_f));
                    }
                    self
                }
                /// Single-threaded version of [`returning`](#method.returning).
                /// Can be used when the argument or return type isn't `Send`.
                ///
                /// It is a runtime error to call the mock method from a
                /// different thread than the one that originally called this
                /// method.
                pub fn returning_st<MockallF>(
                    &mut self,
                    __mockall_f: MockallF,
                ) -> &mut Self
                where
                    MockallF: FnMut(u32) -> u32 + 'static,
                {
                    {
                        let mut __mockall_guard = self.rfunc.lock().unwrap();
                        *__mockall_guard
                            .deref_mut() = Rfunc::MutSt(
                            ::mockall::Fragile::new(Box::new(__mockall_f)),
                        );
                    }
                    self
                }
                /// Add this expectation to a
                /// [`Sequence`](../../../mockall/struct.Sequence.html).
                pub fn in_sequence(
                    &mut self,
                    __mockall_seq: &mut ::mockall::Sequence,
                ) -> &mut Self {
                    self.common.in_sequence(__mockall_seq);
                    self
                }
                fn is_done(&self) -> bool {
                    self.common.is_done()
                }
                /// Validate this expectation's matcher.
                #[allow(clippy::ptr_arg)]
                fn matches(&self, arg: &u32) -> bool {
                    self.common.matches(arg)
                }
                /// Forbid this expectation from ever being called.
                pub fn never(&mut self) -> &mut Self {
                    self.common.never();
                    self
                }
                /// Create a new, default, [`Expectation`](struct.Expectation.html)
                pub fn new() -> Self {
                    Self::default()
                }
                /// Expect this expectation to be called exactly once.  Shortcut for
                /// [`times(1)`](#method.times).
                pub fn once(&mut self) -> &mut Self {
                    self.times(1)
                }
                /// Restrict the number of times that that this method may be called.
                ///
                /// The argument may be:
                /// * A fixed number: `.times(4)`
                /// * Various types of range:
                ///   - `.times(5..10)`
                ///   - `.times(..10)`
                ///   - `.times(5..)`
                ///   - `.times(5..=10)`
                ///   - `.times(..=10)`
                /// * The wildcard: `.times(..)`
                pub fn times<MockallR>(&mut self, __mockall_r: MockallR) -> &mut Self
                where
                    MockallR: Into<::mockall::TimesRange>,
                {
                    self.common.times(__mockall_r);
                    self
                }
                /// Set matching crieteria for this Expectation.
                ///
                /// The matching predicate can be anything implemening the
                /// [`Predicate`](../../../mockall/trait.Predicate.html) trait.  Only
                /// one matcher can be set per `Expectation` at a time.
                pub fn with<MockallMatcher0: ::mockall::Predicate<u32> + Send + 'static>(
                    &mut self,
                    arg: MockallMatcher0,
                ) -> &mut Self {
                    self.common.with(arg);
                    self
                }
                /// Set a matching function for this Expectation.
                ///
                /// This is equivalent to calling [`with`](#method.with) with a
                /// function argument, like `with(predicate::function(f))`.
                pub fn withf<MockallF>(&mut self, __mockall_f: MockallF) -> &mut Self
                where
                    MockallF: Fn(&u32) -> bool + Send + 'static,
                {
                    self.common.withf(__mockall_f);
                    self
                }
                /// Single-threaded version of [`withf`](#method.withf).
                /// Can be used when the argument type isn't `Send`.
                pub fn withf_st<MockallF>(&mut self, __mockall_f: MockallF) -> &mut Self
                where
                    MockallF: Fn(&u32) -> bool + 'static,
                {
                    self.common.withf_st(__mockall_f);
                    self
                }
            }
            impl Default for Expectation {
                fn default() -> Self {
                    Expectation {
                        common: Common::default(),
                        rfunc: Mutex::new(Rfunc::default()),
                    }
                }
            }
            /// A collection of [`Expectation`](struct.Expectations.html)
            /// objects.  Users will rarely if ever use this struct directly.
            #[doc(hidden)]
            pub struct Expectations(Vec<Expectation>);
            impl Expectations {
                /// Verify that all current expectations are satisfied and clear
                /// them.
                pub fn checkpoint(&mut self) -> std::vec::Drain<Expectation> {
                    self.0.drain(..)
                }
                /// Create a new expectation for this method.
                pub fn expect(&mut self) -> &mut Expectation {
                    self.0.push(Expectation::default());
                    let __mockall_l = self.0.len();
                    &mut self.0[__mockall_l - 1]
                }
                pub fn new() -> Self {
                    Self::default()
                }
            }
            impl Default for Expectations {
                fn default() -> Self {
                    Expectations(Vec::new())
                }
            }
            impl Expectations {
                /// Simulate calling the real method.  Every current expectation
                /// will be checked in FIFO order and the first one with
                /// matching arguments will be used.
                pub fn call(&self, arg: u32) -> Option<u32> {
                    self.0
                        .iter()
                        .find(|__mockall_e| {
                            __mockall_e.matches(&arg)
                                && (!__mockall_e.is_done() || self.0.len() == 1)
                        })
                        .map(move |__mockall_e| __mockall_e.call(arg))
                }
            }
        }
        #[allow(missing_docs)]
        pub mod __test_method_2 {
            use super::*;
            use ::mockall::CaseTreeExt;
            use ::std::{
                boxed::Box, mem, ops::{DerefMut, Range},
                sync::Mutex, vec::Vec,
            };
            #[allow(clippy::unused_unit)]
            enum Rfunc {
                Default,
                Expired,
                Mut(Box<dyn FnMut(u32) -> u32 + Send>),
                MutSt(::mockall::Fragile<Box<dyn FnMut(u32) -> u32>>),
                Once(Box<dyn FnOnce(u32) -> u32 + Send>),
                OnceSt(::mockall::Fragile<Box<dyn FnOnce(u32) -> u32>>),
                _Phantom(Box<dyn Fn() + Send>),
            }
            impl Rfunc {
                fn call_mut(
                    &mut self,
                    arg: u32,
                ) -> std::result::Result<u32, &'static str> {
                    match self {
                        Rfunc::Default => {
                            use ::mockall::ReturnDefault;
                            ::mockall::DefaultReturner::<u32>::return_default()
                        }
                        Rfunc::Expired => Err("called twice, but it returns by move"),
                        Rfunc::Mut(__mockall_f) => {
                            ::std::result::Result::Ok(__mockall_f(arg))
                        }
                        Rfunc::MutSt(__mockall_f) => {
                            ::std::result::Result::Ok((__mockall_f.get_mut())(arg))
                        }
                        Rfunc::Once(_) => {
                            if let Rfunc::Once(mut __mockall_f) = mem::replace(
                                self,
                                Rfunc::Expired,
                            ) {
                                ::std::result::Result::Ok(__mockall_f(arg))
                            } else {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                        }
                        Rfunc::OnceSt(_) => {
                            if let Rfunc::OnceSt(mut __mockall_f) = mem::replace(
                                self,
                                Rfunc::Expired,
                            ) {
                                ::std::result::Result::Ok((__mockall_f.into_inner())(arg))
                            } else {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                        }
                        Rfunc::_Phantom(_) => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
            }
            impl std::default::Default for Rfunc {
                fn default() -> Self {
                    Rfunc::Default
                }
            }
            enum Matcher {
                Always,
                Func(Box<dyn Fn(&u32) -> bool + Send>),
                FuncSt(::mockall::Fragile<Box<dyn Fn(&u32) -> bool>>),
                Pred(Box<(Box<dyn ::mockall::Predicate<u32> + Send>,)>),
                _Phantom(Box<dyn Fn() + Send>),
            }
            impl Matcher {
                #[allow(clippy::ptr_arg)]
                fn matches(&self, arg: &u32) -> bool {
                    match self {
                        Matcher::Always => true,
                        Matcher::Func(__mockall_f) => __mockall_f(arg),
                        Matcher::FuncSt(__mockall_f) => (__mockall_f.get())(arg),
                        Matcher::Pred(__mockall_pred) => {
                            [__mockall_pred.0.eval(arg)]
                                .iter()
                                .all(|__mockall_x| *__mockall_x)
                        }
                        _ => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
            }
            impl Default for Matcher {
                #[allow(unused_variables)]
                fn default() -> Self {
                    Matcher::Always
                }
            }
            impl ::std::fmt::Display for Matcher {
                fn fmt(
                    &self,
                    __mockall_fmt: &mut ::std::fmt::Formatter<'_>,
                ) -> ::std::fmt::Result {
                    match self {
                        Matcher::Always => {
                            __mockall_fmt.write_fmt(format_args!("<anything>"))
                        }
                        Matcher::Func(_) => {
                            __mockall_fmt.write_fmt(format_args!("<function>"))
                        }
                        Matcher::FuncSt(_) => {
                            __mockall_fmt
                                .write_fmt(format_args!("<single threaded function>"))
                        }
                        Matcher::Pred(__mockall_p) => {
                            __mockall_fmt.write_fmt(format_args!("{0}", __mockall_p.0))
                        }
                        _ => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
            }
            /// Holds the stuff that is independent of the output type
            struct Common {
                matcher: Mutex<Matcher>,
                seq_handle: Option<::mockall::SeqHandle>,
                times: ::mockall::Times,
            }
            impl std::default::Default for Common {
                fn default() -> Self {
                    Common {
                        matcher: Mutex::new(Matcher::default()),
                        seq_handle: None,
                        times: ::mockall::Times::default(),
                    }
                }
            }
            impl Common {
                fn call(&self, desc: &str) {
                    self.times
                        .call()
                        .unwrap_or_else(|m| {
                            let desc = {
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}", self.matcher.lock().unwrap()),
                                );
                                res
                            };
                            {
                                ::std::rt::panic_fmt(
                                    format_args!(
                                        "{0}: Expectation({1}) {2}",
                                        "MockTestTrait::test_method_2",
                                        desc,
                                        m,
                                    ),
                                );
                            };
                        });
                    self.verify_sequence(desc);
                    if ::mockall::ExpectedCalls::TooFew != self.times.is_satisfied() {
                        self.satisfy_sequence()
                    }
                }
                fn in_sequence(
                    &mut self,
                    __mockall_seq: &mut ::mockall::Sequence,
                ) -> &mut Self {
                    if !self.times.is_exact() {
                        {
                            ::std::rt::begin_panic(
                                "Only Expectations with an exact call count have sequences",
                            );
                        }
                    }
                    self.seq_handle = Some(__mockall_seq.next_handle());
                    self
                }
                fn is_done(&self) -> bool {
                    self.times.is_done()
                }
                #[allow(clippy::ptr_arg)]
                fn matches(&self, arg: &u32) -> bool {
                    self.matcher.lock().unwrap().matches(arg)
                }
                /// Forbid this expectation from ever being called.
                fn never(&mut self) {
                    self.times.never();
                }
                fn satisfy_sequence(&self) {
                    if let Some(__mockall_handle) = &self.seq_handle {
                        __mockall_handle.satisfy()
                    }
                }
                /// Expect this expectation to be called any number of times
                /// contained with the given range.
                fn times<MockallR>(&mut self, __mockall_r: MockallR)
                where
                    MockallR: Into<::mockall::TimesRange>,
                {
                    self.times.times(__mockall_r)
                }
                fn with<MockallMatcher0: ::mockall::Predicate<u32> + Send + 'static>(
                    &mut self,
                    arg: MockallMatcher0,
                ) {
                    let mut __mockall_guard = self.matcher.lock().unwrap();
                    *__mockall_guard
                        .deref_mut() = Matcher::Pred(Box::new((Box::new(arg),)));
                }
                fn withf<MockallF>(&mut self, __mockall_f: MockallF)
                where
                    MockallF: Fn(&u32) -> bool + Send + 'static,
                {
                    let mut __mockall_guard = self.matcher.lock().unwrap();
                    *__mockall_guard.deref_mut() = Matcher::Func(Box::new(__mockall_f));
                }
                fn withf_st<MockallF>(&mut self, __mockall_f: MockallF)
                where
                    MockallF: Fn(&u32) -> bool + 'static,
                {
                    let mut __mockall_guard = self.matcher.lock().unwrap();
                    *__mockall_guard
                        .deref_mut() = Matcher::FuncSt(
                        ::mockall::Fragile::new(Box::new(__mockall_f)),
                    );
                }
                fn verify_sequence(&self, desc: &str) {
                    if let Some(__mockall_handle) = &self.seq_handle {
                        __mockall_handle.verify(desc)
                    }
                }
            }
            impl Drop for Common {
                fn drop(&mut self) {
                    if !::std::thread::panicking() {
                        let desc = {
                            let res = ::alloc::fmt::format(
                                format_args!("{0}", self.matcher.lock().unwrap()),
                            );
                            res
                        };
                        match self.times.is_satisfied() {
                            ::mockall::ExpectedCalls::TooFew => {
                                {
                                    ::std::rt::panic_fmt(
                                        format_args!(
                                            "{0}: Expectation({1}) called {2} time(s) which is fewer than expected {3}",
                                            "MockTestTrait::test_method_2",
                                            desc,
                                            self.times.count(),
                                            self.times.minimum(),
                                        ),
                                    );
                                };
                            }
                            ::mockall::ExpectedCalls::TooMany => {
                                {
                                    ::std::rt::panic_fmt(
                                        format_args!(
                                            "{0}: Expectation({1}) called {2} time(s) which is more than expected {3}",
                                            "MockTestTrait::test_method_2",
                                            desc,
                                            self.times.count(),
                                            self.times.maximum(),
                                        ),
                                    );
                                };
                            }
                            _ => {}
                        }
                    }
                }
            }
            /// Expectation type for methods that return a `'static` type.
            /// This is the type returned by the `expect_*` methods.
            pub struct Expectation {
                common: Common,
                rfunc: Mutex<Rfunc>,
            }
            #[allow(clippy::unused_unit)]
            impl Expectation {
                /// Call this [`Expectation`] as if it were the real method.
                #[doc(hidden)]
                pub fn call(&self, arg: u32) -> u32 {
                    self.common
                        .call(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "MockTestTrait::test_method_2({0:?})",
                                        ::mockall::MaybeDebugger(&arg),
                                    ),
                                );
                                res
                            },
                        );
                    self.rfunc
                        .lock()
                        .unwrap()
                        .call_mut(arg)
                        .unwrap_or_else(|message| {
                            let desc = {
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}", self.common.matcher.lock().unwrap()),
                                );
                                res
                            };
                            {
                                ::std::rt::panic_fmt(
                                    format_args!(
                                        "{0}: Expectation({1}) {2}",
                                        "MockTestTrait::test_method_2",
                                        desc,
                                        message,
                                    ),
                                );
                            };
                        })
                }
                /// Return a constant value from the `Expectation`
                ///
                /// The output type must be `Clone`.  The compiler can't always
                /// infer the proper type to use with this method; you will
                /// usually need to specify it explicitly.  i.e.
                /// `return_const(42i32)` instead of `return_const(42)`.
                #[allow(unused_variables)]
                pub fn return_const<MockallOutput>(
                    &mut self,
                    __mockall_c: MockallOutput,
                ) -> &mut Self
                where
                    MockallOutput: Clone + Into<u32> + Send + 'static,
                {
                    self.returning(move |arg| __mockall_c.clone().into())
                }
                /// Single-threaded version of
                /// [`return_const`](#method.return_const).  This is useful for
                /// return types that are not `Send`.
                ///
                /// The output type must be `Clone`.  The compiler can't always
                /// infer the proper type to use with this method; you will
                /// usually need to specify it explicitly.  i.e.
                /// `return_const(42i32)` instead of `return_const(42)`.
                ///
                /// It is a runtime error to call the mock method from a
                /// different thread than the one that originally called this
                /// method.
                #[allow(unused_variables)]
                pub fn return_const_st<MockallOutput>(
                    &mut self,
                    __mockall_c: MockallOutput,
                ) -> &mut Self
                where
                    MockallOutput: Clone + Into<u32> + 'static,
                {
                    self.returning_st(move |arg| __mockall_c.clone().into())
                }
                /// Supply an `FnOnce` closure that will provide the return
                /// value for this Expectation.  This is useful for return types
                /// that aren't `Clone`.  It will be an error to call this
                /// method multiple times.
                pub fn return_once<MockallF>(
                    &mut self,
                    __mockall_f: MockallF,
                ) -> &mut Self
                where
                    MockallF: FnOnce(u32) -> u32 + Send + 'static,
                {
                    {
                        let mut __mockall_guard = self.rfunc.lock().unwrap();
                        *__mockall_guard
                            .deref_mut() = Rfunc::Once(Box::new(__mockall_f));
                    }
                    self
                }
                /// Single-threaded version of
                /// [`return_once`](#method.return_once).  This is useful for
                /// return types that are neither `Send` nor `Clone`.
                ///
                /// It is a runtime error to call the mock method from a
                /// different thread than the one that originally called this
                /// method.  It is also a runtime error to call the method more
                /// than once.
                pub fn return_once_st<MockallF>(
                    &mut self,
                    __mockall_f: MockallF,
                ) -> &mut Self
                where
                    MockallF: FnOnce(u32) -> u32 + 'static,
                {
                    {
                        let mut __mockall_guard = self.rfunc.lock().unwrap();
                        *__mockall_guard
                            .deref_mut() = Rfunc::OnceSt(
                            ::mockall::Fragile::new(Box::new(__mockall_f)),
                        );
                    }
                    self
                }
                /// Supply a closure that will provide the return value for this
                /// `Expectation`.  The method's arguments are passed to the
                /// closure by value.
                pub fn returning<MockallF>(&mut self, __mockall_f: MockallF) -> &mut Self
                where
                    MockallF: FnMut(u32) -> u32 + Send + 'static,
                {
                    {
                        let mut __mockall_guard = self.rfunc.lock().unwrap();
                        *__mockall_guard.deref_mut() = Rfunc::Mut(Box::new(__mockall_f));
                    }
                    self
                }
                /// Single-threaded version of [`returning`](#method.returning).
                /// Can be used when the argument or return type isn't `Send`.
                ///
                /// It is a runtime error to call the mock method from a
                /// different thread than the one that originally called this
                /// method.
                pub fn returning_st<MockallF>(
                    &mut self,
                    __mockall_f: MockallF,
                ) -> &mut Self
                where
                    MockallF: FnMut(u32) -> u32 + 'static,
                {
                    {
                        let mut __mockall_guard = self.rfunc.lock().unwrap();
                        *__mockall_guard
                            .deref_mut() = Rfunc::MutSt(
                            ::mockall::Fragile::new(Box::new(__mockall_f)),
                        );
                    }
                    self
                }
                /// Add this expectation to a
                /// [`Sequence`](../../../mockall/struct.Sequence.html).
                pub fn in_sequence(
                    &mut self,
                    __mockall_seq: &mut ::mockall::Sequence,
                ) -> &mut Self {
                    self.common.in_sequence(__mockall_seq);
                    self
                }
                fn is_done(&self) -> bool {
                    self.common.is_done()
                }
                /// Validate this expectation's matcher.
                #[allow(clippy::ptr_arg)]
                fn matches(&self, arg: &u32) -> bool {
                    self.common.matches(arg)
                }
                /// Forbid this expectation from ever being called.
                pub fn never(&mut self) -> &mut Self {
                    self.common.never();
                    self
                }
                /// Create a new, default, [`Expectation`](struct.Expectation.html)
                pub fn new() -> Self {
                    Self::default()
                }
                /// Expect this expectation to be called exactly once.  Shortcut for
                /// [`times(1)`](#method.times).
                pub fn once(&mut self) -> &mut Self {
                    self.times(1)
                }
                /// Restrict the number of times that that this method may be called.
                ///
                /// The argument may be:
                /// * A fixed number: `.times(4)`
                /// * Various types of range:
                ///   - `.times(5..10)`
                ///   - `.times(..10)`
                ///   - `.times(5..)`
                ///   - `.times(5..=10)`
                ///   - `.times(..=10)`
                /// * The wildcard: `.times(..)`
                pub fn times<MockallR>(&mut self, __mockall_r: MockallR) -> &mut Self
                where
                    MockallR: Into<::mockall::TimesRange>,
                {
                    self.common.times(__mockall_r);
                    self
                }
                /// Set matching crieteria for this Expectation.
                ///
                /// The matching predicate can be anything implemening the
                /// [`Predicate`](../../../mockall/trait.Predicate.html) trait.  Only
                /// one matcher can be set per `Expectation` at a time.
                pub fn with<MockallMatcher0: ::mockall::Predicate<u32> + Send + 'static>(
                    &mut self,
                    arg: MockallMatcher0,
                ) -> &mut Self {
                    self.common.with(arg);
                    self
                }
                /// Set a matching function for this Expectation.
                ///
                /// This is equivalent to calling [`with`](#method.with) with a
                /// function argument, like `with(predicate::function(f))`.
                pub fn withf<MockallF>(&mut self, __mockall_f: MockallF) -> &mut Self
                where
                    MockallF: Fn(&u32) -> bool + Send + 'static,
                {
                    self.common.withf(__mockall_f);
                    self
                }
                /// Single-threaded version of [`withf`](#method.withf).
                /// Can be used when the argument type isn't `Send`.
                pub fn withf_st<MockallF>(&mut self, __mockall_f: MockallF) -> &mut Self
                where
                    MockallF: Fn(&u32) -> bool + 'static,
                {
                    self.common.withf_st(__mockall_f);
                    self
                }
            }
            impl Default for Expectation {
                fn default() -> Self {
                    Expectation {
                        common: Common::default(),
                        rfunc: Mutex::new(Rfunc::default()),
                    }
                }
            }
            /// A collection of [`Expectation`](struct.Expectations.html)
            /// objects.  Users will rarely if ever use this struct directly.
            #[doc(hidden)]
            pub struct Expectations(Vec<Expectation>);
            impl Expectations {
                /// Verify that all current expectations are satisfied and clear
                /// them.
                pub fn checkpoint(&mut self) -> std::vec::Drain<Expectation> {
                    self.0.drain(..)
                }
                /// Create a new expectation for this method.
                pub fn expect(&mut self) -> &mut Expectation {
                    self.0.push(Expectation::default());
                    let __mockall_l = self.0.len();
                    &mut self.0[__mockall_l - 1]
                }
                pub fn new() -> Self {
                    Self::default()
                }
            }
            impl Default for Expectations {
                fn default() -> Self {
                    Expectations(Vec::new())
                }
            }
            impl Expectations {
                /// Simulate calling the real method.  Every current expectation
                /// will be checked in FIFO order and the first one with
                /// matching arguments will be used.
                pub fn call(&self, arg: u32) -> Option<u32> {
                    self.0
                        .iter()
                        .find(|__mockall_e| {
                            __mockall_e.matches(&arg)
                                && (!__mockall_e.is_done() || self.0.len() == 1)
                        })
                        .map(move |__mockall_e| __mockall_e.call(arg))
                }
            }
        }
        #[allow(missing_docs)]
        pub mod __clone_box {
            use super::*;
            use ::mockall::CaseTreeExt;
            use ::std::{
                boxed::Box, mem, ops::{DerefMut, Range},
                sync::Mutex, vec::Vec,
            };
            #[allow(clippy::unused_unit)]
            enum Rfunc {
                Default,
                Expired,
                Mut(Box<dyn FnMut() -> Box<dyn TestTrait> + Send>),
                MutSt(::mockall::Fragile<Box<dyn FnMut() -> Box<dyn TestTrait>>>),
                Once(Box<dyn FnOnce() -> Box<dyn TestTrait> + Send>),
                OnceSt(::mockall::Fragile<Box<dyn FnOnce() -> Box<dyn TestTrait>>>),
                _Phantom(Box<dyn Fn() + Send>),
            }
            impl Rfunc {
                fn call_mut(
                    &mut self,
                ) -> std::result::Result<Box<dyn TestTrait>, &'static str> {
                    match self {
                        Rfunc::Default => {
                            use ::mockall::ReturnDefault;
                            ::mockall::DefaultReturner::<
                                Box<dyn TestTrait>,
                            >::return_default()
                        }
                        Rfunc::Expired => Err("called twice, but it returns by move"),
                        Rfunc::Mut(__mockall_f) => {
                            ::std::result::Result::Ok(__mockall_f())
                        }
                        Rfunc::MutSt(__mockall_f) => {
                            ::std::result::Result::Ok((__mockall_f.get_mut())())
                        }
                        Rfunc::Once(_) => {
                            if let Rfunc::Once(mut __mockall_f) = mem::replace(
                                self,
                                Rfunc::Expired,
                            ) {
                                ::std::result::Result::Ok(__mockall_f())
                            } else {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                        }
                        Rfunc::OnceSt(_) => {
                            if let Rfunc::OnceSt(mut __mockall_f) = mem::replace(
                                self,
                                Rfunc::Expired,
                            ) {
                                ::std::result::Result::Ok((__mockall_f.into_inner())())
                            } else {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                        }
                        Rfunc::_Phantom(_) => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
            }
            impl std::default::Default for Rfunc {
                fn default() -> Self {
                    Rfunc::Default
                }
            }
            enum Matcher {
                Always,
                Func(Box<dyn Fn() -> bool + Send>),
                FuncSt(::mockall::Fragile<Box<dyn Fn() -> bool>>),
                Pred(Box<()>),
                _Phantom(Box<dyn Fn() + Send>),
            }
            impl Matcher {
                #[allow(clippy::ptr_arg)]
                fn matches(&self) -> bool {
                    match self {
                        Matcher::Always => true,
                        Matcher::Func(__mockall_f) => __mockall_f(),
                        Matcher::FuncSt(__mockall_f) => (__mockall_f.get())(),
                        Matcher::Pred(__mockall_pred) => {
                            [].iter().all(|__mockall_x| *__mockall_x)
                        }
                        _ => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
            }
            impl Default for Matcher {
                #[allow(unused_variables)]
                fn default() -> Self {
                    Matcher::Always
                }
            }
            impl ::std::fmt::Display for Matcher {
                fn fmt(
                    &self,
                    __mockall_fmt: &mut ::std::fmt::Formatter<'_>,
                ) -> ::std::fmt::Result {
                    match self {
                        Matcher::Always => {
                            __mockall_fmt.write_fmt(format_args!("<anything>"))
                        }
                        Matcher::Func(_) => {
                            __mockall_fmt.write_fmt(format_args!("<function>"))
                        }
                        Matcher::FuncSt(_) => {
                            __mockall_fmt
                                .write_fmt(format_args!("<single threaded function>"))
                        }
                        Matcher::Pred(__mockall_p) => {
                            __mockall_fmt.write_fmt(format_args!(""))
                        }
                        _ => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
            }
            /// Holds the stuff that is independent of the output type
            struct Common {
                matcher: Mutex<Matcher>,
                seq_handle: Option<::mockall::SeqHandle>,
                times: ::mockall::Times,
            }
            impl std::default::Default for Common {
                fn default() -> Self {
                    Common {
                        matcher: Mutex::new(Matcher::default()),
                        seq_handle: None,
                        times: ::mockall::Times::default(),
                    }
                }
            }
            impl Common {
                fn call(&self, desc: &str) {
                    self.times
                        .call()
                        .unwrap_or_else(|m| {
                            let desc = {
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}", self.matcher.lock().unwrap()),
                                );
                                res
                            };
                            {
                                ::std::rt::panic_fmt(
                                    format_args!(
                                        "{0}: Expectation({1}) {2}",
                                        "MockTestTrait::clone_box",
                                        desc,
                                        m,
                                    ),
                                );
                            };
                        });
                    self.verify_sequence(desc);
                    if ::mockall::ExpectedCalls::TooFew != self.times.is_satisfied() {
                        self.satisfy_sequence()
                    }
                }
                fn in_sequence(
                    &mut self,
                    __mockall_seq: &mut ::mockall::Sequence,
                ) -> &mut Self {
                    if !self.times.is_exact() {
                        {
                            ::std::rt::begin_panic(
                                "Only Expectations with an exact call count have sequences",
                            );
                        }
                    }
                    self.seq_handle = Some(__mockall_seq.next_handle());
                    self
                }
                fn is_done(&self) -> bool {
                    self.times.is_done()
                }
                #[allow(clippy::ptr_arg)]
                fn matches(&self) -> bool {
                    self.matcher.lock().unwrap().matches()
                }
                /// Forbid this expectation from ever being called.
                fn never(&mut self) {
                    self.times.never();
                }
                fn satisfy_sequence(&self) {
                    if let Some(__mockall_handle) = &self.seq_handle {
                        __mockall_handle.satisfy()
                    }
                }
                /// Expect this expectation to be called any number of times
                /// contained with the given range.
                fn times<MockallR>(&mut self, __mockall_r: MockallR)
                where
                    MockallR: Into<::mockall::TimesRange>,
                {
                    self.times.times(__mockall_r)
                }
                fn with(&mut self) {
                    let mut __mockall_guard = self.matcher.lock().unwrap();
                    *__mockall_guard.deref_mut() = Matcher::Pred(Box::new(()));
                }
                fn withf<MockallF>(&mut self, __mockall_f: MockallF)
                where
                    MockallF: Fn() -> bool + Send + 'static,
                {
                    let mut __mockall_guard = self.matcher.lock().unwrap();
                    *__mockall_guard.deref_mut() = Matcher::Func(Box::new(__mockall_f));
                }
                fn withf_st<MockallF>(&mut self, __mockall_f: MockallF)
                where
                    MockallF: Fn() -> bool + 'static,
                {
                    let mut __mockall_guard = self.matcher.lock().unwrap();
                    *__mockall_guard
                        .deref_mut() = Matcher::FuncSt(
                        ::mockall::Fragile::new(Box::new(__mockall_f)),
                    );
                }
                fn verify_sequence(&self, desc: &str) {
                    if let Some(__mockall_handle) = &self.seq_handle {
                        __mockall_handle.verify(desc)
                    }
                }
            }
            impl Drop for Common {
                fn drop(&mut self) {
                    if !::std::thread::panicking() {
                        let desc = {
                            let res = ::alloc::fmt::format(
                                format_args!("{0}", self.matcher.lock().unwrap()),
                            );
                            res
                        };
                        match self.times.is_satisfied() {
                            ::mockall::ExpectedCalls::TooFew => {
                                {
                                    ::std::rt::panic_fmt(
                                        format_args!(
                                            "{0}: Expectation({1}) called {2} time(s) which is fewer than expected {3}",
                                            "MockTestTrait::clone_box",
                                            desc,
                                            self.times.count(),
                                            self.times.minimum(),
                                        ),
                                    );
                                };
                            }
                            ::mockall::ExpectedCalls::TooMany => {
                                {
                                    ::std::rt::panic_fmt(
                                        format_args!(
                                            "{0}: Expectation({1}) called {2} time(s) which is more than expected {3}",
                                            "MockTestTrait::clone_box",
                                            desc,
                                            self.times.count(),
                                            self.times.maximum(),
                                        ),
                                    );
                                };
                            }
                            _ => {}
                        }
                    }
                }
            }
            /// Expectation type for methods that return a `'static` type.
            /// This is the type returned by the `expect_*` methods.
            pub struct Expectation {
                common: Common,
                rfunc: Mutex<Rfunc>,
            }
            #[allow(clippy::unused_unit)]
            impl Expectation {
                /// Call this [`Expectation`] as if it were the real method.
                #[doc(hidden)]
                pub fn call(&self) -> Box<dyn TestTrait> {
                    self.common
                        .call(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!("MockTestTrait::clone_box()"),
                                );
                                res
                            },
                        );
                    self.rfunc
                        .lock()
                        .unwrap()
                        .call_mut()
                        .unwrap_or_else(|message| {
                            let desc = {
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}", self.common.matcher.lock().unwrap()),
                                );
                                res
                            };
                            {
                                ::std::rt::panic_fmt(
                                    format_args!(
                                        "{0}: Expectation({1}) {2}",
                                        "MockTestTrait::clone_box",
                                        desc,
                                        message,
                                    ),
                                );
                            };
                        })
                }
                /// Return a constant value from the `Expectation`
                ///
                /// The output type must be `Clone`.  The compiler can't always
                /// infer the proper type to use with this method; you will
                /// usually need to specify it explicitly.  i.e.
                /// `return_const(42i32)` instead of `return_const(42)`.
                #[allow(unused_variables)]
                pub fn return_const<MockallOutput>(
                    &mut self,
                    __mockall_c: MockallOutput,
                ) -> &mut Self
                where
                    MockallOutput: Clone + Into<Box<dyn TestTrait>> + Send + 'static,
                {
                    self.returning(move || __mockall_c.clone().into())
                }
                /// Single-threaded version of
                /// [`return_const`](#method.return_const).  This is useful for
                /// return types that are not `Send`.
                ///
                /// The output type must be `Clone`.  The compiler can't always
                /// infer the proper type to use with this method; you will
                /// usually need to specify it explicitly.  i.e.
                /// `return_const(42i32)` instead of `return_const(42)`.
                ///
                /// It is a runtime error to call the mock method from a
                /// different thread than the one that originally called this
                /// method.
                #[allow(unused_variables)]
                pub fn return_const_st<MockallOutput>(
                    &mut self,
                    __mockall_c: MockallOutput,
                ) -> &mut Self
                where
                    MockallOutput: Clone + Into<Box<dyn TestTrait>> + 'static,
                {
                    self.returning_st(move || __mockall_c.clone().into())
                }
                /// Supply an `FnOnce` closure that will provide the return
                /// value for this Expectation.  This is useful for return types
                /// that aren't `Clone`.  It will be an error to call this
                /// method multiple times.
                pub fn return_once<MockallF>(
                    &mut self,
                    __mockall_f: MockallF,
                ) -> &mut Self
                where
                    MockallF: FnOnce() -> Box<dyn TestTrait> + Send + 'static,
                {
                    {
                        let mut __mockall_guard = self.rfunc.lock().unwrap();
                        *__mockall_guard
                            .deref_mut() = Rfunc::Once(Box::new(__mockall_f));
                    }
                    self
                }
                /// Single-threaded version of
                /// [`return_once`](#method.return_once).  This is useful for
                /// return types that are neither `Send` nor `Clone`.
                ///
                /// It is a runtime error to call the mock method from a
                /// different thread than the one that originally called this
                /// method.  It is also a runtime error to call the method more
                /// than once.
                pub fn return_once_st<MockallF>(
                    &mut self,
                    __mockall_f: MockallF,
                ) -> &mut Self
                where
                    MockallF: FnOnce() -> Box<dyn TestTrait> + 'static,
                {
                    {
                        let mut __mockall_guard = self.rfunc.lock().unwrap();
                        *__mockall_guard
                            .deref_mut() = Rfunc::OnceSt(
                            ::mockall::Fragile::new(Box::new(__mockall_f)),
                        );
                    }
                    self
                }
                /// Supply a closure that will provide the return value for this
                /// `Expectation`.  The method's arguments are passed to the
                /// closure by value.
                pub fn returning<MockallF>(&mut self, __mockall_f: MockallF) -> &mut Self
                where
                    MockallF: FnMut() -> Box<dyn TestTrait> + Send + 'static,
                {
                    {
                        let mut __mockall_guard = self.rfunc.lock().unwrap();
                        *__mockall_guard.deref_mut() = Rfunc::Mut(Box::new(__mockall_f));
                    }
                    self
                }
                /// Single-threaded version of [`returning`](#method.returning).
                /// Can be used when the argument or return type isn't `Send`.
                ///
                /// It is a runtime error to call the mock method from a
                /// different thread than the one that originally called this
                /// method.
                pub fn returning_st<MockallF>(
                    &mut self,
                    __mockall_f: MockallF,
                ) -> &mut Self
                where
                    MockallF: FnMut() -> Box<dyn TestTrait> + 'static,
                {
                    {
                        let mut __mockall_guard = self.rfunc.lock().unwrap();
                        *__mockall_guard
                            .deref_mut() = Rfunc::MutSt(
                            ::mockall::Fragile::new(Box::new(__mockall_f)),
                        );
                    }
                    self
                }
                /// Add this expectation to a
                /// [`Sequence`](../../../mockall/struct.Sequence.html).
                pub fn in_sequence(
                    &mut self,
                    __mockall_seq: &mut ::mockall::Sequence,
                ) -> &mut Self {
                    self.common.in_sequence(__mockall_seq);
                    self
                }
                fn is_done(&self) -> bool {
                    self.common.is_done()
                }
                /// Validate this expectation's matcher.
                #[allow(clippy::ptr_arg)]
                fn matches(&self) -> bool {
                    self.common.matches()
                }
                /// Forbid this expectation from ever being called.
                pub fn never(&mut self) -> &mut Self {
                    self.common.never();
                    self
                }
                /// Create a new, default, [`Expectation`](struct.Expectation.html)
                pub fn new() -> Self {
                    Self::default()
                }
                /// Expect this expectation to be called exactly once.  Shortcut for
                /// [`times(1)`](#method.times).
                pub fn once(&mut self) -> &mut Self {
                    self.times(1)
                }
                /// Restrict the number of times that that this method may be called.
                ///
                /// The argument may be:
                /// * A fixed number: `.times(4)`
                /// * Various types of range:
                ///   - `.times(5..10)`
                ///   - `.times(..10)`
                ///   - `.times(5..)`
                ///   - `.times(5..=10)`
                ///   - `.times(..=10)`
                /// * The wildcard: `.times(..)`
                pub fn times<MockallR>(&mut self, __mockall_r: MockallR) -> &mut Self
                where
                    MockallR: Into<::mockall::TimesRange>,
                {
                    self.common.times(__mockall_r);
                    self
                }
                /// Set matching crieteria for this Expectation.
                ///
                /// The matching predicate can be anything implemening the
                /// [`Predicate`](../../../mockall/trait.Predicate.html) trait.  Only
                /// one matcher can be set per `Expectation` at a time.
                pub fn with(&mut self) -> &mut Self {
                    self.common.with();
                    self
                }
                /// Set a matching function for this Expectation.
                ///
                /// This is equivalent to calling [`with`](#method.with) with a
                /// function argument, like `with(predicate::function(f))`.
                pub fn withf<MockallF>(&mut self, __mockall_f: MockallF) -> &mut Self
                where
                    MockallF: Fn() -> bool + Send + 'static,
                {
                    self.common.withf(__mockall_f);
                    self
                }
                /// Single-threaded version of [`withf`](#method.withf).
                /// Can be used when the argument type isn't `Send`.
                pub fn withf_st<MockallF>(&mut self, __mockall_f: MockallF) -> &mut Self
                where
                    MockallF: Fn() -> bool + 'static,
                {
                    self.common.withf_st(__mockall_f);
                    self
                }
            }
            impl Default for Expectation {
                fn default() -> Self {
                    Expectation {
                        common: Common::default(),
                        rfunc: Mutex::new(Rfunc::default()),
                    }
                }
            }
            /// A collection of [`Expectation`](struct.Expectations.html)
            /// objects.  Users will rarely if ever use this struct directly.
            #[doc(hidden)]
            pub struct Expectations(Vec<Expectation>);
            impl Expectations {
                /// Verify that all current expectations are satisfied and clear
                /// them.
                pub fn checkpoint(&mut self) -> std::vec::Drain<Expectation> {
                    self.0.drain(..)
                }
                /// Create a new expectation for this method.
                pub fn expect(&mut self) -> &mut Expectation {
                    self.0.push(Expectation::default());
                    let __mockall_l = self.0.len();
                    &mut self.0[__mockall_l - 1]
                }
                pub fn new() -> Self {
                    Self::default()
                }
            }
            impl Default for Expectations {
                fn default() -> Self {
                    Expectations(Vec::new())
                }
            }
            impl Expectations {
                /// Simulate calling the real method.  Every current expectation
                /// will be checked in FIFO order and the first one with
                /// matching arguments will be used.
                pub fn call(&self) -> Option<Box<dyn TestTrait>> {
                    self.0
                        .iter()
                        .find(|__mockall_e| {
                            __mockall_e.matches()
                                && (!__mockall_e.is_done() || self.0.len() == 1)
                        })
                        .map(move |__mockall_e| __mockall_e.call())
                }
            }
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[allow(missing_docs)]
    struct MockTestTrait_TestTrait {
        test_method: __mock_MockTestTrait_TestTrait::__test_method::Expectations,
        test_method_2: __mock_MockTestTrait_TestTrait::__test_method_2::Expectations,
        clone_box: __mock_MockTestTrait_TestTrait::__clone_box::Expectations,
    }
    impl ::std::default::Default for MockTestTrait_TestTrait {
        fn default() -> Self {
            Self {
                test_method: Default::default(),
                test_method_2: Default::default(),
                clone_box: Default::default(),
            }
        }
    }
    impl MockTestTrait_TestTrait {
        /// Validate that all current expectations for all methods have
        /// been satisfied, and discard them.
        pub fn checkpoint(&mut self) {
            {
                self.test_method.checkpoint();
            }
            {
                self.test_method_2.checkpoint();
            }
            {
                self.clone_box.checkpoint();
            }
        }
    }
    impl MockTestTrait {
        /// Validate that all current expectations for all methods have
        /// been satisfied, and discard them.
        pub fn checkpoint(&mut self) {
            self.TestTrait_expectations.checkpoint();
        }
        /// Create a new mock object with no expectations.
        ///
        /// This method will not be generated if the real struct
        /// already has a `new` method.  However, it *will* be
        /// generated if the struct implements a trait with a `new`
        /// method.  The trait's `new` method can still be called
        /// like `<MockX as TraitY>::new`
        pub fn new() -> Self {
            Self::default()
        }
    }
    impl TestTrait for MockTestTrait {
        fn test_method(&self, arg: u32) -> u32 {
            let no_match_msg = {
                let res = ::alloc::fmt::format(
                    format_args!(
                        "{0}: No matching expectation found",
                        {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "MockTestTrait::test_method({0:?})",
                                    ::mockall::MaybeDebugger(&arg),
                                ),
                            );
                            res
                        },
                    ),
                );
                res
            };
            self.TestTrait_expectations.test_method.call(arg).expect(&no_match_msg)
        }
        fn test_method_2(&mut self, arg: u32) -> u32 {
            let no_match_msg = {
                let res = ::alloc::fmt::format(
                    format_args!(
                        "{0}: No matching expectation found",
                        {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "MockTestTrait::test_method_2({0:?})",
                                    ::mockall::MaybeDebugger(&arg),
                                ),
                            );
                            res
                        },
                    ),
                );
                res
            };
            self.TestTrait_expectations.test_method_2.call(arg).expect(&no_match_msg)
        }
        fn clone_box(&self) -> Box<dyn TestTrait> {
            let no_match_msg = {
                let res = ::alloc::fmt::format(
                    format_args!(
                        "{0}: No matching expectation found",
                        {
                            let res = ::alloc::fmt::format(
                                format_args!("MockTestTrait::clone_box()"),
                            );
                            res
                        },
                    ),
                );
                res
            };
            self.TestTrait_expectations.clone_box.call().expect(&no_match_msg)
        }
    }
    impl MockTestTrait {
        #[must_use = "Must set return value when not using the \"nightly\" feature"]
        ///Create an [`Expectation`](__mock_MockTestTrait_TestTrait/__test_method/struct.Expectation.html) for mocking the `test_method` method
        pub fn expect_test_method(
            &mut self,
        ) -> &mut __mock_MockTestTrait_TestTrait::__test_method::Expectation {
            self.TestTrait_expectations.test_method.expect()
        }
        #[must_use = "Must set return value when not using the \"nightly\" feature"]
        ///Create an [`Expectation`](__mock_MockTestTrait_TestTrait/__test_method_2/struct.Expectation.html) for mocking the `test_method_2` method
        pub fn expect_test_method_2(
            &mut self,
        ) -> &mut __mock_MockTestTrait_TestTrait::__test_method_2::Expectation {
            self.TestTrait_expectations.test_method_2.expect()
        }
        #[must_use = "Must set return value when not using the \"nightly\" feature"]
        ///Create an [`Expectation`](__mock_MockTestTrait_TestTrait/__clone_box/struct.Expectation.html) for mocking the `clone_box` method
        pub fn expect_clone_box(
            &mut self,
        ) -> &mut __mock_MockTestTrait_TestTrait::__clone_box::Expectation {
            self.TestTrait_expectations.clone_box.expect()
        }
    }
    pub struct MockTestTraitWrapper {
        inner: Arc<RwLock<MockTestTrait>>,
    }
    impl Clone for MockTestTraitWrapper {
        fn clone(&self) -> Self {
            Self { inner: self.inner.clone() }
        }
    }
    impl MockTestTraitWrapper {
        pub fn new() -> Self {
            Self {
                inner: Arc::new(RwLock::new(MockTestTrait::new())),
            }
        }
        pub fn set_expectations<F: FnOnce(&mut MockTestTrait)>(&mut self, g: F) {
            g(&mut Arc::get_mut(&mut self.inner).unwrap().write().unwrap())
        }
    }
    impl TestTrait for MockTestTraitWrapper {
        fn test_method(&self, arg: u32) -> u32 {
            self.inner.read().unwrap().test_method(arg)
        }
        fn test_method_2(&mut self, arg: u32) -> u32 {
            self.inner.write().unwrap().test_method_2(arg)
        }
        fn clone_box(&self) -> Box<dyn TestTrait> {
            Box::new(self.clone())
        }
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::test_basic"]
    pub const test_basic: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::test_basic"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/mod.rs",
            start_line: 10usize,
            start_col: 8usize,
            end_line: 10usize,
            end_col: 18usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(test_basic())),
    };
    fn test_basic() {}
}
#[rustc_main]
#[no_coverage]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&test_basic])
}
