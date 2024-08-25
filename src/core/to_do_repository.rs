use reactive_rs::SimpleBroadcast;
use uuid::Uuid;

use crate::core::to_do::ToDo;

/// Repository by which read and write operations on to-dos are performed.
pub trait ToDoRepository<'r> {
  /// Obtains a stream by which all to-dos are sent.
  fn _get(&self) -> &SimpleBroadcast<'r, Vec<ToDo<'r>>>;

  /// Adds the to-do.
  async fn _add(&mut self, to_do: ToDo<'r>);

  /// Removes the to-do identified with the given identifier.
  async fn _remove(&mut self, id: Uuid);
}
