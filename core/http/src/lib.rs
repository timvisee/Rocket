#![feature(specialization)]
#![feature(proc_macro_hygiene)]
#![feature(const_fn)]
#![feature(try_from)]
#![feature(crate_visibility_modifier)]
#![recursion_limit="256"]

//! Types that map to concepts in HTTP.
//!
//! This module exports types that map to HTTP concepts or to the underlying
//! HTTP library when needed. Because the underlying HTTP library is likely to
//! change (see <a
//! href="https://github.com/SergioBenitez/Rocket/issues/17">#17</a>), types in
//! [hyper](hyper/index.html) should be considered unstable.

#[macro_use]
extern crate pear;
extern crate smallvec;
#[macro_use]
extern crate percent_encoding;
extern crate cookie;
extern crate time;
extern crate indexmap;
extern crate state;

pub mod hyper;
pub mod uri;
pub mod ext;

#[doc(hidden)]
#[cfg(feature = "tls")]
pub mod tls;

#[macro_use]
mod docify;
#[macro_use]
mod known_media_types;
mod cookies;
mod method;
mod media_type;
mod content_type;
mod status;
mod header;
mod accept;
mod raw_str;

crate mod parse;

pub mod uncased;

// We need to export these for codegen, but otherwise it's unnecessary.
// TODO: Expose a `const fn` from ContentType when possible. (see RFC#1817)
// FIXME(rustc): These show up in the rexported module.
#[doc(hidden)] pub use self::parse::Indexed;
#[doc(hidden)] pub use self::media_type::{MediaParams, Source};

// This one we need to expose for core.
#[doc(hidden)] pub use self::cookies::{Key, CookieJar};

pub use self::method::Method;
pub use self::content_type::ContentType;
pub use self::accept::{Accept, QMediaType};
pub use self::status::{Status, StatusClass};
pub use self::header::{Header, HeaderMap};
pub use self::raw_str::RawStr;

pub use self::media_type::MediaType;
pub use self::cookies::{Cookie, SameSite, Cookies};
