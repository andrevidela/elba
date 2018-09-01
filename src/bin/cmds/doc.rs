use super::{args, get};
use clap::{App, ArgMatches, SubCommand};
use elba::{
    cli::build,
    util::{config::Config, errors::Res},
};
use failure::ResultExt;
use std::env::current_dir;

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("doc")
        .about("Builds the docs for the root package")
        .arg(args::build_threads())
        .arg(args::debug_log())
        .arg(args::offline())
        .arg(args::idris_opts())
}

pub fn exec(c: &mut Config, args: &ArgMatches) -> Res<String> {
    let project = current_dir().context(format_err!(
        "couldn't get current dir; doesn't exist or no permissions..."
    ))?;

    let logger = get::logger(c, args);

    let ctx = build::BuildCtx {
        indices: c.indices.to_owned(),
        global_cache: c.layout(),
        logger,
        threads: get::threads(c, args),
        shell: c.shell(),
        offline: args.is_present("offline"),
        opts: get::idris_opts(c, args),
    };

    build::doc(&ctx, &project)
}
