extern crate env_logger;
extern crate handlebars;
extern crate serde_derive;
extern crate serde_json;

use serde_json::value::{Map, Value as Json};

use std::io::Result as NoErroResult;
use std::fs::File;
use std::fs::read_to_string;
use std::io::Write;
use std::path::Path;

use handlebars::{Helper,Handlebars, Context, RenderContext, Output, RenderError, JsonRender};

pub fn deserialized_json_file(path: &str) -> NoErroResult<Map<String, Json>> {
    let base_path: String = "./src/template/".to_owned();
    let full_path = base_path + path;

    let file_path = Path::new(&full_path);
    let json_file = File::open(file_path).expect("file not found");

    let deserialized_file: Map<String, Json> =
        serde_json::from_reader(json_file).expect("error reading file");

    Ok(deserialized_file)
}

// fetch data from variable.json
pub fn handle_variables() -> Map<String, Json> {
    let file_path = Path::new("./src/template/variables.json");
    let json_file = File::open(file_path).expect("file not found");
    let deserialized_file: Map<String, Json> =
        serde_json::from_reader(json_file).expect("error reading file");

    deserialized_file
}

pub fn handle_templating() -> NoErroResult<()> {
    let base_path = "./src/template/";
    let handlebars = Handlebars::new();

    let data = deserialized_json_file("variables.json")?;

    let partials_json = deserialized_json_file("partials.json").unwrap();
    let partials_path_array = partials_json["partials"].as_array().unwrap();
    //fetch partials content
    let partials_array: Vec<String> = partials_path_array.iter()
    .map(|x| read_to_string(base_path.to_owned() + x.as_str().expect("value from partials.json can't be read"))
    .expect("partials in partials.json is not an array"))
    .collect();

    //merge all partials content
    let partials: String = partials_array.iter()
    .flat_map(|s| s.chars())
    .collect();

    let template = read_to_string(base_path.to_owned() + "template.hbs").expect("error template. check if syntaxe is valid");
    let full_template = partials + template.clone().as_mut_str(); 

    let mut output_file = File::create("./target/sheet.html")?;
    handlebars
        .render_template_to_write(&full_template, &data, &mut output_file)
        .expect("failed to write file");

    println!("target/table.html generated");

    Ok(())
}

pub fn handle_scss() -> NoErroResult<()> {
    let sass = grass::from_path("./src/css/input.scss", &grass::Options::default()).unwrap();

    let mut css_file = File::create("./target/styles.css")?;
    css_file.write_all(sass.as_bytes())?;

    Ok(())
}

fn main() -> NoErroResult<()> {
    env_logger::init();

    handle_templating()?;

    //handle_scss()?;

    Ok(())
}
