use super::{args, get};
use clap::{App, Arg, ArgMatches, SubCommand};
use elba::{
    cli::build,
    package::Spec,
    util::{config::Config, errors::Res},
};
use failure::ResultExt;
use itertools::Either::{Left, Right};
use std::{env::current_dir, str::FromStr};

// TODO: Don't use specs; use --index and --ver ?

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("install")
        .about("Installs a package's artifacts")
        .arg(Arg::with_name("spec"))
        .arg(args::build_threads())
        .arg(args::target_bin())
        .arg(args::debug_log())
        .arg(args::offline())
        .arg(args::idris_opts())
        .arg(
            Arg::with_name("force")
                .long("force")
                .help("Overwrite existing installed binaries"),
        )
}

pub fn exec(c: &mut Config, args: &ArgMatches) -> Res<String> {
    let current = current_dir();

    let proj = if let Some(spec) = args.value_of_lossy("spec") {
        let spec = &*spec;
        let spec = Spec::from_str(spec)
            .with_context(|e| format_err!("the spec `{}` is invalid:\n{}", spec, e))?;
        Left(spec)
    } else if let Ok(d) = current {
        Right(d)
    } else {
        bail!("no package was specified to be installed and the current directory is inaccessible")
    };

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

    let targets = args
        .values_of("bin")
        .map(|x| x.collect())
        .unwrap_or_else(|| vec![]);

    let backend = get::backends(c, args);

    build::install(&ctx, proj, &targets, &backend, args.is_present("force"))
}
