use gtk::{Button, ButtonExt, Entry, EntryExt, FileChooserExt, FileChooserWidget, Inhibit, Switch, SwitchExt, WidgetExt, Window, prelude::BuilderExtManual};

use crate::data::project::Project;

mod data;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to load GTK");
        return;
    }
    //====================================================================================================
    let glade_src = include_str!("./Basic UI.glade");
    let builder = gtk::Builder::from_string(glade_src);
    // MAIN WINDOW =======================================================================================
    let window: Window = builder.get_object("applicationGtk").unwrap();
    let button_create_project: Button = builder.get_object("button-project-create").unwrap();
    // WINDOW PROJECT ====================================================================================
    let window_project: Window = builder.get_object("window-project-create").unwrap();
    let projectwindow_button_create: Button =
        builder.get_object("projectwindow-button-create").unwrap();
    let projectwindow_button_cancel: Button =
        builder.get_object("projectwindow-button-cancel").unwrap();
    let projectwindow_entry_name: Entry = builder.get_object("projectwindow-entry-name").unwrap();
    let projectwindow_button_dir: Button = builder
        .get_object("projectwindow_button_dir")
        .unwrap();
    let projectwindow_switch_lib: Switch = builder.get_object("projectwindow-switch-lib").unwrap();
    let projectwindow_entry_dir: Entry = builder.get_object("projectwindow_entry_dir").unwrap();
    // WINDOW FILE SELECT ================================================================================
    let window_filechooser: Window = builder.get_object("window-filechooser").unwrap();
    let filewindow_chooser: FileChooserWidget = builder.get_object("filewindow-chooser").unwrap();
    let filewindow_quit: Button = builder.get_object("filewindow-button-quit").unwrap();
    let filewindow_open: Button = builder.get_object("filewindow-button-open").unwrap();
    //====================================================================================================

    // BUTTON HOME WINDOW - Create Project
    let window_clone = window.clone();
    let window_project_clone = window_project.clone();
    button_create_project.connect_clicked(move |_| {
        window_clone.hide();
        window_project_clone.show();
    });
    //====================================================================================================

    // BUTTON PROJECT WINDOW - Create
    let window_clone = window.clone();
    let window_project_clone = window_project.clone();
    let projectwindow_entry_dir_clone = projectwindow_entry_dir.clone();
    let projectwindow_switch_lib_clone = projectwindow_switch_lib.clone();
    let projectwindow_entry_name_clone = projectwindow_entry_name.clone();
    projectwindow_button_create.connect_clicked(move |_| {
        window_clone.show();
        window_project_clone.hide();
        let data = Project {
            name: projectwindow_entry_name_clone.to_string(),
            directory: projectwindow_entry_dir_clone.get_text().to_string().clone(),
            is_lib: projectwindow_switch_lib_clone.get_active(),
        };
        data.create_project();

        projectwindow_entry_name_clone.set_text("");
        projectwindow_entry_dir_clone.set_text("");
        projectwindow_switch_lib_clone.set_active(false);
    });

    // BUTTON PROJECT WINDOW - Cancel
    let window_clone = window.clone();
    let window_project_clone = window_project.clone();
    let projectwindow_entry_name_clone = projectwindow_entry_name.clone();
    let projectwindow_switch_lib_clone = projectwindow_switch_lib.clone();
    let projectwindow_entry_dir_clone = projectwindow_entry_dir.clone();
    projectwindow_button_cancel.connect_clicked(move |_| {
        window_clone.show();
        window_project_clone.hide();

        projectwindow_entry_name_clone.set_text("");
        projectwindow_entry_dir_clone.set_text("");
        projectwindow_switch_lib_clone.set_active(false);
    });

    // BUTTON PROJECT WINDOW - Directory
    let window_project_clone = window_project.clone();
    let window_filechooser_clone = window_filechooser.clone();
    projectwindow_button_dir.connect_clicked(move |_| {
        window_project_clone.hide();
        window_filechooser_clone.show();
    });
    //====================================================================================================


    // BUTTON FILE SELECT - Open
    let window_project_clone = window_project.clone();
    let window_filechooser_clone = window_filechooser.clone();
    filewindow_open.connect_clicked(move |_| {
        window_project_clone.show();
        window_filechooser_clone.hide();

        let dir = filewindow_chooser.get_filename().unwrap();
        projectwindow_entry_dir.set_text(dir.to_str().unwrap());
    });

    // BUTTON FILE SELECT - Cancel
    let window_project_clone = window_project.clone();
    let window_filechooser_clone = window_filechooser.clone();
    filewindow_quit.connect_clicked(move |_| {
        window_project_clone.show();
        window_filechooser_clone.hide();
    });
    //====================================================================================================

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}
