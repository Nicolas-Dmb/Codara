use crate::model::{AnalysisReport, Module, ModuleId, ProjectId, RawModule, RawSymbol, Relation, RunId, Symbol, RawRelation, ServiceError, SymbolId};
use crate::persistence::{AnalysisRepository};

pub struct RawAdapter<A: AnalysisRepository> {
    analysis_repo: A,
    project_id: ProjectId,
    run_id: RunId,
}

impl<A: AnalysisRepository> RawAdapter<A> {
    pub fn new(analysis_repo: A, project_id: ProjectId, run_id: RunId) -> Self {
        Self { analysis_repo, project_id, run_id }
    }

    pub async fn convert_and_store(&self, analysis: AnalysisReport) -> Result<(), ServiceError> {
        let (modules, symbols, relations) = self.convert_module_and_children(analysis.raw_modules);
        self.analysis_repo.store_batch(&self.run_id.to_string(), &modules, &symbols, &relations, &analysis.warnings, &analysis.retryables, &analysis.source_code_issues).await?;
        Ok(())
    }

    fn convert_module_and_children(&self, raw_modules: Vec<RawModule>) -> (Vec<Module>, Vec<Symbol>, Vec<Relation>) {
        let mut modules = Vec::new();
        let mut symbols = Vec::new();
        let mut relations = Vec::new();
        for raw_module in raw_modules {
            let (module, raw_symbols, raw_relations) = raw_module.into_parts(&self.project_id, &self.run_id);
            let (mut syms,mut rels_from_syms) = self.convert_symbol(raw_symbols, &module.id, None);
            let mut extracted_relations = self.convert_relation(raw_relations, &module.id, None);
            modules.push(module);
            symbols.append(&mut syms);
            relations.append(&mut extracted_relations);
            relations.append(&mut rels_from_syms);
        }
        return (modules, symbols, relations);
    }

    fn convert_symbol(&self, raw_symbols: Vec<RawSymbol>, module_id: &ModuleId, parent_symbol_id: Option<&SymbolId>) -> (Vec<Symbol>, Vec<Relation>) {
        let mut symbols = Vec::new();
        let mut relations = Vec::new();
        for raw_symbol in raw_symbols {
            let (symbol, children_symbols, children_relations) = raw_symbol.into_symbol(module_id, &self.run_id, parent_symbol_id);
            let (mut child_syms, mut child_rels) = self.convert_symbol(children_symbols, module_id, Some(&symbol.id));
            let mut extracted_relations = self.convert_relation(children_relations, module_id, Some(&symbol.id));
            symbols.push(symbol);
            symbols.append(&mut child_syms);
            relations.append(&mut child_rels);
            relations.append(&mut extracted_relations);
        }
        ( symbols, relations )
    }

    fn convert_relation(&self, raw_relations: Vec<RawRelation>, module_id: &ModuleId, parent_symbol_id: Option<&SymbolId>) -> Vec<Relation>{
        raw_relations.into_iter()                                                                                                                
            .map(|r| r.into_relation(module_id, &self.run_id, parent_symbol_id))                                                                                         
            .collect()   
    }   

