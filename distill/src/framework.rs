
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameworkOptions {

    // "sm": "576px",
    // "md": "768px",
    // "lg": "992px",
    breakpoints: HashMap<String, String>,

    #[serde(default = "AtomType::ClassAttribute")]
    atom_type: AtomType,
}

pub enum BreakpointModifierPosition {
    Prefix(String)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AtomType {
    ClassAttribute,
    DataAttributes,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Plugin {
    CreateAtom(AtomCreationOptions),
}

pub struct AtomCreationOptions {
    
}

