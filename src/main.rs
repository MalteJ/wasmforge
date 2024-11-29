use log::{debug, info, warn, LevelFilter};
use simple_logger::SimpleLogger;
use wasmer::{Instance, Function, Module, Store, imports, Value, Memory};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new().env().with_level(LevelFilter::Debug).init().unwrap();
    info!("WasmForge is starting...");

    debug!("Reading wasm code...");
    let wasm_bytes = std::fs::read("./examples/helloworld/build/debug.wasm")?;

    debug!("creating store...");
    let mut store = Store::default();

    debug!("creating module...");
    let module = Module::new(&store, &wasm_bytes)?;

    debug!("defining imports..."); // Define a mutable variable to store the WASM memory

    fn say_hello_world(a: i32) {
        warn!("Hello, world! {}", a)
    }

    // We then create an import object so that the `Module`'s imports can be satisfied.
    let import_object = imports! {
        // We use the default namespace "env".
        "env" => {
            // And call our function "say_hello".
            "say_hello" => Function::new_typed(&mut store, say_hello_world),
        }
    };

    debug!("creating instance...");
    let instance = Instance::new(&mut store, &module, &import_object)?;

    debug!("getting exports....");
    let main_function = instance.exports.get_function("add_one")?;

    info!("Executing main...");
    let result = main_function.call(&mut store, &[Value::I32(41)])?;

    info!("The answer is {}", result[0]);

    Ok(())
}
