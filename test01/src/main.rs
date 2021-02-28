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
    // return: 142
    
    //match res {
    //     _ => {println!("ok");
    //           print_type_of(&res);
    //     },
    //}
    
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
