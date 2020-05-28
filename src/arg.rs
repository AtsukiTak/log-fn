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
    pub path: String,
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
    let meta = match arg {
        syn::NestedMeta::Meta(meta) => meta,
        _ => unrecognized_item!(arg),
    };

    match meta.clone() {
        syn::Meta::Path(path) => {
            if path.is_ident("pre") {
                Ok(Arg::Type(TypeArg::Pre))
            } else if path.is_ident("post") {
                Ok(Arg::Type(TypeArg::Post))
            } else {
                unrecognized_item!(path)
            }
        }
        syn::Meta::NameValue(syn::MetaNameValue {
            path,
            lit: syn::Lit::Str(s),
            ..
        }) => {
            if path.is_ident("level") {
                let level = match s.value().as_str() {
                    "trace" => LevelArg::Trace,
                    "debug" => LevelArg::Debug,
                    "info" => LevelArg::Info,
                    "warn" => LevelArg::Warn,
                    "error" => LevelArg::Error,
                    _ => unrecognized_item!(s),
                };
                Ok(Arg::Level(level))
            } else if path.is_ident("msg") {
                Ok(Arg::Msg(MsgArg { msg: s.value() }))
            } else if path.is_ident("if") {
                Ok(Arg::If(IfArg { path: s.value() }))
            } else {
                unrecognized_item!(meta)
            }
        }
        _ => unrecognized_item!(meta),
    }
}
