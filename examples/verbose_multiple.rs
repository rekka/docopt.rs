extern crate serialize;
extern crate docopt;

use docopt::Docopt;

// This shows how to implement multiple levels of verbosity.
//
// When you have multiple patterns, I think the only way to carry the
// repeated flag through all of them is to specify it for each pattern
// explicitly.
//
// This is unfortunate.
static USAGE: &'static str = "
Usage: cp [options] [-v | -vv | -vvv] <source> <dest>
       cp [options] [-v | -vv | -vvv] <source>... <dir>

Options:
    -a, --archive  Copy everything.
    -v, --verbose  Show extra log output.
";

#[deriving(Decodable, Show)]
struct Args {
    arg_source: Vec<String>,
    arg_dest: String,
    arg_dir: String,
    flag_archive: bool,
    flag_verbose: uint,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    println!("{}", args);
}
