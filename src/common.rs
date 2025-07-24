use std::io::Write;

use minijinja::context;

pub fn create(name: &str, content: &str) -> anyhow::Result<()> {
    let mut file = std::fs::File::create(name)?;
    file.write_all(content.as_bytes())?;
    file.flush()?;
    Ok(())
}

pub fn create_without_arg(name: &str, env: &minijinja::Environment) -> anyhow::Result<()> {
    let tmpl = env.get_template(name)?;
    let content = tmpl.render(context!())?;
    create(name, &content)?;
    Ok(())
}

pub fn basic(env: &mut minijinja::Environment, enablecc: bool) -> anyhow::Result<()> {
    if enablecc {
        // main.cpp
        let fname = "main.cc";
        env.add_template(fname, include_str!("tpl/main.cc"))?;
        create_without_arg(fname, env)?;
    }

    {
        // clang-format
        let fname = ".clang-format";
        env.add_template(fname, include_str!("tpl/.clang-format"))?;
        create_without_arg(fname, env)?;
    }

    {
        // clang-tidy
        let fname = ".clang-tidy";
        env.add_template(fname, include_str!("tpl/.clang-tidy"))?;
        create_without_arg(fname, env)?;
    }

    Ok(())
}
