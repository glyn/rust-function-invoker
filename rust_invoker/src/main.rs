extern crate futures;
extern crate hyper;
extern crate function_types;
extern crate libloading as lib;

use std::env;
use std::str;
use function_types::Basic;
use hyper::rt::{Future, Stream};
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server};

/// We need to return different futures depending on the route matched,
/// and we can do that with an enum, such as `futures::Either`, or with
/// trait objects.
///
/// A boxed Future (trait object) is used as it is easier to understand
/// and extend with more types. Advanced users could switch to `Either`.
type BoxFut = Box<Future<Item=Response<Body>, Error=hyper::Error> + Send>;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Expected a single argument of the path to the function library");
        std::process::exit(-1);
    }

    let l = lib::Library::new(&args[1]).unwrap();
    f(&l);
}

fn f<'a>(l: &'a lib::Library) {
    let func: lib::Symbol<Basic>;
    unsafe {
        func = l.get(b"function").unwrap();
    }

    let addr = ([127, 0, 0, 1], 8080).into();

    let echo = |req: Request<Body>| -> BoxFut {
        let mut response = Response::new(Body::empty());

        let resp = req.into_body().concat2().map(|chunk| {
            let body = chunk.iter().cloned().collect::<Vec<u8>>();

            let resp;
            unsafe {
                resp = func(&str::from_utf8(&body).unwrap().to_owned())
            }

            *response.body_mut() = Body::from(resp);
            response
        });

        return Box::new(resp);
    };

    let server = Server::bind(&addr)
        .serve(move || service_fn(echo))
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);
    hyper::rt::run(server);
}