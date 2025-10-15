#[derive(serde::Deserialize, Debug)]
pub struct OutputPre {
    filename: Option<String>,
    directory: Option<String>,
}
