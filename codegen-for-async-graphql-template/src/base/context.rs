use super::{
    Config, FileRender, RenderType, RendererInterfaceType, RendererObjectType, RendererScalarType,
};
use async_graphql_parser::schema::{Definition, Document, TypeDefinition};

#[derive(Debug, Clone)]
pub struct Context<'a> {
    pub config: &'a Config,
    doc: &'a Document,
}

impl<'a> Context<'a> {
    #[must_use]
    pub const fn new(config: &'a Config, doc: &'a Document) -> Self {
        Self { config, doc }
    }

    #[must_use]
    pub fn scalar_names(&self) -> Vec<String> {
        self.scalar_types()
            .iter()
            .map(RendererScalarType::name)
            .collect()
    }

    #[must_use]
    pub fn file_names(&self) -> Vec<String> {
        let mut scalar_names: Vec<String> = self
            .scalar_types()
            .iter()
            .map(RendererScalarType::file_name)
            .collect();

        let object_type_names: Vec<String> = self
            .object_types()
            .iter()
            .map(RendererObjectType::file_name)
            .collect();

        let interface_type_names: Vec<String> = self
            .interface_types()
            .iter()
            .map(RendererInterfaceType::file_name)
            .collect();

        scalar_names.extend(object_type_names);
        scalar_names.extend(interface_type_names);
        scalar_names
    }

    fn type_definition(&self) -> Vec<&TypeDefinition> {
        self.doc
            .definitions
            .iter()
            .filter_map(|f| match &f.node {
                Definition::TypeDefinition(n) => Some(&n.node),
                _ => panic!("Not implemented:{:?}", f),
            })
            .collect()
    }

    #[must_use]
    pub fn object_types(&self) -> Vec<RendererObjectType> {
        self.type_definition()
            .iter()
            .filter_map(|f| match &f {
                TypeDefinition::Object(f) => Some(RendererObjectType {
                    doc: &f.node,
                    context: self,
                }),
                TypeDefinition::Scalar(_f) => None,
                TypeDefinition::Interface(_f) => None,
                _ => panic!("Not implemented"),
            })
            .collect()
    }

    #[must_use]
    pub fn scalar_types(&self) -> Vec<RendererScalarType> {
        self.type_definition()
            .iter()
            .filter_map(|f| match &f {
                TypeDefinition::Scalar(f) => Some(RendererScalarType {
                    doc: &f.node,
                    context: self,
                }),
                _ => None,
            })
            .collect()
    }

    #[must_use]
    pub fn interface_types(&self) -> Vec<RendererInterfaceType> {
        self.type_definition()
            .iter()
            .filter_map(|f| match &f {
                TypeDefinition::Interface(f) => Some(RendererInterfaceType {
                    doc: &f.node,
                    context: self,
                }),
                _ => None,
            })
            .collect()
    }
}
