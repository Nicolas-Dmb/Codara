

#[derive(Debug, Eq)]
pub struct ModuleId(String);

impl ModuleId {
    pub fn new(project_id: ProjectId, relative_path: String) -> Self {
        Self(format!("{}_{}", project_id.value(), relative_path))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct Module{
    pub name: ModuleId,
    pub run_id: RunId,
    pub relative_path: String,
    pub name: String,
}

impl Module {
    pub fn new(project_id: ProjectId, run_id: RunId, relative_path: String) -> Self {
        let name = ModuleId::new(project_id, relative_path.clone());
        Self {
            name,
            run_id,
            relative_path,
        }
    }
}