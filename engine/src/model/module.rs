use crate::model::project::ProjectId;
use crate::model::run::RunId;
use std::path::Path;
use crate::model::symbol::RawSymbol;
use crate::model::relation::RawRelation;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleId(String);

impl ModuleId {
    pub fn new(project_id: ProjectId, relative_path: String) -> Self {
        Self(format!("{}::{}", project_id.value(), relative_path))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct Module{
    pub id: ModuleId,
    pub run_id: RunId,
    pub relative_path: String,
    pub name: String,
}

impl Module {
    pub fn new(project_id: ProjectId, run_id: RunId, relative_path: String) -> Self {
        let id = ModuleId::new(project_id, relative_path.clone());
        let name = Path::new(&relative_path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_string();
        
        Self {
            id,
            run_id,
            relative_path,
            name,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RawModuleId(String);

impl RawModuleId{
    pub fn new(relative_path: &str) -> Self {
        Self(relative_path.to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq)]
pub struct RawModule {
    pub id: RawModuleId,
    pub relative_path: String,
    pub name: String,
    pub symbols: Vec<RawSymbol>,
    pub relations: Vec<RawRelation>,
}

impl RawModule {
    pub fn new(relative_path: String, name: String) -> Self {
        Self {
            id: RawModuleId::new(&relative_path),
            relative_path,
            name,
            symbols: Vec::new(),
            relations: Vec::new(),
        }
    }

    pub fn add_symbol(&mut self, symbol: RawSymbol) {
        self.symbols.push(symbol);
    }

    pub fn add_relation(&mut self, relation: RawRelation) {
        self.relations.push(relation);
    }
}
           