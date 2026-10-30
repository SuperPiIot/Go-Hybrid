use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeShape {
    Primitive,
    Str,
    Atomic,
    Chan(Box<TypeShape>),
    Transactional(Box<TypeShape>), // Adicionar após a definição de TypeShape::Unknown
    Unknown,
}

impl TypeShape {
    pub(crate) fn is_sendable_by_type(&self) -> bool {
        match self {
            TypeShape::Primitive | TypeShape::Str | TypeShape::Atomic | TypeShape::Chan(_) | TypeShape::Transactional(_) => true,
            TypeShape::Pointer(inner) => matches!(**inner, TypeShape::Atomic),
            _ => false,
        }
    }
}