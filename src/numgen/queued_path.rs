use super::Path;

#[derive(Clone)]
pub struct QueuedPath {
    pub path: Path,
    pub priority: usize,
}

impl PartialEq for QueuedPath {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for QueuedPath {}

impl Ord for QueuedPath {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // we make it a min queue by inverting the ordering
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for QueuedPath {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<QueuedPath> for Path {
    fn from(qp: QueuedPath) -> Self {
        qp.path
    }
}
