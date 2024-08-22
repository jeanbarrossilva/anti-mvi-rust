use reactive_rs::SimpleBroadcast;
use uuid::Uuid;

use crate::core::{to_do::ToDo, to_do_repository::ToDoRepository};

/// A to-do repository which writes into and reads from memory.
pub struct InMemoryToDoRepository<'r> {
  /// Broadcast by which the updated to-dos are emitted.
  broadcast: SimpleBroadcast<'r, Vec<ToDo<'r>>>,

  /// Currently added to-dos.
  to_dos: Vec<ToDo<'r>>
}

impl<'r> InMemoryToDoRepository<'r> {
  /// Creates an in-memory repository for to-dos.
  pub fn new() -> InMemoryToDoRepository<'r> {
    return Self {
      broadcast: SimpleBroadcast::new(),
      to_dos: vec![]
    };
  }
}

impl<'r> ToDoRepository<'r> for InMemoryToDoRepository<'r> {
  fn get(&self) -> &SimpleBroadcast<'r, Vec<ToDo<'r>>> {
    return &self.broadcast;
  }

  async fn add(&mut self, to_do: ToDo<'r>) {
    self.to_dos.push(to_do);
    self.broadcast.send(self.to_dos.clone());
  }

  async fn remove(&mut self, id: Uuid) {
    self.to_dos.retain(|to_do| to_do.id != id);
    self.broadcast.send(self.to_dos.clone());
  }
}

#[cfg(test)]
mod tests {
  use std::{
    cell::RefCell,
    future::{Future, IntoFuture},
    rc::Rc
  };

  use reactive_rs::{SimpleBroadcast, Stream};
  use uuid::Uuid;

  use crate::core::{
    in_memory::in_memory_to_do_repository::InMemoryToDoRepository,
    to_do::ToDo,
    to_do_repository::ToDoRepository
  };

  #[tokio::test]
  async fn is_initially_empty<'a>() {
    let got_to_dos = get(InMemoryToDoRepository::new().broadcast.clone(), || {
      async {}.into_future()
    })
    .await;
    assert_eq!(Vec::<ToDo<'a>>::new(), got_to_dos);
  }

  #[tokio::test]
  async fn gets<'a>() {
    let mut repository = InMemoryToDoRepository::new();
    let to_do = ToDo {
      id: Uuid::new_v4(),
      title: "Study",
      is_done: false
    };
    let got_to_dos = get(repository.broadcast.clone(), || {
      repository.add(to_do.clone())
    })
    .await;
    assert_eq!(vec![to_do], got_to_dos);
  }

  #[tokio::test]
  async fn adds<'a>() {
    let mut repository = InMemoryToDoRepository::new();
    let added_to_do = ToDo {
      id: Uuid::new_v4(),
      title: "Clean room",
      is_done: false
    };
    let got_to_dos = get(repository.broadcast.clone(), || {
      repository.add(added_to_do.clone())
    })
    .await;
    assert_eq!(vec![added_to_do], got_to_dos);
  }

  #[tokio::test]
  async fn removes<'a>() {
    let mut repository = InMemoryToDoRepository::new();
    let removed_to_do_id = Uuid::new_v4();
    repository
      .add(ToDo {
        id: removed_to_do_id,
        title: "Wash dishes",
        is_done: true
      })
      .await;
    let got_to_dos = get(repository.broadcast.clone(), || {
      repository.remove(removed_to_do_id)
    })
    .await;
    assert_eq!(Vec::<ToDo<'a>>::with_capacity(0), got_to_dos);
  }

  /// Executes the given action and then returns the to-dos that were broadcasted as a result.
  async fn get<'a, A, R>(broadcast: SimpleBroadcast<'a, Vec<ToDo<'a>>>, action: A) -> Vec<ToDo<'a>>
  where
    A: FnOnce() -> R,
    R: Future<Output = ()>
  {
    let to_dos = Rc::new(RefCell::new(Vec::<ToDo<'a>>::with_capacity(0)));
    replace_with_lastly_broadcasted_to_dos(broadcast, to_dos.clone());
    action().await;
    return to_dos.clone().take();
  }

  /// Attaches an observer to the broadcast that replaces destination's current to-dos by the lastly
  /// broadcasted ones.
  fn replace_with_lastly_broadcasted_to_dos<'a>(
    broadcast: SimpleBroadcast<'a, Vec<ToDo<'a>>>,
    destination: Rc<RefCell<Vec<ToDo<'a>>>>
  ) {
    let mut are_last_to_dos = true;
    broadcast.subscribe({
      let destination = destination.clone();
      move |to_dos| {
        if are_last_to_dos {
          destination.replace(to_dos.clone());
          are_last_to_dos = false;
        }
      }
    });
  }
}
