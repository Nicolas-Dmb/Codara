use std::fmt;
use std::path::Path;
use crate::model::project::ProjectId;
use crate::model::run::RunId;
use crate::model::symbol::RawSymbol;
use crate::model::relation::RawRelation;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleId(String);

impl ModuleId {
    pub fn new(project_id: ProjectId, relative_path: String) -> Self {
        Self(format!("{}::{}", project_id, relative_path))
    }
}

impl fmt::Display for ModuleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug)]
pub struct Module {
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

impl RawModuleId {
    pub fn new(relative_path: &str) -> Self {
        Self(relative_path.to_string())
    }
}

impl fmt::Display for RawModuleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
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
    pub fn new(relative_path: String) -> Self {
        let name = Path::new(&relative_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

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

    pub fn into_parts(
        self,
        project_id: &ProjectId,
        run_id: &RunId,
    ) -> (Module, Vec<RawSymbol>, Vec<RawRelation>) {
        let module = Module::new(project_id.clone(), run_id.clone(), self.relative_path);
        (module, self.symbols, self.relations)
    }
}
