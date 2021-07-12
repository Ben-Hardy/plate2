
mod utils;
mod c_commands;
mod web_commands;

fn warn() {
    println!("You didn\'t provide any file names!");
}

pub fn process_c_command(filenames: Vec<String>) {
    if filenames.len() == 0 {
        warn();
    } else {
        c_commands::process_c_command(filenames);
    }
}

pub fn process_h_command(filenames: Vec<String>) {
    if filenames.len() == 0 {
        warn();
    } else {
        c_commands::process_h_command(filenames);
    }
}

// consider this one more
pub fn process_ch_command(filenames: Vec<String>) {
    if filenames.len() == 0 {
        warn();
    } else {
        c_commands::process_ch_command(filenames);
    }
}

pub fn process_cpp_command(filenames: Vec<String>) {
    if filenames.len() == 0 {
        warn();
    } else {
        c_commands::process_cpp_command(filenames);
    }
}

pub fn process_cpph_command(filenames: Vec<String>) {
    if filenames.len() == 0 {
        warn();
    } else {
        c_commands::process_cpph_command(filenames);
    }
}

pub fn process_cproj_command(filenames: Vec<String>) {
    if filenames.len() == 0 {
        warn();
    } else {
        c_commands::process_cproj_command(filenames);
    }
}

pub fn process_cppproj_command(filenames: Vec<String>) {
    if filenames.len() == 0 {
        warn();
    } else {
        c_commands::process_cppproj_command(filenames);
    }
}

pub fn process_html_command(filenames: Vec<String>) {
    if filenames.len() == 0 {
        warn();
    } else {
        web_commands::process_html_command(filenames);

    }
}

pub fn process_website_command(filenames: Vec<String>) {
    if filenames.len() == 0 {
        warn();
    } else {
        web_commands::process_website_command(filenames);
    }
}