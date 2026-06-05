use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};

use crate::entities::batch_attachments;
use crate::error::AppError;
use crate::models::BatchAttachment;

use super::{new_id, now_secs};

pub struct BatchAttachmentRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> BatchAttachmentRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        batch_id: &str,
        filename: &str,
        original_name: &str,
        mime_type: Option<&str>,
        size_bytes: i64,
    ) -> Result<BatchAttachment, AppError> {
        let id = new_id();
        let now = now_secs() as i32;
        batch_attachments::ActiveModel {
            id: Set(id.clone()),
            batch_id: Set(batch_id.to_string()),
            filename: Set(filename.to_string()),
            original_name: Set(original_name.to_string()),
            mime_type: Set(mime_type.map(|s| s.to_string())),
            size_bytes: Set(size_bytes),
            created_at: Set(now),
        }
        .insert(self.db)
        .await?;
        self.get(&id).await
    }

    pub async fn list(&self, batch_id: &str) -> Result<Vec<BatchAttachment>, AppError> {
        let rows = batch_attachments::Entity::find()
            .filter(batch_attachments::Column::BatchId.eq(batch_id))
            .order_by_asc(batch_attachments::Column::CreatedAt)
            .all(self.db)
            .await?;
        Ok(rows.into_iter().map(Self::to_model).collect())
    }

    pub async fn get(&self, id: &str) -> Result<BatchAttachment, AppError> {
        batch_attachments::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .map(Self::to_model)
            .ok_or_else(|| AppError::Internal(format!("attachment {id} not found")))
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        batch_attachments::Entity::delete_by_id(id)
            .exec(self.db)
            .await?;
        Ok(())
    }

    fn to_model(m: batch_attachments::Model) -> BatchAttachment {
        BatchAttachment {
            id: m.id,
            batch_id: m.batch_id,
            filename: m.filename,
            original_name: m.original_name,
            mime_type: m.mime_type,
            size_bytes: m.size_bytes,
            created_at: m.created_at as i64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{CreateBatchInput, CreateRecipeInput};
    use crate::repositories::batches::BatchRepository;
    use crate::repositories::recipe::RecipeRepository;
    use crate::test_helpers::setup_test_db;

    async fn setup(db: &DatabaseConnection) -> String {
        let recipe = RecipeRepository::new(db)
            .create(CreateRecipeInput {
                name: "Test IPA".into(),
                ..Default::default()
            })
            .await
            .unwrap();
        let batch = BatchRepository::new(db)
            .create(CreateBatchInput {
                recipe_id: recipe.id,
                name: None,
                version_id: None,
            })
            .await
            .unwrap();
        batch.id
    }

    #[tokio::test]
    async fn test_create_and_get() {
        let db = setup_test_db().await;
        let batch_id = setup(&db).await;
        let repo = BatchAttachmentRepository::new(&db);
        let att = repo
            .create(
                &batch_id,
                "abc.jpg",
                "brew-day.jpg",
                Some("image/jpeg"),
                102400,
            )
            .await
            .unwrap();
        assert_eq!(att.batch_id, batch_id);
        assert_eq!(att.filename, "abc.jpg");
        assert_eq!(att.original_name, "brew-day.jpg");
        assert_eq!(att.mime_type.as_deref(), Some("image/jpeg"));
        assert_eq!(att.size_bytes, 102400);
    }

    #[tokio::test]
    async fn test_list_ordered_by_created_at() {
        let db = setup_test_db().await;
        let batch_id = setup(&db).await;
        let repo = BatchAttachmentRepository::new(&db);
        repo.create(&batch_id, "a.pdf", "a.pdf", Some("application/pdf"), 1000)
            .await
            .unwrap();
        repo.create(&batch_id, "b.jpg", "b.jpg", Some("image/jpeg"), 2000)
            .await
            .unwrap();
        let list = repo.list(&batch_id).await.unwrap();
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].filename, "a.pdf");
        assert_eq!(list[1].filename, "b.jpg");
    }

    #[tokio::test]
    async fn test_delete() {
        let db = setup_test_db().await;
        let batch_id = setup(&db).await;
        let repo = BatchAttachmentRepository::new(&db);
        let att = repo
            .create(&batch_id, "x.pdf", "x.pdf", None, 500)
            .await
            .unwrap();
        repo.delete(&att.id).await.unwrap();
        let list = repo.list(&batch_id).await.unwrap();
        assert!(list.is_empty());
    }

    #[tokio::test]
    async fn test_delete_batch_cascades() {
        let db = setup_test_db().await;
        let batch_id = setup(&db).await;
        let repo = BatchAttachmentRepository::new(&db);
        repo.create(&batch_id, "x.pdf", "x.pdf", None, 500)
            .await
            .unwrap();
        BatchRepository::new(&db).delete(&batch_id).await.unwrap();
        let list = repo.list(&batch_id).await.unwrap();
        assert!(list.is_empty());
    }

    #[tokio::test]
    async fn test_get_nonexistent_returns_error() {
        let db = setup_test_db().await;
        let repo = BatchAttachmentRepository::new(&db);
        let result = repo.get("no-such-id").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_null_mime_type() {
        let db = setup_test_db().await;
        let batch_id = setup(&db).await;
        let repo = BatchAttachmentRepository::new(&db);
        let att = repo
            .create(&batch_id, "unknown.bin", "unknown.bin", None, 100)
            .await
            .unwrap();
        assert!(att.mime_type.is_none());
    }
}
