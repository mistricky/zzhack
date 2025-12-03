use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, spanned::Spanned, Attribute, Data, DataEnum, DataStruct, DeriveInput,
    Fields, Ident, Type,
};

#[proc_macro_derive(Parser, attributes(command, arg))]
pub fn derive_parser(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand(&input) {
        Ok(tokens) => {
            // Debugging output can be enabled by setting MICRO_CLI_DEBUG_DERIVE.
            if std::env::var("MICRO_CLI_DEBUG_DERIVE").is_ok() {
                println!("{}", tokens);
            }
            tokens.into()
        }
        Err(err) => err.to_compile_error().into(),
    }
}

#[derive(Default, Clone)]
struct CommandMeta {
    name: Option<syn::LitStr>,
    about: Option<syn::LitStr>,
}

#[derive(Default, Clone)]
struct ArgMeta {
    short: Option<char>,
    long: Option<String>,
    help: Option<String>,
    default_value: Option<syn::Expr>,
    is_subcommand: bool,
    positional: bool,
}

fn expand(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    match &input.data {
        Data::Struct(data) => {
            let meta = parse_command_meta(&input.attrs)?;
            expand_struct(&input.ident, data, &meta)
        }
        Data::Enum(data) => {
            let meta = parse_command_meta(&input.attrs)?;
            expand_enum(&input.ident, data, &meta)
        }
        _ => Err(syn::Error::new(
            input.span(),
            "Parser can only be derived for structs or enums",
        )),
    }
}

fn parse_command_meta(attrs: &[Attribute]) -> syn::Result<CommandMeta> {
    let mut meta = CommandMeta::default();
    for attr in attrs {
        if !attr.path().is_ident("command") {
            continue;
        }
        attr.parse_nested_meta(|nested| {
            if nested.path.is_ident("about") {
                let lit: syn::LitStr = nested.value()?.parse()?;
                meta.about = Some(lit);
            } else if nested.path.is_ident("name") {
                let lit: syn::LitStr = nested.value()?.parse()?;
                meta.name = Some(lit);
            } else if nested.path.is_ident("version") {
                return Err(syn::Error::new(
                    nested.path.span(),
                    "`version` is not supported; micro_cli CLIs do not expose version flags",
                ));
            }
            Ok(())
        })?;
    }
    Ok(meta)
}

fn parse_arg_meta(attrs: &[Attribute]) -> syn::Result<ArgMeta> {
    let mut meta = ArgMeta::default();
    for attr in attrs {
        if !(attr.path().is_ident("arg") || attr.path().is_ident("command")) {
            continue;
        }
        attr.parse_nested_meta(|nested| {
            if nested.path.is_ident("short") {
                if let Ok(lit) = nested.value() {
                    let ch: syn::LitChar = lit.parse()?;
                    meta.short = Some(ch.value());
                } else if let Some(id) = nested.path.get_ident() {
                    meta.short = id.to_string().chars().next();
                }
            } else if nested.path.is_ident("long") {
                if let Ok(lit) = nested.value() {
                    let s: syn::LitStr = lit.parse()?;
                    meta.long = Some(s.value());
                } else if let Some(id) = nested.path.get_ident() {
                    meta.long = Some(id.to_string());
                }
            } else if nested.path.is_ident("default_value_t") {
                let expr: syn::Expr = nested.value()?.parse()?;
                meta.default_value = Some(expr);
            } else if nested.path.is_ident("help") {
                let s: syn::LitStr = nested.value()?.parse()?;
                meta.help = Some(s.value());
            } else if nested.path.is_ident("subcommand") {
                meta.is_subcommand = true;
            } else if nested.path.is_ident("positional") {
                meta.positional = true;
            }
            Ok(())
        })?;
    }
    Ok(meta)
}

fn is_vec_string(ty: &Type) -> bool {
    if let Type::Path(p) = ty {
        let mut segments = p.path.segments.iter();
        if let Some(seg) = segments.next() {
            if seg.ident == "Vec" {
                if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                    if let Some(syn::GenericArgument::Type(Type::Path(inner))) = args.args.first() {
                        return inner.path.is_ident("String");
                    }
                }
            }
        }
    }
    false
}

