use clap::Parser;
use minijinja::context;
use std::io::Write;

#[derive(Parser)]
struct Args {
    /// project name
    #[arg(short, long)]
    name: String,

    /// c stanard
    #[arg(long, default_value = "11")]
    cstd: String,

    /// cpp stanard
    #[arg(long, default_value = "20")]
    cppstd: String,

    /// generate option for address sanitize
    #[arg(long, default_value_t = false)]
    asan: bool,

    /// generate option for thread sanitize
    #[arg(long, default_value_t = false)]
    tsan: bool,

    /// generate option for memory sanitize
    #[arg(long, default_value_t = false)]
    memory: bool,

    /// generate option for undefined sanitize
    #[arg(long, default_value_t = false)]
    undefined: bool,

    /// generate option for clang-tidy
    #[arg(long, default_value_t = false)]
    tidy: bool,

    /// generate option for split-dwarf
    #[arg(long, default_value_t = false)]
    dwarf: bool,

    /// force O3 in RelWithDebInfo mode
    #[arg(long, default_value_t = true)]
    fast: bool,

    /// c compiler
    #[arg(long, default_value = "/usr/bin/cc")]
    cc: String,

    /// c++ compiler
    #[arg(long, default_value = "/usr/bin/c++")]
    cpp: String,

    /// c++ linker
    #[arg(long, default_value = "LLD")]
    linker: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let name = args.name.as_str();
    let cppstd = args.cppstd.as_str();
    let cstd = args.cstd.as_str();
    let linker = args.linker.as_str();

    let cc = args.cc.as_str();
    let cpp = args.cpp.as_str();

    std::fs::create_dir(name)?;

    let dir = std::path::Path::new(name);
    std::env::set_current_dir(dir)?;
    let mut env = minijinja::Environment::new();

    {
        // cmakefile
        let fname = "CMakeLists.txt";
        env.add_template(fname, include_str!("tpl/CMakeLists.txt"))?;
        let tmpl = env.get_template(fname)?;

        let content = tmpl.render(context!(
            name => name,
            cstd => cstd,
            cppstd => cppstd,
            asan => args.asan,
            tsan => args.tsan,
            memory => args.memory,
            undefined => args.undefined,
            tidy => args.tidy,
            dwarf => args.dwarf,
            fast => args.fast,
            linker => linker,
        ))?;
        create(fname, content.as_str())?;
    }

    {
        // makefile
        let fname = "Makefile";
        env.add_template(fname, include_str!("tpl/Makefile"))?;
        let tmpl = env.get_template(fname)?;

        let content = tmpl.render(context!(
            name => name,
            cc => cc,
            cpp => cpp,
        ))?;
        create(fname, content.as_str())?;
    }

    {
        // main.cpp
        let fname = "main.cpp";
        env.add_template(fname, include_str!("tpl/main.cpp"))?;
        create_without_arg(fname, &env)?;
    }

    {
        // clang-format
        let fname = ".clang-format";
        env.add_template(fname, include_str!("tpl/.clang-format"))?;
        create_without_arg(fname, &env)?;
    }

    {
        // clang-tidy
        let fname = ".clang-tidy";
        env.add_template(fname, include_str!("tpl/.clang-tidy"))?;
        create_without_arg(fname, &env)?;
    }

    Ok(())
}

fn create(name: &str, content: &str) -> anyhow::Result<()> {
    let mut file = std::fs::File::create(name)?;
    file.write_all(content.as_bytes())?;
    file.flush()?;
    Ok(())
}

fn create_without_arg(name: &str, env: &minijinja::Environment) -> anyhow::Result<()> {
    let tmpl = env.get_template(name)?;
    let content = tmpl.render(context!())?;
    create(name, &content)?;
    Ok(())
}
