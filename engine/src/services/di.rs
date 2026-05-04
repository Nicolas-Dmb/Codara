use crate::persistence::{AnalysisRepository, RunRepository};

pub struct Context<A: AnalysisRepository, R: RunRepository> {
    pub analysis_repo: A,
    pub run_repo: R
}

impl<A: AnalysisRepository, R: RunRepository> Context<A, R> {
    pub fn new(analysis_repo: A, run_repo: R) -> Self {
        Self { analysis_repo, run_repo }
    }
}

