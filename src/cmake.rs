use minijinja::context;

use crate::{args::Opt, common};

pub fn use_cmake(opt: &Opt) -> anyhow::Result<()> {
    let name = opt.name.as_str();
    let cppstd = opt.cppstd.as_str();
    let cstd = opt.cstd.as_str();
    let linker = opt.linker.as_str();

    let cc = opt.cc.as_str();
    let cpp = opt.cpp.as_str();

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
            fast => opt.fast,
            linker => linker,
        ))?;
        common::create(fname, content.as_str())?;
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
        common::create(fname, content.as_str())?;
    }

    common::basic(&mut env, true)?;

    std::fs::create_dir("cmake")?;

    {
        let fname = "cmake/clang-tidy.cmake";
        env.add_template(fname, include_str!("tpl/cmake/clang-tidy.cmake"))?;
        common::create_without_arg(fname, &env)?;
    }

    {
        let fname = "cmake/sanitizer.cmake";
        env.add_template(fname, include_str!("tpl/cmake/sanitizer.cmake"))?;
        common::create_without_arg(fname, &env)?;
    }

    {
        let fname = "cmake/dwarf.cmake";
        env.add_template(fname, include_str!("tpl/cmake/dwarf.cmake"))?;
        common::create_without_arg(fname, &env)?;
    }

    Ok(())
}
