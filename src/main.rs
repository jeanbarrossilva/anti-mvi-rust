mod core;
mod feature;

use core::to_do::ToDo;

use egui::{containers::Window, Context};
use feature::to_dos::view::{to_do_list::to_do_list, to_do_text_edit::to_do_text_edit};

fn main() {
  let context = Context::default();
  let to_dos: Vec<ToDo> = vec![];
  Window::new("To-dos").show(&context, |ui| {
    ui.vertical(|ui| {
      to_do_list(ui, &to_dos);
      ui.separator();
      to_do_text_edit(ui, |_text| {});
    })
  });
}
