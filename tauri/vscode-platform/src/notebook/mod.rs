//! Notebook serialization service.
//!
//! Mirrors extensions/ipynb/src/{notebookSerializer,deserializers,serializers}.ts.

pub mod data;
pub mod ipynb;
pub mod serializer;

#[cfg(test)]
mod tests;
