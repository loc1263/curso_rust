//! Macros procedurales del módulo 27.
//!
//! Una macro procedural es una función Rust que recibe un `TokenStream` y devuelve otro.
//! Se ejecuta EN COMPILACIÓN. `syn` parsea los tokens a un árbol sintáctico y `quote` genera código.
//!
//! Tres tipos:
//! - derive:          `#[derive(Describir)]`
//! - attribute:       `#[medir_tiempo]`
//! - function-like:   `mayusculas!("texto")`

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, ItemFn, LitStr, parse_macro_input};

/// `#[derive(Describir)]` añade `fn describir() -> String` con el nombre del tipo y sus campos/variantes.
#[proc_macro_derive(Describir)]
pub fn derive_describir(entrada: TokenStream) -> TokenStream {
    // 1) Parsear los tokens a un DeriveInput
    let ast = parse_macro_input!(entrada as DeriveInput);
    let nombre = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    // 2) Recoger la información que necesitamos
    let detalle: String = match &ast.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(f) => {
                let campos: Vec<String> = f
                    .named
                    .iter()
                    .map(|c| {
                        let ty = &c.ty;
                        format!("{}: {}", c.ident.as_ref().unwrap(), quote!(#ty))
                    })
                    .collect();
                format!("struct con campos [{}]", campos.join(", "))
            }
            Fields::Unnamed(f) => format!("tuple struct con {} campos", f.unnamed.len()),
            Fields::Unit => "unit struct".to_string(),
        },
        Data::Enum(e) => {
            let variantes: Vec<String> = e.variants.iter().map(|v| v.ident.to_string()).collect();
            format!("enum con variantes [{}]", variantes.join(", "))
        }
        Data::Union(_) => {
            // Los errores de macro se reportan con `syn::Error`: apuntan al código del usuario
            return syn::Error::new_spanned(&ast.ident, "Describir no soporta uniones")
                .to_compile_error()
                .into();
        }
    };
    let texto = format!("{nombre}: {detalle}");

    // 3) Generar el código con quote!
    quote! {
        impl #impl_generics #nombre #ty_generics #where_clause {
            pub fn describir() -> String {
                #texto.to_string()
            }
        }
    }
    .into()
}

/// `#[medir_tiempo]` envuelve una función para imprimir cuánto tarda.
#[proc_macro_attribute]
pub fn medir_tiempo(_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut funcion = parse_macro_input!(item as ItemFn);
    let nombre = funcion.sig.ident.to_string();
    let cuerpo = funcion.block;

    // Reemplazamos el cuerpo por uno que mide el tiempo. El `return` del cuerpo original
    // sigue funcionando porque lo metemos dentro de un closure.
    funcion.block = syn::parse_quote!({
        let __inicio = ::std::time::Instant::now();
        let __resultado = (|| #cuerpo)();
        println!("[medir_tiempo] {} tardó {:?}", #nombre, __inicio.elapsed());
        __resultado
    });
    quote!(#funcion).into()
}

/// `mayusculas!("hola")` se convierte en el literal `"HOLA"` EN COMPILACIÓN (cero coste en ejecución).
#[proc_macro]
pub fn mayusculas(entrada: TokenStream) -> TokenStream {
    let literal = parse_macro_input!(entrada as LitStr);
    let en_mayus = literal.value().to_uppercase();
    quote!(#en_mayus).into()
}
