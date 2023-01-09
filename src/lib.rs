use wikipedia::Wikipedia;

extern crate wikipedia;

pub fn run(args: &Vec<String>) -> Result<(), String> {
    let flags = get_flags(args);
    let params = get_params(args)?;

    //Should write output to file if WIKIWRITE env variable is set to 1
    for flag in &flags {
        match parse_flags(flag, &params) {
            Ok(s) => println!("{}", s),
            Err(e) => return Err(e)
        }
    }
    Ok(())
}

/*
    Acceptable flags will likely be:
    -h for help
    -v for version
    -l for link - done
    -c for categories - done
    -r for references - done
    -i for pageid - done
    -t for table of contents - done
    -s (or no flags) searches for a page (if two args are provided, second arg is treated as a section and the command gets section content) - done
 */
fn get_flags(args: &Vec<String>) -> Vec<String> {
    let pre = args.into_iter()
        .filter(|&s| (*s).starts_with("-"))
        .collect::<Vec<&String>>();

    let mut ret: Vec<String> = vec![];
    for i in 0..pre.len() {
        ret.push(pre[i].to_string().to_lowercase());
        ret[i].remove(0);
        if ret[i].len() > 1 {
            let second = ret[i].pop().unwrap().to_string();
            ret.push(second);
        }
    }

    if ret.len() == 0 {
        ret.push(String::from(""));
    }
    ret
}

fn get_params(args: &Vec<String>) -> Result<Vec<String>, &str> {
    let pre = args.into_iter()
        .filter(|&s| !(*s).starts_with("-"))
        .collect::<Vec<&String>>();

    let mut params = vec![];

    for s in pre {
        params.push(s.to_string());
    }
    if params.len() > 3 {
        Err("Error: too many arguments provided")
    } else {
        Ok(params)
    }
}

fn parse_flags(flag: &String, params: &Vec<String>) -> Result<String, String> {
    let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
    if flag.len() == 0 {
        return if params.len() > 2 {search_section(&wiki, params.get(1), params.last())} else {search_summary(&wiki, params.last())}
    } else {
        return match flag.as_str() {
            "s" => {
                if params.len() > 2 {
                    search_section(&wiki, params.get(1), params.last())
                } else {
                    search_summary(&wiki, params.last())
                }
            },
            "p" => pageid(&wiki, params.last()),
            "t" => table_of_contents(&wiki, params.last()),
            "r" => references(&wiki, params.last()),
            "c" => categories(&wiki, params.last()),
            "l" => link(&wiki, params.last()),
            "v" => version(),
            "h" => help(),
            _ => return Err(String::from("Unknown flag provided"))
        };
    }   
}

fn too_few_arguments(query: Option<&String>) -> Result<(), String> {
    if query.is_none() {
        Err(String::from("Error: too few arguments"))
    } else {
        Ok(())
    }
}

fn matchout(input: Result<String, wikipedia::Error>) -> Result<String, String> {
    match input {
        Ok(s) => Ok(s),
        Err(e) => Err(format!("Error: Wikipedia failed to fetch data: {}", e.to_string()))
    }
}

fn search_summary(wiki: &Wikipedia<wikipedia::http::default::Client>, query: Option<&String>) -> Result<String, String> {
    too_few_arguments(query)?;

    let page = wiki.page_from_title(query.unwrap().to_lowercase());

    matchout(page.get_summary())
}

fn search_section(wiki: &Wikipedia<wikipedia::http::default::Client>, title: Option<&String>, section: Option<&String>) -> Result<String, String> {
    too_few_arguments(title)?;
    too_few_arguments(section)?;

    let page = wiki.page_from_title(title.unwrap().to_lowercase());
    match page.get_section_content(section.unwrap()) {
        Ok(s) => {
            if let Some(c) = s {
                Ok(c)
            } else {
                //Err(String::from("Error: section not found,"))
                Err(format!("Section not found {}", section.unwrap()))
            }
        },
        Err(e) => Err(format!("Error: Wikipedia failed to fetch data: {}", e.to_string()))
    }
}

fn pageid(wiki: &Wikipedia<wikipedia::http::default::Client>, query: Option<&String>) -> Result<String, String> {
    too_few_arguments(query)?;

    let page = wiki.page_from_title(query.unwrap().to_lowercase());
    
    matchout(page.get_pageid())
}

