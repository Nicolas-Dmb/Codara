#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: ./new_adapter.sh <language>"
    exit 1
fi

LANG_NAME="$1"
ADAPTER_NAME="$(echo "${LANG_NAME:0:1}" | tr '[:lower:]' '[:upper:]')${LANG_NAME:1}Adapter"

if [ -d "src/adapters/$LANG_NAME" ]; then
    echo "Adapter '$LANG_NAME' already exists"
    exit 1
fi

echo "Generating adapter files for '$LANG_NAME'..."

mkdir -p "src/adapters/$LANG_NAME"

cat > "src/adapters/$LANG_NAME/mod.rs" << EOF
use crate::analysis::connector::Adapter;
use crate::model::{AnalysisWarning, ExtractionIssue, RawModule};

/// $ADAPTER_NAME adapter for \`.$LANG_NAME\` files.
///
/// Supported SymbolKinds: TODO
/// Supported RelationKinds: TODO
/// Ignored files: TODO
pub struct $ADAPTER_NAME {}

impl Adapter for $ADAPTER_NAME {

    fn supported_extensions(&self) -> &[&'static str] {
        // TODO: return the supported extensions for this adapter
        &[]
    }

    fn ignore_files(&self) -> &[&'static str] {
        // TODO: return the list of files to ignore for this adapter
        &[]
    }

    fn extract(&self, url: &str) -> Result<RawModule, ExtractionIssue> {
        if self.should_ignore(url) {
            return Err(ExtractionIssue::Warning(AnalysisWarning::IgnoredFile { path: url.to_string() }));
        }
        let mut raw_module = RawModule::new(url.to_string());

        let source_code = self.read_source_code(url)?;

        // TODO: extract symbols and relations from the source code and fill the raw_module

        Ok(raw_module)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    adapter_contract_tests!(
        adapter: $ADAPTER_NAME {},
        extension: //TODO: provide a valid extension for this adapter (e.g., ".java"),
        valid_source: // TODO: provide a valid source code example for this adapter
        ignored_filename: // TODO: return the list of files to ignore for this adapter,
    );
}

EOF

echo "Created src/adapters/$LANG_NAME/mod.rs"
echo "Don't forget to register the adapter in src/adapters/mod.rs"