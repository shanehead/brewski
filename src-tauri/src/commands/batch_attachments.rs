use crate::error::AppError;
use crate::models::BatchAttachment;
use crate::repositories::batch_attachments::BatchAttachmentRepository;
use crate::AppState;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager, State};

pub fn attachments_dir(app: &AppHandle, batch_id: &str) -> Result<PathBuf, AppError> {
    let base = app
        .path()
        .app_data_dir()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(base.join("attachments").join(batch_id))
}

fn file_extension(path: &Path) -> String {
    path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("bin")
        .to_lowercase()
}

fn mime_for_extension(ext: &str) -> String {
    match ext {
        "jpg" | "jpeg" => "image/jpeg".to_string(),
        "png" => "image/png".to_string(),
        "gif" => "image/gif".to_string(),
        "webp" => "image/webp".to_string(),
        "heic" => "image/heic".to_string(),
        "pdf" => "application/pdf".to_string(),
        "doc" => "application/msword".to_string(),
        "docx" => {
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string()
        }
        "xls" => "application/vnd.ms-excel".to_string(),
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string(),
        "txt" => "text/plain".to_string(),
        _ => "application/octet-stream".to_string(),
    }
}

#[tauri::command]
pub async fn add_batch_attachment(
    app: AppHandle,
    state: State<'_, AppState>,
    batch_id: String,
    source_path: String,
    original_name: String,
) -> Result<BatchAttachment, AppError> {
    let src = Path::new(&source_path);
    let ext = file_extension(src);
    let mime = mime_for_extension(&ext);
    let uuid = uuid::Uuid::new_v4().to_string();
    let filename = format!("{uuid}.{ext}");
    let dest_dir = attachments_dir(&app, &batch_id)?;
    let dest = dest_dir.join(&filename);

    std::fs::create_dir_all(&dest_dir)
        .map_err(|e| AppError::Internal(format!("create attachments dir: {e}")))?;

    std::fs::copy(src, &dest).map_err(|e| AppError::Internal(format!("copy attachment: {e}")))?;

    let size_bytes = std::fs::metadata(&dest)
        .map_err(|e| AppError::Internal(format!("stat attachment: {e}")))?
        .len() as i64;

    BatchAttachmentRepository::new(&state.db)
        .create(
            &batch_id,
            &filename,
            &original_name,
            Some(&mime),
            size_bytes,
        )
        .await
}

#[tauri::command]
pub async fn list_batch_attachments(
    state: State<'_, AppState>,
    batch_id: String,
) -> Result<Vec<BatchAttachment>, AppError> {
    BatchAttachmentRepository::new(&state.db)
        .list(&batch_id)
        .await
}

#[tauri::command]
pub async fn delete_batch_attachment(
    app: AppHandle,
    state: State<'_, AppState>,
    id: String,
) -> Result<(), AppError> {
    let attachment = BatchAttachmentRepository::new(&state.db).get(&id).await?;
    let path = attachments_dir(&app, &attachment.batch_id)?.join(&attachment.filename);
    if let Err(e) = std::fs::remove_file(&path) {
        if e.kind() != std::io::ErrorKind::NotFound {
            return Err(AppError::Internal(format!("remove attachment file: {e}")));
        }
    }
    BatchAttachmentRepository::new(&state.db).delete(&id).await
}

#[tauri::command]
pub async fn open_batch_attachment(
    app: AppHandle,
    state: State<'_, AppState>,
    id: String,
) -> Result<(), AppError> {
    let attachment = BatchAttachmentRepository::new(&state.db).get(&id).await?;
    let path = attachments_dir(&app, &attachment.batch_id)?.join(&attachment.filename);
    tauri_plugin_opener::open_path(path, Option::<&str>::None)
        .map_err(|e| AppError::Internal(format!("open attachment: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_file_extension_jpg() {
        assert_eq!(file_extension(Path::new("brew-day.JPG")), "jpg");
    }

    #[test]
    fn test_file_extension_no_ext() {
        assert_eq!(file_extension(Path::new("noext")), "bin");
    }

    #[test]
    fn test_mime_for_extension_known() {
        assert_eq!(mime_for_extension("jpg"), "image/jpeg");
        assert_eq!(mime_for_extension("pdf"), "application/pdf");
        assert_eq!(mime_for_extension("png"), "image/png");
    }

    #[test]
    fn test_mime_for_extension_unknown() {
        assert_eq!(mime_for_extension("xyz"), "application/octet-stream");
    }

    #[test]
    fn test_image_copied_verbatim() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("photo.jpeg");
        let dest_dir = dir.path().join("attachments").join("batch1");
        std::fs::create_dir_all(&dest_dir).unwrap();
        let dest = dest_dir.join("photo_copy.jpeg");
        let bytes = b"\xff\xd8\xff fake jpeg bytes";
        std::fs::write(&src, bytes).unwrap();
        std::fs::copy(&src, &dest).unwrap();
        assert_eq!(std::fs::read(&dest).unwrap(), bytes);
    }
}
