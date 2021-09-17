extern crate env_logger;
extern crate handlebars;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use serde_json::value::{ Map, Value as Json};

use std::io::Result;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use handlebars::{
     Handlebars
};

// fetch data from variable.json
pub fn handle_variables() -> Map<String, Json>  {
    let file_path = Path::new("./src/template/variables.json");
    let json_file = File::open(file_path).expect("file not found");
    let deserialized_file: Map<String, Json> = serde_json::from_reader(json_file).expect("error reading file");

    deserialized_file
}

pub fn handle_templating() -> Result<()> {

    let mut handlebars = Handlebars::new();

    let data = handle_variables();

    handlebars.register_template_file("template", "./src/template/template.hbs").unwrap();

    let mut output_file = File::create("./target/sheet.html")?;

    handlebars.render_to_write("template", &data, &mut output_file).expect("failed to write file");
    println!("target/table.html generated");

    Ok(())
}

pub fn handle_scss() -> Result<()> {

    let sass = grass::from_path("./src/css/input.scss", &grass::Options::default()).unwrap();

    let mut css_file = File::create("./target/styles.css")?;
    css_file.write_all(sass.as_bytes())?;

    Ok(())
}


fn main()-> Result<()> {
    
    env_logger::init();

    handle_templating()?;

    handle_scss()?;

    Ok(())
}