fn expand_struct(
    ident: &Ident,
    data: &DataStruct,
    meta: &CommandMeta,
) -> syn::Result<proc_macro2::TokenStream> {
    let mut declarations = Vec::new();
    let mut match_arms = Vec::new();
    let mut post_process = Vec::new();
    let mut field_inits = Vec::new();
    let mut option_help_lines = Vec::new();
    let mut positional_help_lines = Vec::new();
    let mut positional_names: Vec<syn::LitStr> = Vec::new();
    let name_lit = meta
        .name
        .clone()
        .unwrap_or_else(|| syn::LitStr::new(&ident.to_string(), ident.span()));
    let about_lit = meta
        .about
        .clone()
        .unwrap_or_else(|| syn::LitStr::new(&format!("{} options", ident), ident.span()));
    let mut subcommand_field: Option<Ident> = None;
    let mut needs_positionals_iter = false;
    let mut has_required_fields = false;

    for field in data.fields.iter() {
        let fname = field
            .ident
            .clone()
            .ok_or_else(|| syn::Error::new(field.span(), "Unnamed fields unsupported"))?;
        let ty = field.ty.clone();
        let arg_meta = parse_arg_meta(&field.attrs)?;
        let long = arg_meta.long.clone().unwrap_or_else(|| fname.to_string());
        let long_lit = syn::LitStr::new(&format!("--{}", long), field.span());
        let positional_lit = syn::LitStr::new(&long, field.span());
        let short = arg_meta.short;
        let short_lit = short.map(|c| syn::LitStr::new(&format!("-{}", c), field.span()));
        let short_opt_expr = if let Some(ch) = short {
            quote! { Some(#ch) }
        } else {
            quote! { None }
        };
        let help_text = arg_meta
            .help
            .clone()
            .unwrap_or_else(|| format!("Set {}", long));
        let is_positional = arg_meta.positional;

        if arg_meta.is_subcommand {
            declarations.push(quote! { let mut #fname: Option<#ty> = None; });
            subcommand_field = Some(fname.clone());
            field_inits.push(quote! { #fname });
            positional_names.push(syn::LitStr::new("<SUBCOMMAND>", ident.span()));
            positional_help_lines.push(quote! {
                positional_lines.push(format!("{} [{}], {}", #name_lit, "<SUBCOMMAND>", "Run a subcommand"));
            });
            continue;
        }

        let is_bool = matches!(&field.ty, Type::Path(p) if p.path.is_ident("bool"));
        let is_option = matches!(
            &field.ty,
            Type::Path(p) if p.path.segments.iter().any(|s| s.ident == "Option")
        );
        let default_expr = arg_meta.default_value.clone();

        if is_bool && !is_positional {
            declarations.push(quote! { let mut #fname: bool = false; });
            match_arms.push(quote! {
                #long_lit => { #fname = true; continue; }
            });
            if let Some(lit) = short_lit.clone() {
                match_arms.push(quote! {
                    #lit => { #fname = true; continue; }
                });
            }
            post_process.push(quote! { let #fname: #ty = #fname; });
        } else if is_positional {
            let is_vec = is_vec_string(&ty);
            if is_vec {
                declarations.push(quote! { let mut #fname: Vec<String> = Vec::new(); });
                needs_positionals_iter = true;
                post_process.push(quote! {
                    #fname.extend(positionals_iter.by_ref());
                    let #fname: #ty = #fname;
                });
            } else {
                declarations.push(quote! { let mut #fname: Option<String> = None; });
                needs_positionals_iter = true;
                let convert = if is_option {
                    quote! {
                        let #fname: #ty = match #fname.take() {
                            Some(val) => Some(val.parse().map_err(|_| ::micro_cli::CliError::MissingArgument(#long))?),
                            None => positionals_iter
                                .next()
                                .map(|val| val.parse().map_err(|_| ::micro_cli::CliError::MissingArgument(#long)))
                                .transpose()?,
                        };
                    }
                } else if let Some(default) = default_expr {
                    quote! {
                        let #fname: #ty = match #fname.take().or_else(|| positionals_iter.next()) {
                            Some(val) => val.parse().map_err(|_| ::micro_cli::CliError::MissingArgument(#long))?,
                            None => #default,
                        };
                    }
                } else {
                    has_required_fields = true;
                    quote! {
                        let #fname: #ty = #fname
                            .take()
                            .or_else(|| positionals_iter.next())
                            .ok_or_else(|| ::micro_cli::CliError::MissingArgument(#long))?
                            .parse()
                            .map_err(|_| ::micro_cli::CliError::MissingArgument(#long))?;
                    }
                };
                post_process.push(convert);
            }
        } else {
            declarations.push(quote! { let mut #fname: Option<String> = None; });
            match_arms.push(quote! {
                #long_lit => {
                    let value = iter.next().ok_or_else(|| ::micro_cli::CliError::MissingOptionValue(token.clone()))?;
                    #fname = Some(value);
                    continue;
                }
            });
            if let Some(lit) = short_lit.clone() {
                match_arms.push(quote! {
                    #lit => {
                        let value = iter.next().ok_or_else(|| ::micro_cli::CliError::MissingOptionValue(token.clone()))?;
                        #fname = Some(value);
                        continue;
                    }
                });
            }
            match_arms.push(quote! {
                _ if token.starts_with(#long_lit) && token.contains('=') => {
                    let parts: Vec<_> = token.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        #fname = Some(parts[1].to_string());
                        continue;
                    }
                }
            });

            let convert = if is_option {
                quote! {
                    let #fname: #ty = match #fname.take() {
                        Some(val) => val.parse().ok(),
                        None => None,
                    };
                }
            } else if let Some(default) = default_expr {
                quote! {
                    let #fname: #ty = match #fname.take() {
                        Some(val) => val.parse().map_err(|_| ::micro_cli::CliError::MissingArgument(#long))?,
                        None => #default,
                    };
                }
            } else {
                has_required_fields = true;
                quote! {
                    let #fname: #ty = #fname
                        .take()
                        .ok_or_else(|| ::micro_cli::CliError::MissingArgument(#long))?
                        .parse()
                        .map_err(|_| ::micro_cli::CliError::MissingArgument(#long))?;
                }
            };
            post_process.push(convert);
        }

        field_inits.push(quote! { #fname });
        if is_positional {
            positional_help_lines.push(quote! {
                positional_lines.push(format!("{} [{}], {}", #name_lit, #positional_lit, #help_text));
            });
            positional_names.push(positional_lit.clone());
        } else {
            option_help_lines.push(quote! {
                option_lines.push({
                    let mut flags = Vec::new();
                    if let Some(ch) = #short_opt_expr {
                        flags.push(format!("-{}", ch));
                    }
                    flags.push(format!("--{}", #long));
                    format!("{:<18} {}", flags.join(", "), #help_text)
                });
            });
        }
    }

    let subcommand_parse = if let Some(field) = subcommand_field {
        quote! {
            if #field.is_none() && !positionals.is_empty() {
                #field = Some(::micro_cli::Parser::parse_from(positionals.clone())?);
                positionals.clear();
            }
        }
    } else {
        quote! {}
    };
    let positionals_iter_decl = if needs_positionals_iter {
        quote! { let mut positionals_iter = positionals.into_iter(); }
    } else {
        quote! {}
    };

    let expanded = quote! {
        impl ::micro_cli::Parser for #ident {
            fn parse() -> Result<Self, ::micro_cli::CliError> {
                let args: Vec<String> = std::env::args().skip(1).collect();
                Self::parse_from(args)
            }

            fn parse_from<I, T>(iterable: I) -> Result<Self, ::micro_cli::CliError>
            where
                I: IntoIterator<Item = T>,
                T: Into<String>,
            {
                let mut iter = iterable.into_iter().map(Into::into).peekable();
                #(#declarations)*
                let mut positionals: Vec<String> = Vec::new();

                if #has_required_fields && iter.peek().is_none() {
                    return Err(::micro_cli::CliError::Help(Self::help()));
                }

                while let Some(token) = iter.next() {
                    if token == "--help" || token == "-h" {
                        return Err(::micro_cli::CliError::Help(Self::help()));
                    }
                    match token.as_str() {
                        #(#match_arms)*
                        _ => {
                            if token.starts_with('-') {
                                return Err(::micro_cli::CliError::UnknownOption(token));
                            }
                            positionals.push(token);
                        }
                    }
                }

                #subcommand_parse
                #positionals_iter_decl
                #(#post_process)*

                Ok(Self {
                    #(#field_inits),*
                })
            }

            fn help() -> String {
                let mut option_lines: Vec<String> = Vec::new();
                let mut positional_lines: Vec<String> = Vec::new();
                #(#option_help_lines)*
                #(#positional_help_lines)*

                let mut out = String::new();
                out.push_str("Usage: ");
                out.push_str(#name_lit);
                if !option_lines.is_empty() {
                    out.push_str(" [OPTIONS]");
                }
                let positional_names: &[&str] = &[#(#positional_names),*];
                for name in positional_names {
                    out.push_str(&format!(" [{}]", name));
                }
                out.push('\n');
                out.push_str(&format!("{}\n", #about_lit));

                if !option_lines.is_empty() {
                    out.push('\n');
                    out.push_str("Global options:\n");
                    for line in &option_lines {
                        out.push_str(&format!("  {}\n", line));
                    }
                    out.push_str("  -h, --help         Show help\n");
                }

                if !positional_lines.is_empty() {
                    out.push('\n');
                    out.push_str("Positional arguments:\n");
                    for line in &positional_lines {
                        out.push_str(&format!("  {}\n", line));
                    }
                }

                out
            }

            fn description() -> String {
                #about_lit.to_string()
            }

            fn name() -> &'static str {
                #name_lit
            }
        }
    };

    let command_info_impl = quote! {
        impl shell_parser::integration::CommandInfo for #ident {
            fn command_name(&self) -> &'static str {
                #name_lit
            }

            fn command_about(&self) -> &'static str {
                #about_lit
            }
        }
    };

    Ok(quote! {
        #expanded
        #command_info_impl
    })
}

fn expand_enum(
    ident: &Ident,
    data: &DataEnum,
    meta: &CommandMeta,
) -> syn::Result<proc_macro2::TokenStream> {
    let mut arms = Vec::new();
    let mut help_entries = Vec::new();
    for variant in &data.variants {
        let v_ident = &variant.ident;
        let name = v_ident.to_string().to_lowercase();
        help_entries.push(quote! { format!("  {}", #name) });
        match &variant.fields {
            Fields::Unit => {
                arms.push(quote! { #name => Ok(#ident::#v_ident), });
            }
            Fields::Named(fields) => {
                let mut decls = Vec::new();
                let mut opts = Vec::new();
                let mut finals = Vec::new();
                for field in &fields.named {
                    let fname = field.ident.clone().unwrap();
                    let ty = field.ty.clone();
                    let long = fname.to_string();
                    decls.push(quote! { let mut #fname: Option<String> = None; });
                    opts.push(quote! {
                        format!("--{}", #long) => {
                            let value = iter.next().ok_or_else(|| ::micro_cli::CliError::MissingOptionValue(token.clone()))?;
                            #fname = Some(value);
                            continue;
                        }
                    });
                    finals.push(quote! {
                        let #fname: #ty = #fname
                            .take()
                            .ok_or_else(|| ::micro_cli::CliError::MissingArgument(#long))?
                            .parse()
                            .map_err(|_| ::micro_cli::CliError::MissingArgument(#long))?;
                    });
                }
                let field_inits: Vec<_> = fields
                    .named
                    .iter()
                    .map(|f| f.ident.clone().unwrap())
                    .collect();
                arms.push(quote! {
                    #name => {
                        let mut iter = iter.collect::<Vec<_>>().into_iter();
                        #(#decls)*
                        while let Some(token) = iter.next() {
                            match token.as_str() {
                                #(#opts)*
                                _ => {}
                            }
                        }
                        #(#finals)*
                        Ok(#ident::#v_ident { #(#field_inits),* })
                    }
                });
            }
            _ => {
                return Err(syn::Error::new(
                    variant.span(),
                    "Only unit or named-field variants supported",
                ))
            }
        }
    }

    let about_lit = meta
        .about
        .clone()
        .unwrap_or_else(|| syn::LitStr::new(&format!("{} subcommands", ident), ident.span()));
    let name_lit = meta
        .name
        .clone()
        .unwrap_or_else(|| syn::LitStr::new(&ident.to_string(), ident.span()));

    let expanded = quote! {
        impl #ident {
            pub fn parse_from<I, T>(iterable: I) -> Result<Self, ::micro_cli::CliError>
            where
                I: IntoIterator<Item = T>,
                T: Into<String>,
            {
                let mut iter = iterable.into_iter().map(Into::into);
                let head = iter
                    .next()
                    .ok_or_else(|| ::micro_cli::CliError::MissingArgument("subcommand"))?;
                match head.as_str() {
                    #(#arms)*
                    _ => Err(::micro_cli::CliError::UnknownCommand(head)),
                }
            }

            pub fn help() -> String {
                let mut out = String::from("Subcommands:\n");
                #( out.push_str(&format!("{}\n", #help_entries)); )*
                out
            }

            pub fn description() -> String {
                #about_lit.to_string()
            }

            pub fn name() -> &'static str {
                #name_lit
            }
        }
    };

    let command_info_impl = quote! {
        impl shell_parser::integration::CommandInfo for #ident {
            fn command_name(&self) -> &'static str {
                #name_lit
            }

            fn command_about(&self) -> &'static str {
                #about_lit
            }
        }
    };

    Ok(quote! {
        #expanded
        #command_info_impl
    })
}
