use clap::Parser;

mod args;
mod bazel;
mod cmake;
mod common;

fn main() -> anyhow::Result<()> {
    let args = args::Opt::parse();
    if args.bazel {
        bazel::use_bazel(&args)
    } else {
        cmake::use_cmake(&args)
    }
}
