// The entry file of your WebAssembly module.
@external("env", "say_hello")
declare function say_hello(a: i32): void;
export function add_one(a: i32): i32 {
    say_hello(5);
    //println("Hello from AssemblyScript!");
    return a + 1;
}