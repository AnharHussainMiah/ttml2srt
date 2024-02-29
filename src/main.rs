use clap::Parser;
use minidom::Element;
use std::io::Write;

const LOGO: &str = r#"
_   _              _  _____          _   
| | | |           | |/ __  \        | |  
| |_| |_ _ __ ___ | |`' / /'___ _ __| |_ 
| __| __| '_ ` _ \| |  / / / __| '__| __|
| |_| |_| | | | | | |./ /__\__ \ |  | |_ 
 \__|\__|_| |_| |_|_|\_____/___/_|   \__|                                                                       
        
        2024 (c) TTML2SRT v0.1.0
"#;

#[derive(Parser, Default, Debug)]
#[clap(author="Anhar Hussain Miah", version, about=LOGO)]
struct Agruments {
    /// the TTML filename e.g ttml2srt -f subtitle.ttml
    #[clap(short, long)]
    file_name: String,
}

fn main() {
    let args = Agruments::parse();

    match self::to_str(&args.file_name) {
        Ok(srt) => {
            match std::fs::File::create(format!("{}", args.file_name.replace(".ttml", ".srt"))) {
                Ok(mut w) => {
                    for line in &srt {
                        writeln!(&mut w, "{}", line)
                            .expect(&format!("ERROR: unable to write line `{}`", line));
                    }
                    println!(
                        "success! created SRT file `{}` wrote [{}] lines",
                        args.file_name.replace(".ttml", ".srt"),
                        srt.len()
                    );
                }
                Err(e) => println!("ERROR: unable to create SRT file: {}", e),
            }
        }
        Err(e) => println!("ERROR: {}", e),
    }
}

fn to_str(f: &str) -> Result<Vec<String>, String> {
    let mut srt = Vec::new();

    let content = std::fs::read_to_string(f).map_err(|_| {
        format!(
            "unable to open file `{}` are you sure this exists or spelt correctly?",
            f
        )
    })?;
    let element: Element = content
        .parse()
        .map_err(|_| format!("unable to parse file `{}`", f))?;
    let ns = "http://www.w3.org/ns/ttml";

    let body = element
        .get_child("body", ns)
        .ok_or("unable to extract `body` element");
    let div = body?
        .get_child("div", ns)
        .ok_or("unable to extract `div` element");
    let ps = div?.children();

    let mut idx = 0;
    for child in ps {
        if let (Some(begin), Some(end)) = (child.attr("begin"), child.attr("end")) {
            idx += 1;
            srt.push(format!("{}", idx));
            srt.push(format!("{} --> {}", begin, end));
            for subs in child.children() {
                if subs.text() != "" {
                    srt.push(format!(
                        "{}",
                        subs.text().replace("\n", "").replace("\t", "")
                    ))
                }
            }
            srt.push(format!(""));
        }
    }
    Ok(srt)
}
