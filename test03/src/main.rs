extern crate actix_lua;
extern crate actix;
extern crate futures;
use actix::*;
use actix_lua::{LuaActorBuilder, LuaMessage};

use std::fs::File;
use std::io::prelude::*;


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn read_to_string(filename: &str) -> String {
    let mut f = File::open(filename).expect("File not found");
    let mut body = String::new();
    f.read_to_string(&mut body).expect("Failed to read file");

    body
}

/*  actix-lua /actix 0.9.0
 *
 * https://github.com/geofmureithi/actix-lua
 * 
 */

fn main () {

    let mut system = actix::System::new("actix-lua-example");

    system.block_on(async { 
                  let script = read_to_string("./src/test.lua");
                  let addr = LuaActorBuilder::new()
                       .on_handle_with_lua(&script)
                       .build()
                       .unwrap()
                       .start();
    
                   let res = addr.send(LuaMessage::from(100)).await;
                   print_type_of(&res);
                   match res.unwrap() {
                           LuaMessage::String(s) => println!("String:{}",s),
                           LuaMessage::Integer(s) => println!("Integer:{}",s),
                           LuaMessage::Number(s) => println!("Number:{}",s),
                           LuaMessage::Boolean(s) => println!("Boolean:{}",s),
                           // ignore everything else
                           _ => println!("unknown"),

                   }
    
    });

    let _result = system.run();

}

