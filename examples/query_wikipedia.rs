use headless_chrome::{Browser, LaunchOptionsBuilder};

fn query(input: &str) -> Result<(), failure::Error> {
    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .build()
            .expect("Could not find chrome-executable"),
    )?;
    let tab = browser.wait_for_initial_tab()?;
    tab.navigate_to("https://en.wikipedia.org")?
        .wait_for_element("input#searchInput")?
        .click()?;
    tab.type_str(&input)?.press_key("Enter")?;
    match tab.wait_for_element("div.shortdescription") {
        Err(e) => eprintln!("Query failed: {:?}", e),
        Ok(e) => match e.get_description()?.find(|n| n.node_name == "#text") {
            Some(n) => println!("Result for `{}`: {}", &input, n.node_value),
            None => eprintln!("No shortdescription-node found on page"),
        },
    }
    Ok(())
}

fn main() -> Result<(), failure::Error> {
    let input = "Elvis Aaron Presley";
    query(input)
}
