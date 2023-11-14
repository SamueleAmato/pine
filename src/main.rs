use clap::{command, Arg, ArgMatches};
use std::path::PathBuf;
use std::env;
use std::process::Command;
use std::{fs, io};

mod todo_manager;

/*
Created by: SamueleAmato

Version: 0.1.0
*/

fn main() {
    let home_dir = todo_manager::get_home_directory();
    let editor = match todo_manager::get_editor_from_json(&home_dir) {
        Ok(editor) => editor,
        Err(err) => {
            eprintln!("Errore: {}", err);
            // Puoi fornire un valore di fallback o gestire l'errore in modo diverso qui, se necessario
            String::from("valore_di_default")
        }
    };

    let argomenti: ArgMatches = command!()
        .about("Application description")
        .arg( // Add notebook or note
            Arg::new("add")
                .long_help("- <ITEM_TYPE>: Specify the type of item to add, which can be \x1b[33mnotebook\x1b[0m or \x1b[33mnote\x1b[0m.")
                .long("add")
                .value_names(["ITEM TYPE"])
                .visible_aliases(["new", "put", "create"])
        )
        .arg( // Remove notebook or note
            Arg::new("remove")
                .long_help("- <ITEM_TYPE>: Specify the type of item to remove, which can be \x1b[33mnotebook\x1b[0m or \x1b[33mnote\x1b[0m.")
                .short('r')
                .long("remove")
                .value_names(["ITEM TYPE"])
                .visible_aliases(["rm", "delate"])
        )
        .arg( // Choice the name of the item [ REQUIRED UNLESS ]
            Arg::new("name")
                .long_help("- <ITEM_NAME>: Specify the \x1b[33mname\x1b[0m of the item.")
                .short('n')
                .long("name")
                .value_name("ITEM_NAME")
                .visible_aliases(["title", "identifier"])
                .required_unless_present_any(["list", "open", "seteditor"])
            )
        .arg( // List all notebook/note example: list notebook | list notebook/notebook_name
            Arg::new("list")
                .long_help("- <TARGET>: Specify the item to be listed. It can take on values like \x1b[33mnotebook\x1b[0m to list all notebooks,\n\x1b[33mnote\x1b[0m to list all notes, and \x1b[33mnotebook/notebook_name\x1b[0m to list all notes created within a specific notebook.")
                .short('l')
                .long("list")
                .value_name("TARGET")
                .visible_aliases(["ls", "show"])
            )
        .arg( // Add a new note to a specific notebook
        	Arg::new("is_in_notebook")
        		.long_help("- <TARGET-NOTEBOOK>: Specify the notebook were the note will be saved.")
        		.short('i')
        		.long("to-notebook")
        		.value_name("TARGET-NOTEBOOK")
        		.visible_aliases(["in-notebook", "to_notebook", "innotebook", "tonotebook", "notebook"])
        	)
        .arg( // Move note to a specific notebook
        	Arg::new("moveto")
        		.long_help("- <DESTINATION NOTEBOOK>: Specify the notebook were the note will be moved.")
        		.short('m')
        		.long("moveto")
        		.value_name("DESTINATION NOTEBOOK")
        		.visible_aliases(["mv", "move"])
            )
		/*.arg( // Add task to a todo list
                Arg::new("todo")
                    .long_help("Adds a new task to the to-do list with the specified content, \n- <CONTENT>: name of the activity")
                    .short('t')
                    .long("todo")
                   	.value_name("CONTENT")
                    .visible_aliases(["do", "task", "td"])
                    
            )*/
        .arg( // Check/Uncheck task in todo list
                Arg::new("check")
                    .long_help("Check or uncheck a task in the to-do list.\n\
                    \x1b[33mOPTION:\x1b[0m It can only be \x1b[33myes/y\x1b[0m (check) or \x1b[33mno/n\x1b[0m (uncheck)."
            )
                    .short('c')
                    .long("check")
                    .value_name("OPTION")
                    .visible_aliases(["ch", "completed"])
            )
        .arg( // Change editor
                Arg::new("seteditor")
                    .long_help("Change the text editor used to open note.\n- <EDITOR_NAME>: The name of the file editor.")
                    .long("seteditor")
                    .value_name("EDITOR_NAME")
                    .visible_aliases(["st", "editorset"])
            )
        .arg( // Open file
                Arg::new("open")
                    .long_help("- <FILE NAME>: Specify the name of the file you want to open.")
                    .long("open")
                    .value_name("FILE NAME")
                    .visible_aliases(["edit", "op"])
            )

        .get_matches();
        
        // end of arg processing
        
    if argomenti.get_one::<String>("add").is_some() {
        if argomenti.get_one::<String>("add").unwrap() == "notebook" {
            let _ = fs::create_dir_all(format!("{home_dir}/.pine/notebook/{}", argomenti.get_one::<String>("name").unwrap()));
        } else if argomenti.get_one::<String>("add").unwrap() == "note" && argomenti.get_one::<String>("is_in_notebook").is_some(){
            let _ = todo_manager::open_file_with_editor(format!("{home_dir}/.pine/notebook/{}/{}", argomenti.get_one::<String>("is_in_notebook").unwrap(), argomenti.get_one::<String>("name").unwrap()), editor);
        } else {
            let _ = todo_manager::open_file_with_editor(format!("{home_dir}/.pine/note/{}", argomenti.get_one::<String>("name").unwrap()), editor);
        }
    } 

    else if argomenti.get_one::<String>("remove").is_some() {
    	if argomenti.get_one::<String>("remove").unwrap() == "notebook"{ // ad "are you sure"
            let _ = fs::remove_dir(format!("{home_dir}/.pine/notebook/{}", argomenti.get_one::<String>("name").unwrap()));
		} else if argomenti.get_one::<String>("remove").unwrap() == "note" && argomenti.get_one::<String>("is_in_notebook").is_some() {
            let _ = fs::remove_file(format!("{home_dir}/.pine/notebook/{}/{}", argomenti.get_one::<String>("is_in_notebook").unwrap(), argomenti.get_one::<String>("name").unwrap()));
        } else {
        	if argomenti.get_one::<String>("name").is_some() {
            	let _ = fs::remove_file(format!("{home_dir}/.pine/note/{}", argomenti.get_one::<String>("name").unwrap()));
            }
        }
    }

    else if argomenti.get_one::<String>("moveto").is_some() {
    	if argomenti.get_one::<String>("is_in_notebook").is_some() {
    		let _ = fs::copy(format!("{home_dir}/.pine/notebook/{}/{}", argomenti.get_one::<String>("is_in_notebook").unwrap(), argomenti.get_one::<String>("name").unwrap()), format!("{home_dir}/.pine/notebook/{}/{}", argomenti.get_one::<String>("moveto").unwrap(), argomenti.get_one::<String>("name").unwrap()));
			let _ = fs::remove_file(format!("{home_dir}/.pine/notebook/{}/{}", argomenti.get_one::<String>("is_in_notebook").unwrap(), argomenti.get_one::<String>("name").unwrap()));
    	} else {
      		let _ = fs::copy(format!("{home_dir}/.pine/note/{}", argomenti.get_one::<String>("name").unwrap()), format!("{home_dir}/.pine/notebook/{}/{}", argomenti.get_one::<String>("moveto").unwrap(), argomenti.get_one::<String>("name").unwrap()));
			let _ = fs::remove_file(format!("{home_dir}/.pine/note/{}", argomenti.get_one::<String>("name").unwrap()));

    	}
    }

    else if argomenti.get_one::<String>("seteditor").is_some() {
        let _ = todo_manager::save_editor_to_json(argomenti.get_one::<String>("seteditor").unwrap(), &home_dir);
    }

    else if argomenti.get_one::<String>("list").is_some() {
        if argomenti.get_one::<String>("is_in_notebook").is_some() {
            match todo_manager::list(format!("{}/.pine/notebook/{}", home_dir, argomenti.get_one::<String>("is_in_notebook").unwrap())) {
                Ok(entries) => {
                    for entry in entries {
                        println!("{}", entry.display());
                    }
                }
                Err(err) => {
                    eprintln!("Errore durante la lista delle directory/file: {}", err);
                }
            }
        } else if argomenti.get_one::<String>("list").unwrap() == &"notebook".to_string() {
            match todo_manager::list(format!("{}/.pine/notebook", home_dir)) {
                Ok(entries) => {
                    for entry in entries {
                        println!("{}", entry.display());
                    }
                }
                Err(err) => {
                    eprintln!("Errore durante la lista delle directory/file: {}", err);
                }
            }

        } else {
            match todo_manager::list(format!("{}/.pine/note", home_dir)) {
                Ok(entries) => {
                    for entry in entries {
                        println!("{}", entry.display());
                    }
                }
                Err(err) => {
                    eprintln!("Errore durante la lista delle directory/file: {}", err);
                }
            }
        }
    }
    else if argomenti.get_one::<String>("remove").is_some() {
        if argomenti.get_one::<String>("remove").unwrap() == &"note".to_string() {
            let _ = fs::remove_file(format!("{}/.pine/note/{}", home_dir, argomenti.get_one::<String>("name").unwrap()));
        }
        else if argomenti.get_one::<String>("remove").unwrap() == &"notebook".to_string() {
            let _ = fs::remove_file(format!("{}/.pine/notebook/{}", home_dir, argomenti.get_one::<String>("is_in_notebook").unwrap()));
        }
        else if argomenti.get_one::<String>("remove").unwrap() == &"note".to_string() && argomenti.get_one::<String>("is_in_notebook").is_some() {
            let _ = fs::remove_file(format!("{}/.pine/notebook/{}/{}", home_dir, argomenti.get_one::<String>("is_in_notebook").unwrap(), argomenti.get_one::<String>("name").unwrap()));
        }
    }
    /*
    else if argomenti.get_one::<String>("todo").is_some() {
		if argomenti.get_one::<String>("todo").unwrap() == &"ls".to_string() || argomenti.get_one::<String>("todo").unwrap() == &"list".to_string() {
			let _ = todo_manager::list_tasks();
		} else if argomenti.get_one::<String>("todo").is_some() && argomenti.get_one::<String>("remove").is_some() {
			let _ = todo_manager::delete_task(*argomenti.get_one::<usize>("remove").unwrap());
		}
    	else {
    		let _ = todo_manager::add_task(argomenti.get_one::<String>("todo").unwrap());
    	}
    }*/

    else if argomenti.get_one::<String>("open").is_some() {
        if argomenti.get_one::<String>("is_in_notebook").is_some() {
            let _ = todo_manager::open_file_with_editor(format!("{home_dir}/.pine/notebook/{}/{}", argomenti.get_one::<String>("is_in_notebook").unwrap(), argomenti.get_one::<String>("open").unwrap()), editor);
        } else {
            let _ = todo_manager::open_file_with_editor(format!("{home_dir}/.pine/note/{}", argomenti.get_one::<String>("open").unwrap()), editor);
        }
    }

}
