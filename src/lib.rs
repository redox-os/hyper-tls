//! # hyper-tls
//!
//! An HTTPS connector to be used with [hyper][].
//!
//! [hyper]: https://hyper.rs
//!
//! ## Example
//!
//! ```no_run
//! extern crate futures;
//! extern crate hyper;
//! extern crate hyper_tls;
//! extern crate tokio;
//!
//! use futures::{future, Future};
//! use hyper_tls::HttpsConnector;
//! use hyper::Client;
//!
//! fn main() {
//!     tokio::run(future::lazy(|| {
//!         // 4 is number of blocking DNS threads
//!         let https = HttpsConnector::new(4).unwrap();
//!         let client = Client::builder()
//!             .build::<_, hyper::Body>(https);
//!
//!         client
//!             .get("https://hyper.rs".parse().unwrap())
//!             .map(|res| {
//!                 assert_eq!(res.status(), 200);
//!             })
//!             .map_err(|e| println!("request error: {}", e))
//!     }));
//! }
//! ```
#![deny(warnings)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

extern crate bytes;
extern crate futures;
extern crate hyper;
extern crate native_tls;
extern crate tokio_io;
extern crate tokio_tls;

pub use client::{HttpsConnector, HttpsConnecting, Error};
pub use stream::MaybeHttpsStream;

mod client;
mod stream;
