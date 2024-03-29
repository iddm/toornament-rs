use std::collections::BTreeSet;

/// Unique permission identity
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct PermissionId(pub String);

/// Permission attribute definition
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PermissionAttribute {
    /// Edit permission
    Edit,
    /// Delete permission
    Delete,
    /// Authorize permission
    Authorize,
    /// Report permission
    Report,
    /// Fill permission
    Fill,
    /// Place permission
    Place,
    /// Register permission
    Register,
}

/// A list of permission attributes
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct PermissionAttributes(pub BTreeSet<PermissionAttribute>);

/// A user permission
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct Permission {
    /// The permission identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<PermissionId>,
    /// Email of the permitted user.
    pub email: String,
    /// The list of permission of the related user on the tournament.
    pub attributes: PermissionAttributes,
}
impl Permission {
    /// Create permission object for adding it to a tournament
    /// (Toornament::create_tournament_permission)
    pub fn create<S: Into<String>>(email: S, attributes: PermissionAttributes) -> Permission {
        Permission {
            id: None,
            email: email.into(),
            attributes,
        }
    }
}

/// A list of permissions
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct Permissions(pub Vec<Permission>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_attributes_parse() {
        let s = r#"
        [
            "report",
            "place",
            "register",
            "edit",
            "authorize",
            "fill",
            "delete"
        ]
        "#;

        let ps: PermissionAttributes = serde_json::from_str(s).unwrap();
        assert_eq!(ps.0.len(), 7);
        assert!(ps.0.iter().any(|p| *p == PermissionAttribute::Edit));
        assert!(ps.0.iter().any(|p| *p == PermissionAttribute::Report));
        assert!(ps.0.iter().any(|p| *p == PermissionAttribute::Place));
        assert!(ps.0.iter().any(|p| *p == PermissionAttribute::Register));
        assert!(ps.0.iter().any(|p| *p == PermissionAttribute::Authorize));
        assert!(ps.0.iter().any(|p| *p == PermissionAttribute::Fill));
        assert!(ps.0.iter().any(|p| *p == PermissionAttribute::Delete));
    }
}
