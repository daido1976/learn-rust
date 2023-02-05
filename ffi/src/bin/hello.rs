// $ gcc -fPIC -c hello.c -o ./target/libhello.a
#[link(name = "hello")]
extern "C" {
    fn hello_world();
}

fn main() {
    unsafe {
        hello_world();
    }
}
