use std::env;
use std::io;

extern crate term;
use std::io::prelude::*;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

use curl::easy::{Easy, List};

const SCREENSHOT_LIMIT: usize = 5;

#[derive(Debug)]
struct ModInfo {
    modtype: String,
    name: String,
    author: String,
    desc: String,
    license: String,
    media_license: String,
    repo: String,
    screenshots: [String; SCREENSHOT_LIMIT],
    title: String,
    download_link: String,
    depends: String,
    optional_depends: String,
    contentdb: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    let mut result: String = String::new();
    let mut input: String = String::new();

    let mut modinfo = ModInfo {
        modtype: String::from("Mod"),
        name: String::new(),
        title: String::new(),
        author: String::new(),
        desc: String::new(),
        license: String::new(),
        media_license: String::new(),
        repo: String::new(),
        screenshots: Default::default(),
        download_link: String::new(),
        depends: String::new(),
        optional_depends: String::new(),
        contentdb: String::new(),
    };

    print_color("[MT Forums Modpost Generator]\n\n", term::color::GREEN);

    if args.len() > 1 {
        if args[1] == "--fromcdb" || args[1] == "-c" {
            if args.len() <= 2 {
                get_input(true, "Please link your ContentDB page", &mut input);
            } else {
                input = args[2].clone();
            }

            let mut link: String = input.replace(
                "https://content.minetest.net/",
                "https://content.minetest.net/api/",
            );

            if input == link {
                print_color("Invalid link!\n\n", term::color::RED);
                return;
            } else {
                let result = get_info_from_cdb(&link);

                modinfo.name.push_str(result["name"].as_str().unwrap());
                modinfo.title.push_str(result["title"].as_str().unwrap());
                modinfo.author.push_str(result["author"].as_str().unwrap());
                modinfo
                    .desc
                    .push_str(result["short_description"].as_str().unwrap());
                modinfo
                    .license
                    .push_str(result["license"].as_str().unwrap());
                modinfo
                    .media_license
                    .push_str(result["media_license"].as_str().unwrap());
                modinfo.repo.push_str(result["repo"].as_str().unwrap());

                for (idx, val) in result["screenshots"].members().enumerate() {
                    if idx < SCREENSHOT_LIMIT {
                        modinfo.screenshots[idx].push_str(val.as_str().unwrap());
                    } else {
                        break;
                    }
                }

                modinfo.download_link = link.replace("api/", "");

                // download link is cdb link without /download on the end
                modinfo.contentdb = modinfo.download_link.clone();
                modinfo.download_link.push_str("download/");

                link.push_str("dependencies/");

                let result = get_info_from_cdb(&link);

                for dep in result[modinfo.author.clone() + "/" + &modinfo.name].members() {
                    let mut dependptr = &mut modinfo.depends;
                    if dep["is_optional"] == true {
                        dependptr = &mut modinfo.optional_depends;
                    }

                    if !dependptr.is_empty() {
                        dependptr.push_str(", ");
                    }

                    dependptr.push_str(dep["name"].as_str().unwrap())
                }
            }
        } else {
            println!("Usage: {} [--fromcdb | -c]", args[0]);
            return;
        }
    } else {
        print_color("To skip a request just press enter\n", term::color::YELLOW);

        get_input(
            true,
            "Please provide the mod name used in your mod.conf",
            &mut modinfo.name,
        );

        get_input(
            true,
            "Please provide a human-readable mod name",
            &mut modinfo.title,
        );

        get_input(
            true,
            "Please provide a one-line mod description",
            &mut modinfo.desc,
        );

        get_input(true, "Please provide a code license", &mut modinfo.license);

        get_input(
            false,
            "Please provide a media license",
            &mut modinfo.media_license,
        );

        get_input(
            false,
            "Please provide a link to your git repo",
            &mut modinfo.repo,
        );

        for idx in 0..SCREENSHOT_LIMIT {
            if !get_input(
                false,
                "Please link a screenshot",
                &mut modinfo.screenshots[idx],
            ) {
                break;
            }
        }

        get_input(
            false,
            "Please provide a direct download link",
            &mut modinfo.download_link,
        );

        get_input(
            false,
            "Please list your hard dependencies",
            &mut modinfo.depends,
        );

        get_input(
            false,
            "Please list your optional dependencies",
            &mut modinfo.optional_depends,
        );

        get_input(false, "Is your mod a Work In Progress? (y/n)", &mut input);

        if input.contains("y") {
            modinfo.modtype = String::from("WIP")
        }
    }

    print_color("[!Forum Post Generated!]\n\n", term::color::GREEN);

