use super::error::ProjectError;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectId(String);

impl ProjectId {
    pub fn new(namespace: String, project_name: String) -> Self {
        Self(format!("{}::{}", namespace, project_name))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub repo_url: String,
}

impl Project {
    fn split_repo_url(repo_url: &str) -> Result<(String, String), ProjectError> {
        let cleaned_url = repo_url.trim_end_matches('/');
        let parts: Vec<&str> = cleaned_url.split('/').collect();

        let name = parts
            .last()
            .ok_or(ProjectError::MissingRepositoryName)?
            .trim_end_matches(".git")
            .to_string();

        let namespace = parts
            .get(parts.len().saturating_sub(2))
            .ok_or(ProjectError::MissingNamespace)?
            .to_string();

        if namespace.is_empty() || name.is_empty() {
            return Err(ProjectError::InvalidRepositoryUrl);
        }

        Ok((namespace, name))
    }

    pub fn new(repo_url: String) -> Result<Self, ProjectError> {
        let (namespace, name) = Self::split_repo_url(&repo_url)?;

        let id = ProjectId::new(namespace, name.clone());

        Ok(Self {
            id,
            name,
            repo_url,
        })
    }
}