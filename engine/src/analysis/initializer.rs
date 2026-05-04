use crate::model;
use model::run::Run;
use crate::persistence::AnalysisRepository;
use crate::services::Context;

pub fn initializer<R: AnalysisRepository>(context: Context<R>, run: &Run) {
    // request project data 
    // init run as Running
    // clone projet un codebase 
    // set at right branch and commit 
    
    //try{
        // send to walker 
    //}cacth {
        // init run as Failed
    //}
}