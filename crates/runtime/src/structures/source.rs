pub struct SourceEntry {
    pub name: String,
    pub source: String,
}

pub type SourceMap = std::collections::HashMap<String, SourceEntry>;