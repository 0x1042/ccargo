use clap::Parser;

#[derive(Parser)]
pub(crate) struct Opt {
    /// project name
    #[arg(short, long)]
    pub name: String,

    /// c stanard
    #[arg(long, default_value = "11")]
    pub cstd: String,

    /// cpp stanard
    #[arg(long, default_value = "23")]
    pub cppstd: String,

    /// force O3 in RelWithDebInfo mode
    #[arg(long, default_value_t = true)]
    pub fast: bool,

    /// c compiler
    #[arg(long, default_value = "/usr/bin/cc")]
    pub cc: String,

    /// c++ compiler
    #[arg(long, default_value = "/usr/bin/c++")]
    pub cpp: String,

    /// c++ linker
    #[arg(long, default_value = "LLD")]
    pub linker: String,

    /// use bazel
    #[arg(long, default_value_t = false)]
    pub bazel: bool,
}
