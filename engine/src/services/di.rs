use crate::persistence::AnalysisRepository;

pub struct Context<R: AnalysisRepository> {
    pub analysis_repo: R,
}

impl<R: AnalysisRepository> Context<R> {
    pub fn new(analysis_repo: R) -> Self {
        Self { analysis_repo }
    }
}