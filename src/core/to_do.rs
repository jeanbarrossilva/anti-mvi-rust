use uuid::Uuid;

/// User-defined task intended to be done.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToDo<'t> {
  /// Unique identifier.
  pub id: Uuid,

  /// Description of what is to be done.
  pub title: &'t str,

  /// Whether the task has been completed.
  pub is_done: bool
}
