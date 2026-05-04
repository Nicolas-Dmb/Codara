use crate::model;
use model::Run;
use crate::persistence::{AnalysisRepository, RunRepository};
use crate::services::Context;

pub fn initializer<A: AnalysisRepository, R: RunRepository>(context: Context<A, R>, run: &Run) {
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