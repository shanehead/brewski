use crate::error::AppError;
use crate::models::Recipe;
use crate::repositories::recipe::RecipeRepository;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};

pub(crate) fn delete_image_file(path: &Path) -> Result<(), AppError> {
    match std::fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(AppError::Internal(format!("remove image: {e}"))),
    }
}

pub fn images_dir(app: &AppHandle) -> Result<PathBuf, AppError> {
    let base = app
        .path()
        .app_data_dir()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(base.join("images"))
}

pub fn image_path(app: &AppHandle, recipe_id: &str) -> Result<PathBuf, AppError> {
    Ok(images_dir(app)?.join(format!("{recipe_id}.jpg")))
}

pub fn write_image(src: &Path, dest: &Path) -> Result<(), AppError> {
    let img = image::open(src).map_err(|e| AppError::Internal(format!("image decode: {e}")))?;
    let resized = resize_to_fit(img, 1200);
    std::fs::create_dir_all(dest.parent().unwrap())
        .map_err(|e| AppError::Internal(format!("create images dir: {e}")))?;
    let file = std::fs::File::create(dest)
        .map_err(|e| AppError::Internal(format!("create image file: {e}")))?;
    let mut enc =
        image::codecs::jpeg::JpegEncoder::new_with_quality(std::io::BufWriter::new(file), 85);
    enc.encode_image(&resized)
        .map_err(|e| AppError::Internal(format!("image encode: {e}")))?;
    Ok(())
}

fn resize_to_fit(img: image::DynamicImage, max_px: u32) -> image::DynamicImage {
    let (w, h) = (img.width(), img.height());
    if w <= max_px && h <= max_px {
        return img;
    }
    let scale = max_px as f32 / w.max(h) as f32;
    let new_w = (w as f32 * scale) as u32;
    let new_h = (h as f32 * scale) as u32;
    img.resize(new_w, new_h, image::imageops::FilterType::Lanczos3)
}

#[tauri::command]
pub async fn upload_recipe_image(
    app: AppHandle,
    state: tauri::State<'_, crate::AppState>,
    recipe_id: String,
    source_path: String,
) -> Result<Recipe, AppError> {
    let dest = image_path(&app, &recipe_id)?;
    write_image(Path::new(&source_path), &dest)?;
    let filename = format!("{recipe_id}.jpg");
    RecipeRepository::new(&state.db)
        .set_image_path(&recipe_id, Some(&filename))
        .await
}

#[tauri::command]
pub async fn delete_recipe_image(
    app: AppHandle,
    state: tauri::State<'_, crate::AppState>,
    recipe_id: String,
) -> Result<Recipe, AppError> {
    let path = image_path(&app, &recipe_id)?;
    delete_image_file(&path)?;
    RecipeRepository::new(&state.db)
        .set_image_path(&recipe_id, None)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb};
    use tempfile::tempdir;

    fn make_test_image(path: &Path, width: u32, height: u32) {
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
            ImageBuffer::from_fn(width, height, |x, _y| Rgb([x as u8, 100, 200]));
        img.save(path).unwrap();
    }

    #[test]
    fn test_resize_to_fit_large_image() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("src.png");
        let dest = dir.path().join("images/out.jpg");
        make_test_image(&src, 3000, 2000);
        write_image(&src, &dest).unwrap();
        let result = image::open(&dest).unwrap();
        assert!(result.width() <= 1200);
        assert!(result.height() <= 1200);
    }

    #[test]
    fn test_resize_to_fit_small_image_unchanged() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("src.png");
        let dest = dir.path().join("images/out.jpg");
        make_test_image(&src, 800, 600);
        write_image(&src, &dest).unwrap();
        let result = image::open(&dest).unwrap();
        assert_eq!(result.width(), 800);
        assert_eq!(result.height(), 600);
    }

    #[test]
    fn test_write_image_creates_dir() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("src.png");
        let dest = dir.path().join("nested/dir/out.jpg");
        make_test_image(&src, 100, 100);
        write_image(&src, &dest).unwrap();
        assert!(dest.exists());
    }

    #[test]
    fn test_delete_image_file_nonexistent_returns_ok() {
        let dir = tempdir().unwrap();
        let nonexistent = dir.path().join("images").join("no-such-recipe.jpg");
        assert!(!nonexistent.exists());
        // Calling delete_image_file on a missing path must not return an error.
        delete_image_file(&nonexistent).unwrap();
    }

    #[test]
    fn test_delete_image_file_existing_deletes_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("recipe.jpg");
        make_test_image(&path, 100, 100);
        assert!(path.exists());
        delete_image_file(&path).unwrap();
        assert!(!path.exists());
    }
}
