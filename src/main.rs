extern crate lapp;

fn main() {
    let args = lapp::parse_args("
    Watches for any file that matches the provided glob
    and moves it to the requested destination.
    -g, --glob (default '*') watch for files that match this glob
    -d, --dest (string) destination for matching files
    ");
    let glob = args.get_string("glob");
    let dest = args.get_string("dest");

    println!("g:{}, d:{}", glob, dest);
}
