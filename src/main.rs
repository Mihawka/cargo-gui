use gtk::{
    prelude::BuilderExtManual, Button, ButtonExt, Entry, FileChooser, Inhibit, Switch, WidgetExt,
    Window,
};

use crate::data::project::Project;

mod data;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to load GTK");
        return;
    }

    let glade_src = include_str!("./Basic UI.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let window: Window = builder.get_object("applicationGtk").unwrap();
    let button_create_project: Button = builder.get_object("button-project-create").unwrap();

    let window_project: Window = builder.get_object("window-project-create").unwrap();
    let projectwindow_button_create: Button =
        builder.get_object("projectwindow-button-create").unwrap();
    let projectwindow_button_cancel: Button =
        builder.get_object("projectwindow-button-cancel").unwrap();
    let projectwindow_entry_name: Entry = builder.get_object("projectwindow-entry-name").unwrap();
    let projectwindow_folderchooser_dir: FileChooser = builder
        .get_object("projectwindow-folderchooser-dir")
        .unwrap();
    let projectwindow_switch_lib: Switch = builder.get_object("projectwindow-switch-lib").unwrap();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // BUTTON HOME WINDOW - Create Project
    let window_clone = window.clone();
    let window_project_clone = window_project.clone();
    button_create_project.connect_clicked(move |_| {
        window_clone.hide();
        window_project_clone.show();
        let data = Project {
            name: projectwindow_entry_name.to_string(),
            directory: projectwindow_folderchooser_dir.to_string(),
            is_lib: projectwindow_switch_lib.activate(),
        };
        data.create_project();
    });

    // BUTTON PROJECT WINDOW - Create
    let window_clone = window.clone();
    let window_project_clone = window_project.clone();
    projectwindow_button_create.connect_clicked(move |_| {
        window_clone.show();
        window_project_clone.hide();
    });

    // BUTTON PROJECT WINDOW - Cancel
    let window_clone = window.clone();
    let window_project_clone = window_project.clone();
    projectwindow_button_cancel.connect_clicked(move |_| {
        window_clone.show();
        window_project_clone.hide();
    });

    window.show_all();
    gtk::main();
}
