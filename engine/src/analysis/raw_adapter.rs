use crate::model::{AnalysisReport, Module, ModuleId, ProjectId, RawModule, RawSymbol, Relation, RunId, Symbol, RawRelation, ServiceError};
use crate::persistence::AnalysisRepository;
use crate::services::Context;

pub struct RawAdapter<R: AnalysisRepository> {
    context: Context<R>,
    project_id: ProjectId,
    run_id: RunId,
}

impl<R: AnalysisRepository> RawAdapter<R> {
    pub fn new(context: Context<R>, project_id: ProjectId, run_id: RunId) -> Self {
        Self { context, project_id, run_id }
    }


    pub async fn convert_and_store(&self, analysis: AnalysisReport) -> Result<(), ServiceError> {
        let (modules, symbols, relations) = self.convert_module_and_children(analysis.raw_modules);
        self.context.analysis_repo.store_batch(&modules, &symbols, &relations).await?;
        Ok(())
    }

    fn convert_module_and_children(&self, raw_modules: Vec<RawModule>) -> (Vec<Module>, Vec<Symbol>, Vec<Relation>) {
        let mut modules = Vec::new();
        let mut symbols = Vec::new();
        let mut relations = Vec::new();
        for raw_module in raw_modules {
            let (module, raw_symbols, raw_relations) = raw_module.into_parts(&self.project_id, &self.run_id);
            let (mut syms,mut rels_from_syms) = self.convert_symbol(raw_symbols, &module.id);
            let mut extracted_relations = self.convert_relation(raw_relations, &module.id);
            modules.push(module);
            symbols.append(&mut syms);
            relations.append(&mut extracted_relations);
            relations.append(&mut rels_from_syms);
        }
        return (modules, symbols, relations);
    }

    fn convert_symbol(&self, raw_symbols: Vec<RawSymbol>, module_id: &ModuleId) -> (Vec<Symbol>, Vec<Relation>) {
        let mut symbols = Vec::new();
        let mut relations = Vec::new();
        for raw_symbol in raw_symbols {
            let (symbol, children_symbols, children_relations) = raw_symbol.into_symbol(module_id, &self.run_id);
            let (mut child_syms, mut child_rels) = self.convert_symbol(children_symbols, module_id);
            let mut extracted_relations = self.convert_relation(children_relations, module_id);
            symbols.push(symbol);
            symbols.append(&mut child_syms);
            relations.append(&mut child_rels);
            relations.append(&mut extracted_relations);
        }
        ( symbols, relations )
    }

    fn convert_relation(&self, raw_relations: Vec<RawRelation>, module_id: &ModuleId) -> Vec<Relation>{
        raw_relations.into_iter()                                                                                                                
            .map(|r| r.into_relation(module_id, &self.run_id))                                                                                         
            .collect()   
    }   

    fn resolve_relation_target(&self, relations: Vec<Relation>, symbols: Vec<Symbol>)->(){
        unimplemented!()
    }

}