    // Give forum post title
    result.push_str(&format!(
        "[{}] {} [{}]",
        modinfo.modtype, modinfo.title, modinfo.name
    ));

    if get_yn_answer("Do you want to copy the mod post title to your clipboard? (y/n)") {
        ctx.set_contents(result.to_owned()).unwrap();

        println!("Copied to clipboard!\n")
    }

    print_color(
        &format!("\n{}\n{}\n{}\n\n", "-".repeat(8), result, "-".repeat(8)),
        term::color::YELLOW,
    );

    get_yn_answer("Press Enter to continue"); // you see nothing

    result.clear();
    get_result(modinfo, &mut result);

    if get_yn_answer("Do you want to copy the mod post contents to your clipboard? (y/n)") {
        ctx.set_contents(result.to_owned()).unwrap();

        println!("Copied to clipboard!\n")
    }

    print_color(
        &format!("\n{}\n{}\n{}\n\n", "-".repeat(15), &result, "-".repeat(15),),
        term::color::YELLOW,
    );
}

fn get_result(modinfo: ModInfo, result: &mut String) {
    // Add header
    result.push_str(&format!("[h]{}[/h]\n{}\n\n", modinfo.title, modinfo.desc));

    // Show one screenshot and put the rest in a spoiler
    if !modinfo.screenshots[0].is_empty() {
        result.push_str(&format!("[img]{}[/img]\n", modinfo.screenshots[0]));

        if !modinfo.screenshots[1].is_empty() {
            result.push_str("[tspoiler=More Screenshots]");

            for idx in 1..SCREENSHOT_LIMIT {
                if modinfo.screenshots[idx].is_empty() {
                    break;
                }

                result.push_str(&format!("\n[img]{}[/img]", modinfo.screenshots[idx]));
            }

            result.push_str("\n[/tspoiler]\n\n")
        } else {
            result.push('\n');
        }
    }

    result.push_str("[h][/h]\n\n");

    // Add direct download link
    if !modinfo.download_link.is_empty() {
        result.push_str(&format!(
            "[b]Downloads:[/b] [url={}]Latest Stable[/url]\n",
            modinfo.download_link
        ));
    }

    // Add repo link
    if !modinfo.repo.is_empty() {
        result.push_str(&format!(
            "[b]View:[/b] [url={}]Source Code[/url]",
            modinfo.repo
        ));
        if !modinfo.contentdb.is_empty() {
            result.push_str(&format!(" | [url={}]ContentDB[/url]", modinfo.contentdb));
        }

        result.push_str("\n\n");
    } else {
        result.push('\n');
    }

    // Add mod license(s)
    if modinfo.media_license.is_empty() {
        result.push_str(&format!("[b]License:[/b] {}\n\n", modinfo.license));
    } else {
        result.push_str(&format!("[b]License of Code:[/b] {}\n", modinfo.license));
        result.push_str(&format!(
            "[b]License of Media:[/b] {}\n\n",
            modinfo.media_license
        ));
    }

    // Add mod dependencies
    if !modinfo.depends.is_empty() {
        result.push_str(&format!("[b]Depends:[/b] {}\n", modinfo.depends));
    }
    if !modinfo.optional_depends.is_empty() {
        result.push_str(&format!(
            "[b]Optional Depends:[/b] {}\n",
            modinfo.optional_depends
        ));
    }
}

fn get_input(required: bool, message: &str, string: &mut String) -> bool {
    print!(
        "\n[{}] {}\n\n",
        if required { "Required" } else { "Optional" },
        message
    );

    while string.is_empty() {
        io::stdin().read_line(string).expect("Failed to read");
        *string = String::from(string.trim());

        if !required {
            break;
        }
    }

    string.shrink_to_fit();
    return !string.is_empty();
}

// Get y/n answer
fn get_yn_answer(message: &str) -> bool {
    let mut string: String = String::new();

    println!("{}\n", message);

    io::stdin().read_line(&mut string).expect("Failed to read");
    string = String::from(string.trim());

    return string.contains("y");
}

fn print_color(message: &str, color: term::color::Color) {
    let mut t = term::stdout().unwrap();

    t.fg(color).unwrap();
    write!(t, "{}", message).unwrap();

    t.reset().unwrap();
}

fn get_info_from_cdb(link: &String) -> json::JsonValue {
    let mut easy = Easy::new();
    let mut list = List::new();
    let mut json_buf = Vec::new();

    easy.url(&link).unwrap();
    list.append("Content-Type: application/json").unwrap();
    easy.http_headers(list).unwrap();
    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|data| {
                json_buf.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }

    return json::parse(std::str::from_utf8(json_buf.as_slice()).expect("Invalid UTF-8")).unwrap();
}
