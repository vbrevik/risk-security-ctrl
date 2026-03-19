use std::str::FromStr;

use super::models::AuthUser;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Feature {
    Ontology,
    Compliance,
    Analysis,
    Reports,
    Auth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Read,
    Create,
    Update,
    Delete,
    Export,
    ManageUsers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Admin,
    RiskManager,
    Specialist,
    Viewer,
}

impl FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(Role::Admin),
            "risk_manager" | "riskmanager" => Ok(Role::RiskManager),
            "specialist" => Ok(Role::Specialist),
            "viewer" => Ok(Role::Viewer),
            other => Err(format!("Unknown role: {}", other)),
        }
    }
}

/// Static permission check. No allocations, no I/O.
/// Returns true if the given role is allowed to perform the action on the feature.
pub fn has_permission(role: &Role, feature: &Feature, action: &Action) -> bool {
    // Admin has all permissions
    if *role == Role::Admin {
        return true;
    }

    // All authenticated users can read all features
    if *action == Action::Read {
        return true;
    }

    match (role, feature, action) {
        // RiskManager permissions
        (Role::RiskManager, Feature::Compliance, Action::Create | Action::Update | Action::Delete) => true,
        (Role::RiskManager, Feature::Analysis, Action::Create | Action::Update | Action::Delete) => true,
        (Role::RiskManager, Feature::Ontology, Action::Create | Action::Update) => true,
        (Role::RiskManager, Feature::Reports, Action::Export) => true,

        // Specialist permissions
        (Role::Specialist, Feature::Compliance, Action::Create | Action::Update) => true,
        (Role::Specialist, Feature::Analysis, Action::Create | Action::Update) => true,
        (Role::Specialist, Feature::Reports, Action::Export) => true,

        // Everything else is denied (Viewer, and any unmatched combos)
        _ => false,
    }
}

pub trait HasPermission {
    fn has_permission(&self, feature: Feature, action: Action) -> bool;
}

impl HasPermission for AuthUser {
    fn has_permission(&self, feature: Feature, action: Action) -> bool {
        let role = self.role.parse::<Role>().unwrap_or_else(|_| {
            tracing::warn!(
                user_id = %self.id,
                role = %self.role,
                "Unrecognized role, defaulting to Viewer"
            );
            Role::Viewer
        });
        has_permission(&role, &feature, &action)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admin_has_all_permissions() {
        let features = [Feature::Ontology, Feature::Compliance, Feature::Analysis, Feature::Reports, Feature::Auth];
        let actions = [Action::Read, Action::Create, Action::Update, Action::Delete, Action::Export, Action::ManageUsers];
        for feature in &features {
            for action in &actions {
                assert!(has_permission(&Role::Admin, feature, action),
                    "Admin should have permission for {:?}:{:?}", feature, action);
            }
        }
    }

    #[test]
    fn risk_manager_full_compliance_access() {
        assert!(has_permission(&Role::RiskManager, &Feature::Compliance, &Action::Read));
        assert!(has_permission(&Role::RiskManager, &Feature::Compliance, &Action::Create));
        assert!(has_permission(&Role::RiskManager, &Feature::Compliance, &Action::Update));
        assert!(has_permission(&Role::RiskManager, &Feature::Compliance, &Action::Delete));
    }

    #[test]
    fn risk_manager_cannot_manage_users() {
        assert!(!has_permission(&Role::RiskManager, &Feature::Auth, &Action::ManageUsers));
    }

    #[test]
    fn specialist_can_create_compliance() {
        assert!(has_permission(&Role::Specialist, &Feature::Compliance, &Action::Create));
    }

    #[test]
    fn specialist_cannot_delete_compliance() {
        assert!(!has_permission(&Role::Specialist, &Feature::Compliance, &Action::Delete));
    }

    #[test]
    fn specialist_cannot_write_ontology() {
        assert!(!has_permission(&Role::Specialist, &Feature::Ontology, &Action::Create));
        assert!(!has_permission(&Role::Specialist, &Feature::Ontology, &Action::Update));
        assert!(!has_permission(&Role::Specialist, &Feature::Ontology, &Action::Delete));
    }

    #[test]
    fn viewer_can_read_all_features() {
        let features = [Feature::Ontology, Feature::Compliance, Feature::Analysis, Feature::Reports, Feature::Auth];
        for feature in &features {
            assert!(has_permission(&Role::Viewer, feature, &Action::Read),
                "Viewer should be able to read {:?}", feature);
        }
    }

    #[test]
    fn viewer_cannot_create_update_delete() {
        let features = [Feature::Ontology, Feature::Compliance, Feature::Analysis, Feature::Reports, Feature::Auth];
        for feature in &features {
            assert!(!has_permission(&Role::Viewer, feature, &Action::Create));
            assert!(!has_permission(&Role::Viewer, feature, &Action::Update));
            assert!(!has_permission(&Role::Viewer, feature, &Action::Delete));
        }
    }

    #[test]
    fn viewer_cannot_export_reports() {
        assert!(!has_permission(&Role::Viewer, &Feature::Reports, &Action::Export));
    }

    #[test]
    fn unknown_role_defaults_to_viewer() {
        let user = AuthUser {
            id: "1".to_string(),
            email: "test@test.com".to_string(),
            name: "Test".to_string(),
            role: "unknown_role".to_string(),
            session_id: "s1".to_string(),
        };
        // Should behave like viewer: can read, cannot write
        assert!(user.has_permission(Feature::Compliance, Action::Read));
        assert!(!user.has_permission(Feature::Compliance, Action::Create));
        assert!(!user.has_permission(Feature::Compliance, Action::Delete));
    }

    #[test]
    fn has_permission_trait_on_auth_user() {
        let admin = AuthUser {
            id: "1".to_string(),
            email: "admin@test.com".to_string(),
            name: "Admin".to_string(),
            role: "admin".to_string(),
            session_id: "s1".to_string(),
        };
        assert!(admin.has_permission(Feature::Auth, Action::ManageUsers));

        let viewer = AuthUser {
            id: "2".to_string(),
            email: "viewer@test.com".to_string(),
            name: "Viewer".to_string(),
            role: "viewer".to_string(),
            session_id: "s2".to_string(),
        };
        assert!(!viewer.has_permission(Feature::Compliance, Action::Delete));
    }

    #[test]
    fn role_from_str_parsing() {
        assert_eq!(Role::from_str("admin").unwrap(), Role::Admin);
        assert_eq!(Role::from_str("risk_manager").unwrap(), Role::RiskManager);
        assert_eq!(Role::from_str("riskmanager").unwrap(), Role::RiskManager);
        assert_eq!(Role::from_str("specialist").unwrap(), Role::Specialist);
        assert_eq!(Role::from_str("viewer").unwrap(), Role::Viewer);
        assert_eq!(Role::from_str("ADMIN").unwrap(), Role::Admin);
        assert!(Role::from_str("unknown").is_err());
    }
}
