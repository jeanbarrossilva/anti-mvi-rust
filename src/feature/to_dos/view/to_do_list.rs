use egui::Ui;

use crate::core::to_do::ToDo;

pub fn to_do_list(ui: &mut Ui, to_dos: &Vec<ToDo>) {
  ui.vertical(|ui| {
    for to_do in to_dos {
      ui.label(to_do.title);
    }
  });
}
