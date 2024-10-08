use std::fs::{File, create_dir};
use std::io::Write;
use std::process::Command;

fn main () {
    println!("henlo");
    let args: Vec<String> = std::env::args().collect();
    let command: String = args[1].clone();

    let light_string =
r#"\usepackage{xcolor}
\pagecolor[rgb]{1,1,1}
\color[rgb]{0,0,0}"#;
    let dark_string = 
r#"\usepackage{xcolor}
\pagecolor[rgb]{0,0,0}
\color[rgb]{1,1,1}"#;
    let wide_string =
r#"\usepackage{geometry}
\geometry{margin=0.75in}
\geometry{tmargin=0.75in}
\geometry{bmargin=1in}"#;
    let thin_string =
r#"\usepackage{geometry}
\geometry{margin=1.25in}
\geometry{tmargin=1.25in}
\geometry{bmargin=1.5in}"#;

    match command.as_str() {
        "new" => {
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
            let mut colorfile = File::create("./ld.sty").expect("failed to open light-dark file");
            colorfile.write_all(light_string.as_bytes()).expect("failed to lightenize ld.sty");
            println!("rise + shine! light mode activated");
        },
        "dark" => {
            let mut colorfile = File::create("./ld.sty").expect("failed to open light-dark file");
            colorfile.write_all(dark_string.as_bytes()).expect("failed to darkenize ld.sty");
            println!("dark mode activated, night night");
        },
        "wide" => {
            let mut geofile = File::create("geo.sty").expect("failed to open geometry file");
            geofile.write_all(wide_string.as_bytes()).expect("failed to hit the gym consistently :/");
        },
        "thin" => {
            let mut geofile = File::create("geo.sty").expect("failed to open geometry file");
            geofile.write_all(thin_string.as_bytes()).expect("failed to cut that belly fat :/");
        },
        _ => (),
    }
}

fn new(name: String) -> Result<(), i32> {
    let path: String = format!("./{name}");
    create_dir(path.as_str());
    let mut main_file: File = 
    match File::create(path.clone() + "/main.tex") {
        Ok(v) => v,
        Err(_) => return Err(1),
    };
    let mut lib_file =
    match File::create(path.clone() + "/lib.sty") {
        Ok(v) => v,
        Err(_) => return Err(2),
    };
    let mut ld_file =
    match File::create(path.clone() + "/ld.sty") {
        Ok(v) => v,
        Err(_) => return Err(3),
    };
    let mut geo_file =
    match File::create(path.clone() + "/geo.sty") {
        Ok(v) => v,
        Err(_) => return Err(4),
    };

    let lib_preamble = 

r#"\usepackage{amssymb}
\usepackage{amsmath}
\usepackage{mathunicode}
\usepackage{ld}
\usepackage{geo}

"#;

    let main_preamble = 

r#"\documentclass[10pt]{article}
\usepackage{lib}

\begin{document}



\end{document}"#;

    let ld_preamble =
r#"\usepackage{xcolor}
\pagecolor[rgb]{1,1,1}
\color[rgb]{0,0,0}"#;
    let geo_preamble =
r#"\usepackage{geometry}
\geometry{margin=1.25in}
\geometry{tmargin=1.25in}
\geometry{bmargin=1.5in}"#;

    match main_file.write_all(main_preamble.as_bytes()) {
        Ok(_) => (),
        Err(_) => return Err(11),
    };
    match lib_file.write_all(lib_preamble.as_bytes()) {
        Ok(_) => (),
        Err(_) => return Err(12),
    };
    match ld_file.write_all(ld_preamble.as_bytes()) {
        Ok(_) => (),
        Err(_) => return Err(13),
    };
    match geo_file.write_all(geo_preamble.as_bytes()) {
        Ok(_) => (),
        Err(_) => return Err(14),
    };
    Ok(())
}