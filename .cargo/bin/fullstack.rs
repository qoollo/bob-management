mod backend;
mod frontend;
mod net;
mod tsync;

pub fn main() {
    if !net::is_port_free(21012) {
        println!("========================================================");
        println!(" ViteJS (the frontend compiler/bundler) needs to run on");
        println!(" port 21012 but it seems to be in use.");
        println!("========================================================");
        panic!("Port 21012 is taken but is required for development!")
    }

    let project_dir = env!("CARGO_MANIFEST_DIR");

    tsync::main();

    // create_rust_app::dev::run_server(project_dir);

    println!("..................................");
    println!(".. Starting development server ...");
    println!("..................................");
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async move {
        tokio::spawn(async move {
            frontend::main();
        });
        tokio::spawn(async move {
            backend::main();
        });
    })
}
