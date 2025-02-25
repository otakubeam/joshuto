use std::fs::File;
use std::path;
use std::time::SystemTime;

use filetime::FileTime;

use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::history::create_dirlist_with_history;

fn _update_actime(file: &path::Path) -> std::io::Result<()> {
    let file_time = FileTime::from_system_time(SystemTime::now());
    filetime::set_file_times(file, file_time, file_time)
}

fn _create_file(file: &path::Path) -> std::io::Result<()> {
    File::create(file)?;
    Ok(())
}

pub fn touch_file(context: &mut AppContext, arg: &str) -> JoshutoResult<()> {
    match arg {
        "" => {
            match context
                .tab_context_ref()
                .curr_tab_ref()
                .curr_list_ref()
                .and_then(|s| s.curr_entry_ref())
                .map(|s| s.file_path().to_path_buf())
            {
                Some(selected_file_path) => _update_actime(&selected_file_path)?,
                None => {}
            }
        }
        file_arg => {
            let file = path::PathBuf::from(file_arg);
            if file.exists() {
                _update_actime(file.as_path())?;
            } else {
                _create_file(file.as_path())?;
            }
        }
    }

    let path = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .map(|lst| lst.file_path().to_path_buf());

    if let Some(path) = path {
        let options = context.config_ref().display_options_ref().clone();

        let history = context.tab_context_mut().curr_tab_mut().history_mut();
        let new_dirlist = create_dirlist_with_history(history, path.as_path(), &options)?;
        history.insert(path, new_dirlist);
    }
    Ok(())
}
