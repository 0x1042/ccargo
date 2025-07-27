use minijinja::context;

use crate::{args::Opt, common};

pub fn use_bazel(opt: &Opt) -> anyhow::Result<()> {
    let name = opt.name.as_str();

    std::fs::create_dir(name)?;

    let dir = std::path::Path::new(name);
    std::env::set_current_dir(dir)?;
    let mut env = minijinja::Environment::new();
    common::basic(&mut env, false)?;

    {
        // .bazelrc
        let fname = ".bazelrc";
        env.add_template(fname, include_str!("tpl/.bazelrc"))?;
        common::create_without_arg(fname, &env)?;
    }

    {
        // .bazelrc
        let fname = "BUILD.bazel";
        env.add_template(fname, include_str!("tpl/BUILD2.bazel"))?;
        common::create_without_arg(fname, &env)?;
    }

    {
        let fname = "MODULE.bazel";
        env.add_template(fname, include_str!("tpl/MODULE.bazel"))?;
        let tmpl = env.get_template(fname)?;

        let content = tmpl.render(context!(
            name => name,
        ))?;
        common::create(fname, content.as_str())?;
    }

    std::fs::create_dir("app")?;

    {
        let fname = "app/copts.bzl";
        env.add_template(fname, include_str!("tpl/copts.bzl"))?;
        common::create_without_arg(fname, &env)?;
    }

    {
        let fname = "app/BUILD.bazel";
        env.add_template(fname, include_str!("tpl/BUILD.bazel"))?;
        let tmpl = env.get_template(fname)?;

        let content = tmpl.render(context!(
            name => name,
        ))?;
        common::create(fname, content.as_str())?;
    }

    {
        let fname = "app/main.cc";
        env.add_template(fname, include_str!("tpl/main.cc"))?;
        common::create_without_arg(fname, &env)?;
    }

    Ok(())
}
