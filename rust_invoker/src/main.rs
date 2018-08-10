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

extern crate function_types;
extern crate libloading as lib;

use std::env;
use function_types::Basic;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Expected a single argument of the path to the function library");
        std::process::exit(-1);
    }

    let lib = lib::Library::new(&args[1]).unwrap();
    unsafe {
        let func: lib::Symbol<Basic> = lib.get(b"function").unwrap();
        println!("{}", func(&"riff".to_string()));
    }
}
