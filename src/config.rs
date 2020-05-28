use crate::arg::{Arg, IfArg, LevelArg, MsgArg, TypeArg};
use proc_macro2::Span;
use syn::Error;

#[derive(Debug)]
pub struct Config {
    pub typ: TypeArg,
    pub msg: MsgArg,
    pub level: LevelArg,
    pub cond: Option<IfArg>,
}

pub fn from_args(args: Vec<Arg>) -> syn::Result<Config> {
    let typ = args
        .iter()
        .find_map(|arg| match arg {
            Arg::Type(t) => Some(t.clone()),
            _ => None,
        })
        .ok_or_else(|| Error::new(Span::call_site(), "\"pre\" or \"post\" arg is required"))?;

    let msg = args
        .iter()
        .find_map(|arg| match arg {
            Arg::Msg(m) => Some(m.clone()),
            _ => None,
        })
        .ok_or_else(|| Error::new(Span::call_site(), "\"msg\" arg is required"))?;

    let level = args
        .iter()
        .find_map(|arg| match arg {
            Arg::Level(l) => Some(l.clone()),
            _ => None,
        })
        .ok_or_else(|| Error::new(Span::call_site(), "\"level\" arg is required"))?;

    let cond = args.iter().find_map(|arg| match arg {
        Arg::If(i) => Some(i.clone()),
        _ => None,
    });

    Ok(Config {
        typ,
        msg,
        level,
        cond,
    })
}
