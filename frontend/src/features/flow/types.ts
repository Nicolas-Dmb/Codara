
export interface SymbolResponse {
    id: string;
    run_id: string;
    module_id: string;
    parent_symbol_id: string | null;
    name: string;
    kind: string;
    doc: string;
    location: string;
    start_line: number;
    end_line: number;
}

export interface RelationResponse {
    id: string;
    run_id: string;
    module_id: string;
    parent_symbol_id: string | null;
    imported_name: string;
    source_path: string;
    target_symbol_id: string | null;
    kind: string;
    line: number;
}


export interface GraphResponse {
    symbols: SymbolResponse[];
    relations: RelationResponse[];
}