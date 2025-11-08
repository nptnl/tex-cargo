// ...existing code...
use std::fs::{File, create_dir_all, read_to_string, write};
use std::io::Write;
use std::process::Command;

fn main () {
    println!("henlo");
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("usage: tex-cargo <new|build|run|light|dark|wide|thin> [name]");
        return;
    }
    let command: String = args[1].clone();

    // Section contents (only the inner content for replacement)
    let light_section =
r#"
\usepackage{xcolor}
\pagecolor[rgb]{1,1,1}
\color[rgb]{0,0,0}
"#;
    let dark_section = 
r#"
\usepackage{xcolor}
\pagecolor[rgb]{0,0,0}
\color[rgb]{1,1,1}
"#;
    let wide_section =
r#"
\usepackage{geometry}
\geometry{margin=0.75in}
\geometry{tmargin=0.75in}
\geometry{bmargin=1in}
"#;
    let thin_section =
r#"
\usepackage{geometry}
\geometry{margin=1.25in}
\geometry{tmargin=1.25in}
\geometry{bmargin=1.5in}
"#;
    let prev_section =
r#"
\usepackage[active,tightpage]{preview}
\renewcommand{\PreviewBorder}{0.75in}
\newcommand{\Newpage}{\end{preview}\begin{preview}}
"#;
    let nprev_section = "\n";

    match command.as_str() {
        "new" => {
            if args.len() < 3 {
                println!("please provide a project name");
                return;
            }
            let res = new(args[2].clone());
            match res {
                Ok(()) => println!("new LaTeX project '{}' created.", args[2].clone()),
                Err(e) => println!("error {} in tex-cargo.rs", e),
            };
        },
        "build" => {
            let cmd = Command::new("pdflatex").arg("./main.tex").status().expect("failed to pdflatex");
            println!("process complete with {cmd}");
        },
        "run" => {
            let cmd = Command::new("pdflatex").arg("./main.tex").status().expect("failed to pdflatex");
            println!("process complete with {cmd}");
            let cmd = Command::new("open").arg("./main.pdf").status().expect("failed to open PDF");
            println!("pdf opened with {cmd}");
        },
        "light" => {
            match replace_section("./lib.sty", "% --- LD START", "% --- LD END", light_section) {
                Ok(()) => println!("rise + shine! light mode activated"),
                Err(e) => println!("error updating lib.sty: {}", e),
            }
        },
        "dark" => {
            match replace_section("./lib.sty", "% --- LD START", "% --- LD END", dark_section) {
                Ok(()) => println!("dark mode activated, night night"),
                Err(e) => println!("error updating lib.sty: {}", e),
            }
        },
        "wide" => {
            match replace_section("./lib.sty", "% --- GEO START", "% --- GEO END", wide_section) {
                Ok(()) => println!("wide margins activated"),
                Err(e) => println!("error updating lib.sty: {}", e),
            }
        },
        "thin" => {
            match replace_section("./lib.sty", "% --- GEO START", "% --- GEO END", thin_section) {
                Ok(()) => println!("thin margins activated"),
                Err(e) => println!("error updating lib.sty: {}", e),
            }
        },
        "prev" => {
            match replace_section("./lib.sty", "% --- PREV START", "% --- PREV END", prev_section) {
                Ok(()) => println!("infinitely expanding pages activated."),
                Err(e) => println!("error updating lib.sty: {}", e),
            }
        },
        "nprev" => {
            match replace_section("./lib.sty", "% --- PREV START", "% --- PREV END", nprev_section) {
                Ok(()) => println!("normal page length restored."),
                Err(e) => println!("error updating lib.sty: {}", e),
            }
        },
        _ => (),
    }
}

fn replace_section(path: &str, start_marker: &str, end_marker: &str, new_content: &str) -> Result<(), String> {
    let s = read_to_string(path).map_err(|e| format!("read error: {}", e))?;
    let start_idx = s.find(start_marker).ok_or("start marker not found")?;
    let end_idx = s.find(end_marker).ok_or("end marker not found")?;
    let start_after = start_idx + start_marker.len();
    let end_after = end_idx /* + end_marker.len() */;
    // Keep the start marker line and end marker line, replace the inner content
    let before = &s[..start_after];
    let after = &s[end_after..];
    let new_file = format!("{}{}{}", before, new_content, after);
    write(path, new_file.as_bytes()).map_err(|e| format!("write error: {}", e))?;
    Ok(())
}

fn new(name: String) -> Result<(), i32> {
    let path: String = format!("./{name}");
    if let Err(_) = create_dir_all(path.as_str()) {
        return Err(0);
    }
    let main_path = path.clone() + "/main.tex";
    let lib_path = path.clone() + "/lib.sty";

    let mut main_file: File = 
    match File::create(main_path.clone()) {
        Ok(v) => v,
        Err(_) => return Err(1),
    };
    let mut lib_file =
    match File::create(lib_path.clone()) {
        Ok(v) => v,
        Err(_) => return Err(2),
    };

    let lib_preamble = 

r#"% --- auto generated, don't mess with the markers
\usepackage{amssymb}
\usepackage{amsmath}

\usepackage{mathunicode}
\usepackage{graphicx}
\graphicspath{{.}}

% --- LD START
\usepackage{xcolor}
\pagecolor[rgb]{1,1,1}
\color[rgb]{0,0,0}
% --- LD END

% --- GEO START
\usepackage{geometry}
\geometry{margin=1.25in}
\geometry{tmargin=1.25in}
\geometry{bmargin=1.5in}
% --- GEO END

% --- PREV START
% --- PREV END

% --- extra stuff

"#;

    let main_preamble = 

r#"\documentclass[10pt]{article}
\usepackage{lib}

\begin{document}



\end{document}"#;

    match main_file.write_all(main_preamble.as_bytes()) {
        Ok(_) => (),
        Err(_) => return Err(11),
    };
    match lib_file.write_all(lib_preamble.as_bytes()) {
        Ok(_) => (),
        Err(_) => return Err(12),
    };
    Ok(())
}