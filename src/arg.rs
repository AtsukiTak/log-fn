use syn::{Lit, Meta, MetaNameValue, NestedMeta};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Arg {
    Type(TypeArg),
    Level(LevelArg),
    Msg(MsgArg),
    If(IfArg),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeArg {
    Pre,
    Post,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LevelArg {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MsgArg {
    pub msg: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfArg {
    pub path: syn::ExprPath,
}

impl LevelArg {
    pub fn ident(&self) -> syn::Ident {
        let span = proc_macro2::Span::call_site();
        match self {
            LevelArg::Trace => syn::Ident::new("Trace", span),
            LevelArg::Debug => syn::Ident::new("Debug", span),
            LevelArg::Info => syn::Ident::new("Info", span),
            LevelArg::Warn => syn::Ident::new("Warn", span),
            LevelArg::Error => syn::Ident::new("Error", span),
        }
    }
}

macro_rules! unrecognized_item {
    ($item:expr) => {
        return Err(syn::Error::new_spanned($item, "Unrecognized attribute."));
    };
}

pub fn from_input_vec(args: syn::AttributeArgs) -> syn::Result<Vec<Arg>> {
    args.into_iter().map(from_input).collect()
}

pub fn from_input(arg: syn::NestedMeta) -> syn::Result<Arg> {
    match arg {
        NestedMeta::Meta(Meta::Path(path)) => {
            let ident = match path.get_ident() {
                Some(i) => i,
                None => unrecognized_item!(path),
            };
            match ident {
                i if i == "Pre" => Ok(Arg::Type(TypeArg::Pre)),
                i if i == "Post" => Ok(Arg::Type(TypeArg::Post)),
                i if i == "Trace" => Ok(Arg::Level(LevelArg::Trace)),
                i if i == "Debug" => Ok(Arg::Level(LevelArg::Debug)),
                i if i == "Info" => Ok(Arg::Level(LevelArg::Info)),
                i if i == "Warn" => Ok(Arg::Level(LevelArg::Warn)),
                i if i == "Error" => Ok(Arg::Level(LevelArg::Error)),
                i => unrecognized_item!(i),
            }
        }
        NestedMeta::Meta(Meta::NameValue(MetaNameValue {
            path,
            lit: syn::Lit::Str(s),
            ..
        })) => {
            if path.is_ident("if") {
                Ok(Arg::If(IfArg {
                    path: syn::parse_str(&s.value())?,
                }))
            } else {
                unrecognized_item!(path)
            }
        }
        NestedMeta::Lit(Lit::Str(s)) => Ok(Arg::Msg(MsgArg { msg: s.value() })),
        _ => unrecognized_item!(arg),
    }
}
