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
    use image::{ImageBuffer, Rgb};
    use tempfile::tempdir;

    fn make_png(path: &Path, width: u32, height: u32) {
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
            ImageBuffer::from_fn(width, height, |x, _y| Rgb([x as u8, 100, 200]));
        img.save(path).unwrap();
    }

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
    fn test_image_extension_is_detected() {
        assert!(IMAGE_EXTENSIONS.contains(&"jpg"));
        assert!(IMAGE_EXTENSIONS.contains(&"jpeg"));
        assert!(IMAGE_EXTENSIONS.contains(&"png"));
        assert!(!IMAGE_EXTENSIONS.contains(&"pdf"));
        assert!(!IMAGE_EXTENSIONS.contains(&"docx"));
    }

    #[test]
    fn test_write_image_called_for_png() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("photo.png");
        let dest = dir.path().join("out/photo_out.jpg");
        make_png(&src, 200, 200);
        write_image(&src, &dest).unwrap();
        assert!(dest.exists());
        let img = image::open(&dest).unwrap();
        assert!(img.width() <= 1200);
    }

    #[test]
    fn test_copy_used_for_pdf() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("report.pdf");
        std::fs::write(&src, b"%PDF-1.4 fake content").unwrap();
        let dest_dir = dir.path().join("attachments").join("batch1");
        std::fs::create_dir_all(&dest_dir).unwrap();
        let dest = dest_dir.join("report_copy.pdf");
        std::fs::copy(&src, &dest).unwrap();
        assert_eq!(std::fs::read(&dest).unwrap(), b"%PDF-1.4 fake content");
    }
}
