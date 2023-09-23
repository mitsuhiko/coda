use std::collections::HashSet;
use std::hash::Hash;

use uuid::Uuid;

pub struct Supervisor {
    supervisor_id: Uuid,
    workflows: HashSet<String>,
    tasks: HashSet<String>,
}
