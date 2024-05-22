#[macro_use]
extern crate tera;
#[macro_use]
extern crate lazy_static;
extern crate serde_json;

use std::{collections::HashMap, default, path::PathBuf, vec};
use heck::ToSnakeCase;
use anyhow::Context;
use serde_json::{json, value::{to_value, Value}};
use tera::{Result, Tera};

const PROJECT_ROOT: &str = "..";

lazy_static! {
    pub static ref SHAPE_TEMPLATES: Tera = {
        let tera = match Tera::new("templates/shapes/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        // tera.autoescape_on(vec!["html", ".sql"]);
        // tera.register_filter("do_nothing", do_nothing_filter);
        tera
    };
    pub static ref INDEX_TEMPLATES: Tera = {
        let tera = match Tera::new("templates/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        // tera.autoescape_on(vec!["html", ".sql"]);
        // tera.register_filter("do_nothing", do_nothing_filter);
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

#[derive(Debug, Clone, Default)]
struct ShapeTemplateCfg {
    pub shape: String,
    pub output_file_path: PathBuf,
    pub template_file_name: String,
    pub context: tera::Context,
}

#[derive(Debug, Clone, Default)]
struct IndexTemplateCfg {
    pub output_file_path: PathBuf,
    pub template_file_name: String,
    pub context: tera::Context,
}

// impl default::Default for TemplateCfg {
//     fn default() -> Self {
//         Self {
//             shape: "Rectangle".to_string(),
//             output_file_path: PathBuf::new(),
//             template_file_name: "somefile.rs.tera".to_string(),
//             // output_file_name: "somefile.rs".to_string(),
//             context: tera::Context::new(),
//         }
//     }
// }

const GLAM_TYPES: [&'static str; 3] = ["IVec2", "UVec2", "Vec2"];
const SHAPES: [&'static str; 3] = ["Rectangle", "Circle", "LineSegment"];
// [("IVec2", "i", 32), ("UVec2", "u", 32), ("Vec2", "f", 32)];

fn main() -> anyhow::Result<()> {
    let repo = git2::Repository::open(PROJECT_ROOT).context("failed to open git repo")?;
    let workdir = repo.workdir().unwrap();


    let mut template_cfgs: Vec<ShapeTemplateCfg> = vec![];
    let mut index_cfgs: Vec<IndexTemplateCfg> = vec![];


    for glam_type in GLAM_TYPES {
        for shape in SHAPES {
            let (num_class, num_size) = match glam_type {
                "IVec2" => ("i", 32),
                "UVec2" => ("u", 32),
                "Vec2" => ("f", 32),
                _ => unreachable!(),
            };
            let int_based = num_class != "f";
            let num_type = format!("{}{}", num_class, num_size);
            template_cfgs.push(ShapeTemplateCfg {
                shape: shape.to_string(),
                // output_file_name: format!("{}_{}.rs", shape.to_ascii_lowercase(), glam_type.to_ascii_lowercase()),
                template_file_name: format!("{}.rs.tera", shape.to_snake_case()),
                output_file_path: workdir.join("src")
                    // .join(shape.to_ascii_lowercase())
                    .join(glam_type.to_ascii_lowercase())
                    .join(format!("{}.rs", shape.to_snake_case())),
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
        index_cfgs.push(IndexTemplateCfg {
            // shape: shape.to_string(),
            template_file_name: "mod.rs.tera".to_string(),
            output_file_path: workdir.join("src")
                .join(glam_type.to_ascii_lowercase())
                .join("mod.rs"),
            context: tera::Context::from_value(json!({
                "shapes": SHAPES,
                "snake_case_shapes": SHAPES.iter().map(|s| s.to_snake_case()).collect::<Vec<String>>(),
                "glam_type": glam_type,
                "path_delim": std::path::MAIN_SEPARATOR,
            })).unwrap(),
            ..Default::default()
        })
    }


    // for shape in ["Rectangle", "Circle"] {
    //     for glam_type in GLAM_TYPES {
    //         let (num_class, num_size) = match glam_type {
    //             "IVec2" => ("i", 32),
    //             "UVec2" => ("u", 32),
    //             "Vec2" => ("f", 32),
    //             _ => unreachable!(),
    //         };
    //         let int_based = num_class != "f";
    //         let num_type = format!("{}{}", num_class, num_size);
    //         template_cfgs.push(TemplateCfg {
    //             shape: shape.to_string(),
    //             // output_file_name: format!("{}_{}.rs", shape.to_ascii_lowercase(), glam_type.to_ascii_lowercase()),
    //             template_file_name: format!("{}.rs.tera", shape.to_ascii_lowercase()),
    //             output_file_path: workdir.join("src")
    //                 // .join(shape.to_ascii_lowercase())
    //                 .join(glam_type.to_ascii_lowercase())
    //                 .join(format!("{}.rs", shape.to_ascii_lowercase())),
    //             context: tera::Context::from_value(json!({
    //                 "name": shape,
    //                 "glam_type": glam_type,
    //                 "num_type": num_type,
    //                 "int_based": int_based,
    //                 "num_size": num_size, 
    //                 "num_suffix": if int_based { "" } else { ".0" },
    //             })).unwrap(),
    //             ..Default::default()
    //         })
    //     }
    //     index_cfgs.push(TemplateCfg {
    //         shape: shape.to_string(),
    //         template_file_name: "mod.rs.tera".to_string(),
    //         output_file_path: workdir.join("src")
    //             .join(glam_type.to_ascii_lowercase())
    //             .join("mod.rs"),
    //         context: tera::Context::from_value(json!({
    //             "name": shape,
    //             "glam_types": GLAM_TYPES,
    //             "path_delim": std::path::MAIN_SEPARATOR,
    //         })).unwrap(),
    //         ..Default::default()
    //     })
    // }
    
    for tcfg in template_cfgs {
        let output_str = generate_file(&SHAPE_TEMPLATES, &tcfg.context, tcfg.template_file_name.as_str())?;
        // println!("{:?}", output_str);
        let output_path = tcfg.output_file_path;

        let dirname = output_path.parent().unwrap();

        if !dirname.exists() {
            std::fs::create_dir_all(&dirname)
                .with_context(|| format!("failed to create {:?}", dirname))?;
        }
        std::fs::write(&output_path, output_str)
            .with_context(|| format!("failed to write {:?}", output_path))?;

    }

    for icfg in index_cfgs {
        let output_str = generate_file(&INDEX_TEMPLATES, &icfg.context, icfg.template_file_name.as_str())?;
        // println!("{:?}", output_str);
        let output_path = icfg.output_file_path;

        // let dirname = output_path.parent().unwrap();

        // if !dirname.exists() {
        //     std::fs::create_dir_all(&dirname)
        //         .with_context(|| format!("failed to create {:?}", dirname))?;
        // }
        std::fs::write(&output_path, output_str)
            .with_context(|| format!("failed to write {:?}", output_path))?;
    }

    Ok(())
}
