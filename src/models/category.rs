// Category model per data-model.md Commerce Domain entity
// Fields: id, parent_category_id, name, slug, description, status, created_at, updated_at
// Self-referencing hierarchical structure: parent_category_id → commerce.categories(id)

// Category status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CategoryStatus {
    Active,
    Inactive,
}

/// Category platform entity
/// Owned by: Commerce Domain (constitution.md §127-137)
/// Responsible for: Product categorization, hierarchical organization (constitution.md §127-137)
/// Relationships:
///   - SELF-REFERENCING: parent_category_id references commerce.categories(id)
///   - ONE-TO-MANY: inverse of parent_category_id (a category may have sub-categories)

/// Category platform entity
/// Fields: id, parent_category_id, name, slug, description, status, created_at, updated_at
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Category {
    /// Unique identifier
    pub id: Uuid,
    /// Parent category ID (nullable, self-referencing)
    pub parent_category_id: Option<Uuid>,
    /// Category name
    pub name: String,
    /// URL-friendly slug (unique per commerce context)
    pub slug: String,
    /// Category description
    pub description: String,
    /// Category status
    pub status: CategoryStatus,
    /// When the category was created
    pub created_at: time::OffsetDateTime,
    /// When the category was last updated
    pub updated_at: time::OffsetDateTime,
}

/// Category status enum helper
impl Category {
    /// Check if category is active
    pub fn is_active(&self) -> bool {
        matches!(self.status, CategoryStatus::Active)
    }

    /// Check if category is inactive
    pub fn is_inactive(&self) -> bool {
        matches!(self.status, CategoryStatus::Inactive)
    }

    /// Check if category has parent (hierarchical level)
    pub fn has_parent(&self) -> bool {
        self.parent_category_id.is_some()
    }

    /// Check if category has sub-categories (children)
    pub fn has_children(&self, children: &[Category]) -> bool {
        children.iter().any(|c| c.parent_category_id == Some(self.id))
    }

    /// Convert Category to API response DTO
    pub fn to_dto(&self) -> CategoryDto {
        CategoryDto {
            id: self.id,
            name: self.name.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            status: self.status,
            has_parent: self.has_parent(),
        }
    }
}

/// Category DTO for API responses
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CategoryDto {
        /// Unique identifier
        pub id: Uuid,
        /// Category name
        pub name: String,
        /// Category slug
        pub slug: String,
        /// Category description
        pub description: String,
        /// Category status
        pub status: CategoryStatus,
        /// Whether category has a parent category
        pub has_parent: bool,
    }
}

/// Category creation helper
impl Category {
    /// Create a new category with Active status
    pub fn new(name: String, slug: String, description: String) -> Self {
        let now = time::OffsetDateTime::now_local()
            .expect("Failed to get current time");
        Category {
            id: uuid::Uuid::new_v4(),
            parent_category_id: None,
            name,
            slug,
            description,
            status: CategoryStatus::Active,
            created_at: now,
            updated_at: now,
        }
    }

    /// Set parent category (hierarchical level)
    pub fn with_parent(mut self, parent_id: Uuid) -> Self {
        self.parent_category_id = Some(parent_id);
        self
    }
}