fn table_of_contents(wiki: &Wikipedia<wikipedia::http::default::Client>, query: Option<&String>) -> Result<String, String> {
    too_few_arguments(query)?;

    let page = wiki.page_from_title(query.unwrap().to_lowercase());
    
    let mut out = String::from("Sections:\n");
    match page.get_sections() {
        Ok(v) => {
            for s in v {
                out.push_str(&format!("{}\n", s));
            }
        },
        Err(e) => return Err(format!("Error: Wikipedia failed to fetch data: {}", e.to_string())),
    }

    Ok(out)
}

fn references(wiki: &Wikipedia<wikipedia::http::default::Client>, query: Option<&String>) -> Result<String, String> {
    too_few_arguments(query)?;

    let page = wiki.page_from_title(query.unwrap().to_lowercase());

    let mut refs = String::new();
    match page.get_references() {
        Ok(i) => {
            for s in i {
                refs.push_str(&format!("{}\n", s.url));
            }
        },
        Err(e) => return Err(format!("Error: Wikipedia failed to fetch data: {}", e.to_string()))
    }

    Ok(refs)
}

fn categories(wiki: &Wikipedia<wikipedia::http::default::Client>, query: Option<&String>) -> Result<String, String> {
    too_few_arguments(query)?;

    let page = wiki.page_from_title(query.unwrap().to_owned());

    let mut cats = String::new();
    match page.get_categories() {
        Ok(i) => {
            for s in i {
                cats.push_str(&format!("{}\n", s.title));
            }
        },
        Err(e) => return Err(format!("Error: Wikipedia failed to fetch data: {}", e.to_string()))
    }

    Ok(cats)
}

fn link(wiki: &Wikipedia<wikipedia::http::default::Client>, query: Option<&String>) -> Result<String, String> {
    too_few_arguments(query)?;

    match wiki.page_from_title(query.unwrap().to_owned()).get_title() {
        Ok(_) => Ok(format!("https://wikipedia.org/wiki/{}", query.unwrap())),
        Err(e) => Err(format!("Error: invalid link: {}", e.to_string()))
    }
}

fn version() -> Result<String, String> {
    Ok(format!("qwiki version {}", env!("CARGO_PKG_VERSION")))
}

fn help() -> Result<String, String> {
    Ok(concat!(
        "-s: Gets the summary of the Wikipedia article specified by the argument provided. If two arguments are provided instead, gets the content of the section (specified by the second argument) from the article (specified by the first argument).",
        "This flag is used by default if no flags are provided.\n",
        "-t: Lists all of the sections of the Wikipedia article specified by the argument provided.\n",
        "-r: Gets all of the references of the Wikipedia article specified by the argument provided.\n",
        "-c: Gets all of the categories of the Wikipedia article specified by the argument provided.\n",
        "-l: Gets a link to the Wikipedia article specified by the argument provided.\n",
        "-p: Gets the pageid of the Wikipedia article specified by the argument provided.\n",
        "-v: Gets the version of qwiki."
    ).to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_flags() {
        let args = vec![String::from("cow"), String::from("-s"), String::from("-e")];
        let result = get_flags(&args);
        let check = vec![String::from("s"), String::from("e")];
        let matching = result.iter().zip(check.iter()).filter(|&(a, b)| a == b).count();
        assert!(matching == result.len() && matching == check.len());
    }

    #[test]
    fn check_flags_long() {
        let args = vec![String::from("cow"), String::from("-s"), String::from("-pe")];
        let result = get_flags(&args);
        let check = vec![String::from("s"), String::from("p"), String::from("e")];
        let matching = result.iter().zip(check.iter()).filter(|&(a, b)| a == b).count();
        assert!(matching == result.len() && matching == check.len());
    }

    #[test]
    fn check_params_ok() {
        let args = vec![String::from("co-w"), String::from("moose"), String::from("-s"), String::from("-e")];
        let result = get_params(&args);
        let check = vec![String::from("co-w"), String::from("moose")];
        assert!(result.is_ok());
        let matching = result.as_ref().unwrap().iter().zip(check.iter()).filter(|&(a, b)| a == b).count();
        assert!(matching == result.unwrap().len() && matching == check.len());
    }

    #[test]
    fn check_params_too_many() {
        let args = vec![String::from("-s"), String::from("cow"), String::from("moose"), String::from("geese"), String::from("jackrabbit"), String::from("-e")];
        let result = get_params(&args);
        assert!(if result.is_err() && result.unwrap_err() == "Error: too many arguments provided" {true} else {false});
    }

    #[test]
    fn check_case() {
        let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
        let test = search_summary(&wiki, Some(&String::from("cristiano ronaldo")));
        assert!(test.is_ok());
    }
}