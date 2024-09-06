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
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let name = args.name.as_str();
    let cppstd = args.cppstd.as_str();
    let cstd = args.cstd.as_str();
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
        ))?;
        create(fname, content.as_str())?;
    }

    {
        // makefile
        let fname = "Makefile";
        env.add_template(fname, include_str!("tpl/Makefile"))?;
        let tmpl = env.get_template(fname)?;

        let content = tmpl.render(context!(name => name))?;
        create(fname, content.as_str())?;
    }

    {
        // main.cpp
        let fname = "main.cpp";
        env.add_template(fname, include_str!("tpl/main.cpp"))?;
        let tmpl = env.get_template(fname)?;

        let content = tmpl.render(context!(name => "world"))?;
        create(fname, content.as_str())?;
    }

    Ok(())
}

fn create(name: &str, content: &str) -> anyhow::Result<()> {
    let mut file = std::fs::File::create(name)?;
    file.write_all(content.as_bytes())?;
    file.flush()?;
    Ok(())
}
