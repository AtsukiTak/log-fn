use std::convert::TryFrom;

pub enum Attribute {
    Type(TypeAttr),
    Level(LevelAttr),
    Msg(MsgAttr),
    If(IfAttr),
}

pub enum TypeAttr {
    Pre,
    Post,
}

pub enum LevelAttr {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

pub struct MsgAttr {
    pub msg: String,
}

pub struct IfAttr {
    pub path: String,
}

macro_rules! unrecognized_item {
    ($item:expr) => {
        return Err(syn::Error::new_spanned(
            $item,
            format!("Unrecognized attribute."),
        ));
    };
}

impl TryFrom<syn::NestedMeta> for Attribute {
    type Error = syn::Error;

    fn try_from(attr: syn::NestedMeta) -> Result<Self, Self::Error> {
        let meta = match attr {
            syn::NestedMeta::Meta(meta) => meta,
            _ => unrecognized_item!(attr),
        };

        match meta.clone() {
            syn::Meta::Path(path) => {
                if path.is_ident("pre") {
                    Ok(Attribute::Type(TypeAttr::Pre))
                } else if path.is_ident("post") {
                    Ok(Attribute::Type(TypeAttr::Post))
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
                        "trace" => LevelAttr::Trace,
                        "debug" => LevelAttr::Debug,
                        "info" => LevelAttr::Info,
                        "warn" => LevelAttr::Warn,
                        "error" => LevelAttr::Error,
                        _ => unrecognized_item!(s),
                    };
                    Ok(Attribute::Level(level))
                } else if path.is_ident("msg") {
                    Ok(Attribute::Msg(MsgAttr { msg: s.value() }))
                } else if path.is_ident("if") {
                    Ok(Attribute::If(IfAttr { path: s.value() }))
                } else {
                    unrecognized_item!(meta)
                }
            }
            _ => unrecognized_item!(meta),
        }
    }
}
