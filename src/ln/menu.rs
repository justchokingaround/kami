use colored::Colorize;

use crate::{
    helpers::take_input::string_input,
    ln::{
        self,
        scraper::{get_ln_chapters_urls, get_ln_id, get_ln_last_page},
        search::get_ln_chapters,
    },
    main, page_selector,
};

pub fn chapter_selector(ln_url: &str, mut selected_page: u32) -> (String, u32) {
    loop {
        let ln_html = ln::scraper::get_html(ln_url);
        let ln_id = get_ln_id(&ln_html);
        let ln_last_page = get_ln_last_page(&ln_html);
        let ln_page_html = page_selector(&ln_id, selected_page);
        let ln_chapters = get_ln_chapters(&ln_page_html);
        let ln_chapters_urls = get_ln_chapters_urls(&ln_page_html);
        let mut count = 0;
        ln_chapters.into_iter().for_each(|chaprer| {
            if count % 2 == 0 {
                println!("({})\t{}", count.to_string().blue(), format_args!("{}", chaprer.blue()));
            } else {
                println!("({})\t{}", count.to_string().yellow(), format_args!("{}", chaprer.yellow()));
            }
            count += 1;
        });
        println!("{}\t{}","n:".green(), "Go to next page".green());
        println!("{}\t{}","b:".yellow(), "Go to previous page".yellow());
        println!("{}\t{}","s:".green(), "Search another title".green());
        println!("{}\t{}","q:".red(), "quit".red());
        let chapter_number = string_input("Which chapter do you want to read? ");
        if chapter_number == "n" && selected_page < ln_last_page.parse::<u32>().unwrap() {
            selected_page += 1;
            print!("\x1B[2J\x1B[1;1H");
        } else if chapter_number == "b" && selected_page > 1 {
            selected_page -= 1;
            print!("\x1B[2J\x1B[1;1H");
        } else if chapter_number == "q" {
            print!("\x1B[2J\x1B[1;1H");
           std::process::exit(0);
        } else if chapter_number == "s" {
            main();
        }
        else {
            let chapter_number = chapter_number.trim().to_string();
            let mut _chapter_number_int = 0;
            if chapter_number.parse::<u32>().is_ok() {
                _chapter_number_int = chapter_number.parse::<u32>().unwrap();
            } else {
                println!("{}", "Invalid option".red());
                continue;
            }
            let chapter_url = &ln_chapters_urls[_chapter_number_int as usize];
            let chapter_url = chapter_url.trim().to_string();
            return (chapter_url, selected_page);
        }
    }
}
