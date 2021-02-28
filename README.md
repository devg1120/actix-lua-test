# test01  actix-lua / actix 0.7 tokio 0.1

[actix-lua](https://github.com/poga/actix-lua)

```
extern crate actix_lua;
extern crate actix;
extern crate futures;
use futures::{future, Future};
use actix::*;
use actix_lua::{LuaActorBuilder, LuaMessage};

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main () {

    let sys = System::new("test");

    let addr = LuaActorBuilder::new()
        .on_handle_with_lua(r#"return ctx.msg + 42"#)
        .build()
        .unwrap()
        .start();

    let res = addr.send(LuaMessage::from(100));
    
    print_type_of(&res);

    Arbiter::spawn(res.then(|res| {
        match res {
            Ok(result) => println!("SUM: {:?}", result),
            _ => println!("Something wrong"),
        }
        
        System::current().stop();
        future::result(Ok(()))
    }));

    sys.run();

}

```
---
# test02  actix-lua2 / actic 0.9 tokio 0.2

[actix-lua2](https://github.com/devg1120/actix-lua2)

```
extern crate actix_lua;
extern crate actix;
extern crate futures;
use actix::*;
use actix_lua::{LuaActorBuilder, LuaMessage};

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


#[actix_rt::main] 
async fn main () {

    let addr = LuaActorBuilder::new()
        .on_handle_with_lua(r#"return ctx.msg + 42"#)
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


}

```
---
# test03  actix-lua2 / actic 0.9 tokio 0.2

[actix-lua2](https://github.com/devg1120/actix-lua2)

```
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

```

```
-- test.lua

return ctx.msg + 45

```
