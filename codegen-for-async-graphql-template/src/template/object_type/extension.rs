use quote::quote;

use async_graphql_parser::schema::ObjectType;

use proc_macro2::{Ident, Span, TokenStream};

use super::{FieldRenderer, FileRender, RenderType, RendererObjectType, Save, SupportField};

pub struct Renderer<'a, 'b> {
    renderer_object_type: &'a RendererObjectType<'a, 'b>,
}

impl<'a, 'b> Renderer<'a, 'b> {
    pub fn model_file(renderer_object_type: &'a RendererObjectType<'a, 'b>) {
        let src = Renderer::token_stream(renderer_object_type);
        let file_name = renderer_object_type.file_name();
        ObjectType::save(&file_name, &src.to_string(), renderer_object_type.context);
    }

    pub fn token_stream(renderer_object_type: &'a RendererObjectType<'a, 'b>) -> TokenStream {
        let mut obj = Renderer {
            renderer_object_type,
        };

        let name = obj.name_token();
        let fields = obj.custom_fields_token();
        let struct_properties = obj.struct_properties_token();
        let scalar_fields_token = obj.scalar_fields_token();

        let uses = obj.uses();

        Self::object_type_code(
            &uses,
            &name,
            &struct_properties,
            &fields,
            &scalar_fields_token,
        )
    }

    fn uses(&self) -> TokenStream {
        let mut res = quote! {
            use async_graphql::*;
            use super::DataSource;
        };
        self.renderer_object_type
            .dependencies()
            .iter()
            .for_each(|f| {
                let module_name = Ident::new(&f.module_name, Span::call_site());
                let name = Ident::new(&f.name, Span::call_site());
                res = quote!(
                    #res
                    use super::#module_name::#name;
                )
            });

        res
    }

    fn name_token(&self) -> TokenStream {
        let name = Ident::new(&self.renderer_object_type.name(), Span::call_site());
        quote!(#name)
    }

    fn struct_properties_token(&mut self) -> TokenStream {
        let mut properties = quote! {};
        self.renderer_object_type
            .scalar_fields()
            .iter()
            .for_each(|f| {
                let field_property = FieldRenderer::field_property_token(f);
                properties = quote!(
                    #properties
                    #field_property
                );
            });
        properties
    }

    fn custom_fields_token(&mut self) -> TokenStream {
        let mut fields = quote! {};
        self.renderer_object_type
            .custom_fields()
            .iter()
            .for_each(|f| {
                let field = &FieldRenderer::custom_field_token(f);
                fields = quote!(
                    #fields
                    #field
                );
            });
        fields
    }

    fn object_type_code(
        uses: &TokenStream,
        name: &TokenStream,
        struct_properties: &TokenStream,
        fields: &TokenStream,
        scalar_fields_token: &TokenStream,
    ) -> TokenStream {
        quote!(
            #uses

            #[derive(Debug)]
            pub struct #name {
                #struct_properties
            }

            #[Object]
            impl #name {
                #fields
                #scalar_fields_token
            }
        )
    }

    fn scalar_fields_token(&mut self) -> TokenStream {
        let mut scalar_fields = quote! {};
        self.renderer_object_type
            .scalar_fields()
            .iter()
            .for_each(|f| {
                let field = FieldRenderer::scalar_fields_token(f);
                scalar_fields = quote!(
                    #scalar_fields
                    #field
                );
            });
        scalar_fields
    }
}
