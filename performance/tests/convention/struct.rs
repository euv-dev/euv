use super::*;

#[derive(Clone, Debug, Default, Getter, GetterMut)]
pub(super) struct FieldAccessAudit {
    #[get(pub(super))]
    #[get_mut(pub(super))]
    pub(super) violations: Vec<String>,
}
