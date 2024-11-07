use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Branch {
    branch_name: String,
}
