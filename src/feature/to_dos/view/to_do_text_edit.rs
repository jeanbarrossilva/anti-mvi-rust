use egui::Ui;

pub fn to_do_text_edit(ui: &mut Ui, on_submission: fn(text: &str)) {
  let mut text = "";
  let to_do_field = ui.text_edit_singleline(&mut text);
  if to_do_field.lost_focus() {
    on_submission(text);
  }
}
