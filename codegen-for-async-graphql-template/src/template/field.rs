use quote::quote;

use super::RendererFieldType;

use proc_macro2::{Ident, Span, TokenStream};

pub struct Renderer {
    // context: &'a mut Context<'b>,
}

impl Renderer {
    fn field_name_token(f: &RendererFieldType) -> TokenStream {
        let name = f.snaked_field_name();
        let name = Ident::new(name.as_str(), Span::call_site());
        quote!(#name)
    }

    fn struct_name(f: &RendererFieldType) -> String {
        let name = f.code_type_name();
        match (f.non_null(), f.is_list()) {
            (true, false) => name,
            (true, true) => format!("Vec<{}>", name),
            (false, false) => format!("FieldResult<{}>", name),
            (false, true) => format!("FieldResult<Vec<{}>>", name),
        }
    }

    fn struct_name_token(f: &RendererFieldType) -> TokenStream {
        let name = f.code_type_name();
        let name = Ident::new(&name, Span::call_site());
        match (f.non_null(), f.is_list()) {
            (true, false) => quote!(#name),
            (true, true) => quote!(Vec<#name>),
            (false, false) => quote!(FieldResult<#name>),
            (false, true) => quote!(FieldResult<Vec<#name>>),
        }
    }

    pub fn custom_field_token(f: &RendererFieldType) -> TokenStream {
        let n = &Self::field_name_token(f);
        let ty = &Self::struct_name_token(f);
        quote!(
            pub async fn #n(&self, ctx: &Context<'_>) -> #ty {
                ctx.data::<DataSource>().#n()
            }
        )
    }

    pub fn scalar_fields_token(f: &RendererFieldType) -> TokenStream {
        let n = &Self::field_name_token(f);
        let ty = &Self::struct_name_token(f);
        quote!(
            pub async fn #n(&self) -> #ty {
                self.#n.clone()
            }
        )
    }

    pub fn field_property_token(f: &RendererFieldType) -> TokenStream {
        let n = &Self::field_name_token(f);
        let ty = &Self::struct_name_token(f);
        quote!(
           pub #n : #ty
        )
    }

    pub fn field_interface_token(f: &RendererFieldType) -> TokenStream {
        let n = f.snaked_field_name();
        let ty = &Self::struct_name(f);
        quote!(
            field(name = #n, type = #ty)
        )
    }
}
