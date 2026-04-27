use std::path::Path;

const CODEBASE_PATH: &Path = Path::new("/codebase");

// TODO
// test -> walker should : 
// catch all errors that stuck the analysis 
// should return all partial errors to finalizer  
fn walk(run: &AnalysisRun, project: &Project, path: Option<&Path>) {
    
    let path = path.unwrap_or_else(|| Path::new(CODEBASE_PATH));

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        if entry_path.is_file() {
            // analyze file and send results to finalizer
            continue;
        }
        if entry_path.is_dir() {
            walk(run, project, &entry_path);
        }
    }
}