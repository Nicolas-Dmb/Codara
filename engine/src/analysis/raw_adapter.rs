use crate::model::{AnalysisReport, Module, ModuleId, ProjectId, RawModule, RawSymbol, Relation, RunError, RunId, Symbol, relation, RawRelation};


pub fn convert_and_store(analysis: AnalysisReport, project_id: ProjectId, run_id: RunId) -> () {
    let (module, symbol, relation) = convert_module_and_children(analysis.raw_modules, project_id, run_id);
    
}

fn convert_module_and_children(raw_modules: Vec<RawModule>, project_id: ProjectId, run_id: RunId) -> (Vec<Module>, Vec<Symbol>, Vec<Relation>) {
    let mut modules = Vec::new();
    let mut symbols = Vec::new();
    let mut relations = Vec::new();
    for raw_module in raw_modules {
        let (module, raw_symbols, raw_relations) = raw_module.into_parts(&project_id, &run_id);
        let (mut syms,mut rels_from_syms) = convert_symbol(raw_symbols, &module.id, &run_id);
        let mut extracted_relations = convert_relation(raw_relations, &module.id, &run_id);
        modules.push(module);
        symbols.append(&mut syms);
        relations.append(&mut extracted_relations);
        relations.append(&mut rels_from_syms);
    }
    return (modules, symbols, relations);
}

fn convert_symbol(raw_symbols: Vec<RawSymbol>, module_id: &ModuleId, run_id: &RunId ) -> (Vec<Symbol>, Vec<Relation>) {
    let mut symbols = Vec::new();
    let mut relations = Vec::new();
    for raw_symbol in raw_symbols {
        let (symbol, children_symbols, children_relations) = raw_symbol.into_symbol(module_id, run_id);
        let (mut child_syms, mut child_rels) = convert_symbol(children_symbols, module_id, run_id);
        let mut extracted_relations = convert_relation(children_relations, module_id, run_id);
        symbols.push(symbol);
        symbols.append(&mut child_syms);
        relations.append(&mut child_rels);
        relations.append(&mut extracted_relations);
    }
    ( symbols, relations )
}

fn convert_relation(raw_relations: Vec<RawRelation>, module_id: &ModuleId, run_id: &RunId) -> Vec<Relation>{
    raw_relations.into_iter()                                                                                                                
          .map(|r| r.into_relation(module_id, run_id))                                                                                         
          .collect()   
}   

fn resolve_relation_target(relations: Vec<Relation>, symbols: Vec<Symbol>)->(){
    unimplemented!()
}
