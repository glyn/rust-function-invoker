/*
 * Copyright 2018 The original author or authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//extern crate futures_core;
//extern crate function_types;
//extern crate hyper;
//extern crate futures;
//extern crate libloading as lib;
//
//use function_types::Basic;
//use hyper::rt::Future;
//use hyper::service::service_fn_ok;
//use hyper::{Body, Request, Response, Server};
//use std::env;
//use std::str;
//use futures::prelude::*;
//use futures_core::future;
//use std::io;
//use std::io::prelude::*;
//use std::fs::File;
//
//fn main() {
//    let args: Vec<String> = env::args().collect();
//    if args.len() != 2 {
//        println!("Expected a single argument of the path to the function library");
//        std::process::exit(-1);
//    }
//
//    let lib = lib::Library::new(&args[1]).unwrap();
//    let func: lib::Symbol<Basic>;
//    unsafe {
//        func = lib.get(b"function").unwrap();
//    }
//
//    const PHRASE: &str = "Hello, World!";
//
////    let hello_world = move |_req: Request<Body>| -> Response<Body> {
////        Response::new(Body::from(func(_req.body().concat2().and_then(|body|{
////            let body = str::from_utf8(&body).unwrap();
////            future::ok(body)
////        }))))
////    };
//    let hello_world = move |_req: Request<String>| -> Response<Body> {
//        Response::new(Body::from(func(& {
//            let mut buf = String::new();
//            match _req.read_to_string(&mut buf) {
//                Ok(_) => (),
//                Err(_) => panic!("I give up."),
//            };
//            buf
//        })))
//    };
//
//    // This is our socket address...
//    let addr = ([127, 0, 0, 1], 8080).into();
//
//    // A `Service` is needed for every connection, so this
//    // creates on of our `hello_world` function.
//    let new_svc = || {
//        // service_fn_ok converts our function into a `Service`
//        service_fn_ok(hello_world)
//    };
//
//    let server = Server::bind(&addr)
//        .serve(new_svc)
//        .map_err(|e| eprintln!("server error: {}", e));
//
//    // Run this server for... forever!
//    hyper::rt::run(server);
//}
