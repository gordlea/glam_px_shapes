#[macro_use]
extern crate tera;
#[macro_use]
extern crate lazy_static;
extern crate serde_json;

use std::{collections::HashMap, default, vec};

use anyhow::Context;
use serde_json::{json, value::{to_value, Value}};
use std::error::Error;
use tera::{Result, Tera};
use rustfmt_wrapper::rustfmt;

const PROJECT_ROOT: &str = "..";

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec!["html", ".sql"]);
        tera.register_filter("do_nothing", do_nothing_filter);
        tera
    };
}

pub fn do_nothing_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("do_nothing_filter", "value", String, value);
    Ok(to_value(s).unwrap())
}

fn generate_file(
    tera: &tera::Tera,
    context: &tera::Context,
    template_path: &str,
) -> anyhow::Result<String> {
    tera.render(template_path, context)
        .context("tera render error")
}

struct TemplateCfg {
    pub shape: String,
    pub template_file_name: String,
    pub output_file_name: String,
    pub template_dir: String,
    pub output_dir: String,
    pub context: tera::Context,
}

impl default::Default for TemplateCfg {
    fn default() -> Self {
        Self {
            shape: "Rectangle".to_string(),
            template_file_name: "somefile.rs.tera".to_string(),
            output_file_name: "somefile.rs".to_string(),
            template_dir: "./".to_string(),
            output_dir: "../src".to_string(),
            context: tera::Context::new(),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let repo = git2::Repository::open(PROJECT_ROOT).context("failed to open git repo")?;
    let workdir = repo.workdir().unwrap();

    let mut context = tera::Context::new();
    context.insert("message", &"Bob");

    let ctx = tera::Context::from_value(json!({
        "name": "Rectangle",
    }));

    // let shapes = vec!["rectangle", "circle"];
    // let glam_types = vec!["IVec2", "UVec2", "Vec2"];


    let mut template_cfgs: Vec<TemplateCfg> = vec![];

    for shape in ["Rectangle", "Circle"] {
        for (glam_type, num_class, num_size) in [("IVec2", "i", 32), ("UVec2", "u", 32), ("Vec2", "f", 32)] {
            let int_based = num_class != "f";
            let num_type = format!("{}{}", num_class, num_size);
            template_cfgs.push(TemplateCfg {
                shape: shape.to_string(),
                output_file_name: format!("{}_{}.rs", shape.to_ascii_lowercase(), glam_type.to_ascii_lowercase()),
                template_file_name: format!("{}.rs.tera", shape.to_ascii_lowercase()),
                template_dir: "./".to_string(),
                output_dir: "../src/rectangle".to_string(),
                context: tera::Context::from_value(json!({
                    "name": shape,
                    "glam_type": glam_type,
                    "num_type": num_type,
                    "int_based": int_based,
                    "num_size": num_size, 
                    "num_suffix": if int_based { "" } else { ".0" },
                })).unwrap(),
                ..Default::default()
            })
        }
    }
    
    for tcfg in template_cfgs {
        let mut output_str = generate_file(&TEMPLATES, &tcfg.context, tcfg.template_file_name.as_str())?;
        // output_str = rustfmt(&output_str).context("rustfmt failed")?;
        println!("{:?}", output_str);
        // let output_path = format!("{}/{}", tcfg.output_dir, tcfg.output_file_name);
        let output_path = workdir.join("src").join(tcfg.shape.to_ascii_lowercase()).join(tcfg.output_file_name);
        std::fs::write(&output_path, output_str)
            .with_context(|| format!("failed to write {:?}", output_path))?;

    }



    // A one off template
    // Tera::one_off("hello", &Context::new(), true).unwrap();

    // match TEMPLATES.render("rectangle.rs.tera", &context) {
    //     Ok(s) => println!("{:?}", s),
    //     Err(e) => {
    //         println!("Error: {}", e);
    //         let mut cause = e.source();
    //         while let Some(e) = cause {
    //             println!("Reason: {}", e);
    //             cause = e.source();
    //         }
    //     }
    // };
    Ok(())
}