    fn resolve_relation_target(&self, relations: Vec<Relation>, symbols: Vec<Symbol>)->(){
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use crate::model::{RelationKind, SymbolKind, AnalysisWarning, RetryableIssue, SourceCodeIssue};

    struct FakeAnalysisRepository {
        stored: Mutex<Option<(Vec<Module>, Vec<Symbol>, Vec<Relation>)>>,
    }

    impl FakeAnalysisRepository {
        fn new() -> Self {
            Self { stored: Mutex::new(None) }
        }
    }

    impl AnalysisRepository for FakeAnalysisRepository {
        async fn store_batch(
            &self,
            _run_id: &str,
            _modules: &[Module],
            _symbols: &[Symbol],
            _relations: &[Relation],
            _warnings: &[AnalysisWarning],
            _retryable_issues: &[RetryableIssue],
            _source_code_issues: &[SourceCodeIssue],
        ) -> Result<(), ServiceError> {
            *self.stored.lock().unwrap() = None;
            Ok(())
        }
    }

    fn test_project_id() -> ProjectId {
        ProjectId::new("test-ns".to_string(), "test-project".to_string())
    }

    fn test_run_id(project_id: &ProjectId) -> RunId {
        RunId::new(project_id, "abc123")
    }

    fn make_adapter() -> RawAdapter<FakeAnalysisRepository> {
        let project_id = test_project_id();
        let run_id = test_run_id(&project_id);
        let analysis_repo = FakeAnalysisRepository::new();
        RawAdapter::new(analysis_repo, project_id, run_id)
    }

    #[test]
    fn empty_modules_returns_empty_vecs() {
        let adapter = make_adapter();
        let (modules, symbols, relations) = adapter.convert_module_and_children(vec![]);
        assert!(modules.is_empty());
        assert!(symbols.is_empty());
        assert!(relations.is_empty());
    }

    #[test]
    fn single_module_no_symbols_no_relations() {
        let adapter = make_adapter();
        let raw = RawModule::new("src/main.py".to_string());

        let (modules, symbols, relations) = adapter.convert_module_and_children(vec![raw]);

        assert_eq!(modules.len(), 1);
        assert_eq!(modules[0].relative_path, "src/main.py");
        assert_eq!(modules[0].name, "main.py");
        assert!(symbols.is_empty());
        assert!(relations.is_empty());
    }

    #[test]
    fn module_with_flat_symbols() {
        let adapter = make_adapter();
        let mut raw = RawModule::new("src/utils.py".to_string());
        raw.add_symbol(RawSymbol::new(
            "my_func".to_string(), SymbolKind::Function,
            "a function".to_string(), "src/utils.py".to_string(), 1, 10,
        ));
        raw.add_symbol(RawSymbol::new(
            "MyClass".to_string(), SymbolKind::Class,
            "a class".to_string(), "src/utils.py".to_string(), 12, 30,
        ));

        let (modules, symbols, relations) = adapter.convert_module_and_children(vec![raw]);

        assert_eq!(modules.len(), 1);
        assert_eq!(symbols.len(), 2);
        assert_eq!(symbols[0].name, "my_func");
        assert_eq!(symbols[1].name, "MyClass");
        assert!(symbols[0].parent_symbol_id.is_none());
        assert!(symbols[1].parent_symbol_id.is_none());
        assert_eq!(symbols[0].module_id.to_string(), modules[0].id.to_string());
        assert_eq!(symbols[1].module_id.to_string(), modules[0].id.to_string());
        assert!(relations.is_empty());
    }

    #[test]
    fn nested_symbols_get_parent_id() {
        let adapter = make_adapter();
        let mut raw = RawModule::new("src/models.py".to_string());

        let mut class_sym = RawSymbol::new(
            "MyClass".to_string(), SymbolKind::Class,
            "".to_string(), "src/models.py".to_string(), 1, 20,
        );
        class_sym.add_children_symbol(RawSymbol::new(
            "my_method".to_string(), SymbolKind::Method,
            "".to_string(), "src/models.py".to_string(), 5, 15,
        ));
        raw.add_symbol(class_sym);

        let (_modules, symbols, _relations) = adapter.convert_module_and_children(vec![raw]);

        assert_eq!(symbols.len(), 2);
        assert_eq!(symbols[0].name, "MyClass");
        assert!(symbols[0].parent_symbol_id.is_none());
        assert_eq!(symbols[1].name, "my_method");
        assert_eq!(
            symbols[1].parent_symbol_id.as_ref().unwrap().to_string(),
            symbols[0].id.to_string(),
        );
    }

    #[test]
    fn deeply_nested_symbols() {
        let adapter = make_adapter();
        let mut raw = RawModule::new("src/deep.py".to_string());

        let mut inner_method = RawSymbol::new(
            "inner".to_string(), SymbolKind::Function,
            "".to_string(), "src/deep.py".to_string(), 10, 15,
        );
        inner_method.add_children_symbol(RawSymbol::new(
            "helper".to_string(), SymbolKind::Function,
            "".to_string(), "src/deep.py".to_string(), 11, 14,
        ));

        let mut class_sym = RawSymbol::new(
            "Outer".to_string(), SymbolKind::Class,
            "".to_string(), "src/deep.py".to_string(), 1, 20,
        );
        class_sym.add_children_symbol(inner_method);
        raw.add_symbol(class_sym);

        let (_modules, symbols, _relations) = adapter.convert_module_and_children(vec![raw]);

        assert_eq!(symbols.len(), 3);
        assert!(symbols[0].parent_symbol_id.is_none());
        assert_eq!(
            symbols[1].parent_symbol_id.as_ref().unwrap().to_string(),
            symbols[0].id.to_string(),
        );
        assert_eq!(
            symbols[2].parent_symbol_id.as_ref().unwrap().to_string(),
            symbols[1].id.to_string(),
        );
    }

    #[test]
    fn module_level_relations() {
        let adapter = make_adapter();
        let mut raw = RawModule::new("src/main.py".to_string());
        raw.add_relation(RawRelation::new(
            RelationKind::Import,
            "os".to_string(),
            "src/main.py".to_string(),
            None, 1,
        ));

        let (modules, symbols, relations) = adapter.convert_module_and_children(vec![raw]);

        assert_eq!(modules.len(), 1);
        assert!(symbols.is_empty());
        assert_eq!(relations.len(), 1);
        assert_eq!(relations[0].imported_name, "os");
        assert_eq!(relations[0].module_id.to_string(), modules[0].id.to_string());
        assert!(relations[0].parent_symbol_id.is_none());
    }

    #[test]
    fn relations_inside_symbol_get_parent_symbol_id() {
        let adapter = make_adapter();
        let mut raw = RawModule::new("src/service.py".to_string());

        let mut class_sym = RawSymbol::new(
            "Service".to_string(), SymbolKind::Class,
            "".to_string(), "src/service.py".to_string(), 1, 30,
        );
        class_sym.add_children_relation(RawRelation::new(
            RelationKind::Import,
            "json".to_string(),
            "src/service.py".to_string(),
            None, 5,
        ));
        raw.add_symbol(class_sym);

        let (_modules, symbols, relations) = adapter.convert_module_and_children(vec![raw]);

        assert_eq!(symbols.len(), 1);
        assert_eq!(relations.len(), 1);
        assert_eq!(
            relations[0].parent_symbol_id.as_ref().unwrap().to_string(),
            symbols[0].id.to_string(),
        );
    }

    #[test]
    fn multiple_modules_aggregated() {
        let adapter = make_adapter();
        let mut raw1 = RawModule::new("src/a.py".to_string());
        raw1.add_symbol(RawSymbol::new(
            "func_a".to_string(), SymbolKind::Function,
            "".to_string(), "src/a.py".to_string(), 1, 5,
        ));
        let mut raw2 = RawModule::new("src/b.py".to_string());
        raw2.add_symbol(RawSymbol::new(
            "func_b".to_string(), SymbolKind::Function,
            "".to_string(), "src/b.py".to_string(), 1, 5,
        ));

        let (modules, symbols, relations) = adapter.convert_module_and_children(vec![raw1, raw2]);

        assert_eq!(modules.len(), 2);
        assert_eq!(symbols.len(), 2);
        assert!(relations.is_empty());
        assert_eq!(symbols[0].module_id.to_string(), modules[0].id.to_string());
        assert_eq!(symbols[1].module_id.to_string(), modules[1].id.to_string());
    }

    #[tokio::test]
    async fn convert_and_store_calls_repository() {
        let adapter = make_adapter();
        let mut raw = RawModule::new("src/main.py".to_string());
        raw.add_symbol(RawSymbol::new(
            "main".to_string(), SymbolKind::Function,
            "".to_string(), "src/main.py".to_string(), 1, 10,
        ));

        let mut report = AnalysisReport::new();
        report.raw_modules.push(raw);

        let result = adapter.convert_and_store(report).await;
        assert!(result.is_ok());
    }
}
