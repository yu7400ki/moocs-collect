use crate::state::CollectState;
use collect::error::CollectError;
use tauri::State;

#[derive(Debug, thiserror::Error)]
pub enum ArchiveYearsError {
    #[error("Core library error: {0}")]
    Core(#[from] CollectError),
}

impl serde::Serialize for ArchiveYearsError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[tauri::command]
pub async fn get_archive_years(
    state: State<'_, CollectState>,
) -> Result<Vec<u32>, ArchiveYearsError> {
    let collect = &state.collect;

    let years = collect.get_archive_years().await?;

    Ok(years.into_iter().map(|year| year.value()).collect())
}
