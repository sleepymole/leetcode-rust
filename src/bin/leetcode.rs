use std::fmt;
use std::io::{Read, Write};
use std::str::FromStr;

use anyhow::{anyhow, bail};
use clap::{Parser, Subcommand};
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use heck::AsSnakeCase;
use reqwest::blocking::Client;
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a solution structure.
    Init {
        /// The question ID to initialize a solution for
        question_id: i64,
        /// Force overwrite existing solution
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/bin/schema.graphql",
    query_path = "src/bin/question_list.graphql",
    response_derives = "Debug"
)]
struct QuestionList;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/bin/schema.graphql",
    query_path = "src/bin/question_data.graphql",
    response_derives = "Debug"
)]
struct QuestionData;

#[derive(Debug, Clone, Default)]
enum Type {
    #[default]
    Integer,
    String,
    Double,
    Boolean,
    ListNode,
    TreeNode,
    Array(Box<Type>),
}

impl Type {
    fn has_list_node(&self) -> bool {
        match self {
            Type::ListNode => true,
            Type::Array(t) => t.has_list_node(),
            _ => false,
        }
    }

    fn has_tree_node(&self) -> bool {
        match self {
            Type::TreeNode => true,
            Type::Array(t) => t.has_tree_node(),
            _ => false,
        }
    }

    fn default_value(&self) -> String {
        match self {
            Type::Integer => "0".to_owned(),
            Type::String => "\"\".to_owned()".to_owned(),
            Type::Double => "0.0".to_owned(),
            Type::Boolean => "false".to_owned(),
            Type::ListNode => "None".to_owned(),
            Type::TreeNode => "None".to_owned(),
            Type::Array(_) => "Vec::new()".to_owned(),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Integer => write!(f, "i32"),
            Type::String => write!(f, "String"),
            Type::Double => write!(f, "f64"),
            Type::Boolean => write!(f, "bool"),
            Type::ListNode => write!(f, "Option<Box<ListNode>>"),
            Type::TreeNode => write!(f, "Option<Rc<RefCell<TreeNode>>>"),
            Type::Array(t) => write!(f, "Vec<{}>", t),
        }
    }
}

impl FromStr for Type {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err_op = |_| anyhow!("invalid type: {}", s);
        if let Some(inner) = s.strip_suffix("[]") {
            return Ok(Type::Array(Box::new(inner.parse().map_err(err_op)?)));
        }
        if s.starts_with("list<") && s.ends_with('>') {
            return Ok(Type::Array(Box::new(
                s[5..s.len() - 1].parse().map_err(err_op)?,
            )));
        }
        match s {
            "integer" => Ok(Type::Integer),
            "string" => Ok(Type::String),
            "double" => Ok(Type::Double),
            "boolean" => Ok(Type::Boolean),
            "ListNode" => Ok(Type::ListNode),
            "TreeNode" => Ok(Type::TreeNode),
            _ => Err(anyhow!("invalid type: {}", s)),
        }
    }
}

impl<'de> Deserialize<'de> for Type {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(D::Error::custom)
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
struct Param {
    #[serde(default)]
    name: String,
    #[serde(rename = "type")]
    type_: Type,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct Constructor {
    #[serde(rename = "params")]
    _params: Vec<Param>,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct Method {
    #[serde(rename = "name")]
    _name: String,
    #[serde(rename = "params")]
    _params: Vec<Param>,
    #[serde(rename = "return")]
    _return_: Param,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct Metadata {
    #[serde(default)]
    name: String,
    #[serde(rename = "systemdesign", default)]
    system_design: bool,
    #[serde(rename = "classname", default)]
    _class_name: String,
    #[serde(rename = "constructor", default)]
    _constructor: Constructor,
    #[serde(rename = "methods", default)]
    _methods: Vec<Method>,
    #[serde(default)]
    params: Vec<Param>,
    #[serde(rename = "return", default)]
    return_: Param,
}

fn init_solution(question_id: i64, force: bool) -> Result<(), anyhow::Error> {
    let client = Client::new();
    let question_list = post_graphql::<QuestionList, _>(
        &client,
        "https://leetcode.com/graphql",
        question_list::Variables {
            category_slug: "".to_owned(),
            limit: Some(1),
            skip: Some(question_id - 1),
            filters: question_list::QuestionListFilterInput { tags: None },
        },
    )?
    .data
    .ok_or_else(|| anyhow!("response has no data"))?
    .question_list
    .questions;
    if question_list.is_empty() {
        bail!("no such question");
    }
    let question = &question_list[0];
    let resp_question_id: i64 = question.frontend_question_id.parse()?;
    if resp_question_id != question_id {
        bail!("no such question");
    }
    if question.paid_only {
        bail!("question is paid only");
    }

    let filename = format!(
        "s{:04}_{}.rs",
        question_id,
        question.title_slug.replace('-', "_")
    );
    if !force && std::path::Path::new(&format!("src/solutions/{filename}")).exists() {
        bail!("solution already exists");
    }

    let question_data = post_graphql::<QuestionData, _>(
        &client,
        "https://leetcode.com/graphql",
        question_data::Variables {
            title_slug: question.title_slug.to_owned(),
        },
    )?
    .data
    .ok_or_else(|| anyhow!("response has no data"))?
    .question;

    let metadata: Metadata = serde_json::from_str(&question_data.meta_data)?;
    if metadata.system_design {
        bail!("system design question is not supported yet");
    }

    let snippet = question_data
        .code_snippets
        .iter()
        .find(|s| s.lang_slug == "rust")
        .ok_or_else(|| anyhow!("question doesn't support rust solution"))?
        .code
        .as_str();

    let fn_name = if snippet.contains(&metadata.name) {
        metadata.name.clone()
    } else {
        let snake_case = AsSnakeCase(metadata.name.as_str()).to_string();
        if !snippet.contains(&snake_case) {
            bail!("snippet function name doesn't match metadata name");
        }
        snake_case
    };

    // Write solution file
    {
        let mut file = std::fs::File::create(format!("src/solutions/{filename}"))?;
        file.write_all("#![allow(dead_code)]\npub struct Solution {}\n\n".as_bytes())?;

        if metadata.params.iter().any(|p| p.type_.has_list_node())
            || metadata.return_.type_.has_list_node()
        {
            file.write_all("use crate::util::ListNode;\n\n".as_bytes())?;
        }
        if metadata.params.iter().any(|p| p.type_.has_tree_node())
            || metadata.return_.type_.has_tree_node()
        {
            file.write_all("use crate::util::TreeNode;\n\n".as_bytes())?;
            file.write_all("use std::cell::RefCell;\n".as_bytes())?;
            file.write_all("use std::rc::Rc;\n\n".as_bytes())?;
        }

        file.write_all("impl Solution {\n".as_bytes())?;
        file.write_all("    pub fn ".as_bytes())?;
        file.write_all(fn_name.as_bytes())?;
        file.write_all("(".as_bytes())?;
        for (i, param) in metadata.params.iter().enumerate() {
            if i > 0 {
                file.write_all(", ".as_bytes())?;
            }
            file.write_all(AsSnakeCase(param.name.as_str()).to_string().as_bytes())?;
            file.write_all(": ".as_bytes())?;
            file.write_all(param.type_.to_string().as_bytes())?;
        }
        file.write_all(") -> ".as_bytes())?;
        file.write_all(metadata.return_.type_.to_string().as_bytes())?;
        file.write_all(" {\n        ".as_bytes())?;
        file.write_all(metadata.return_.type_.default_value().as_bytes())?;
        file.write_all("\n    }\n}\n\n".as_bytes())?;

        file.write_all("#[cfg(test)]\nmod tests {\n    use super::*;\n\n    #[test]\n".as_bytes())?;
        file.write_all("    fn test_".as_bytes())?;
        file.write_all(fn_name.as_bytes())?;
        file.write_all("() {}\n".as_bytes())?;
        file.write_all("}\n".as_bytes())?;
        file.sync_all()?;
    }

    // Add solution module to mod.rs
    {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("src/solutions/mod.rs")?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let mod_name = format!(
            "s{:04}_{}",
            question_id,
            question.title_slug.replace('-', "_")
        );
        if !content.contains(&format!("mod {mod_name};")) {
            file.write_all(format!("mod {mod_name};\n").as_bytes())?;
            file.sync_all()?;
        }
    }

    println!("Solution is initialized at src/solutions/{}", filename);
    println!(
        "Question link: https://leetcode.com/problems/{}/",
        question.title_slug
    );
    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init { question_id, force } => {
            if question_id < 1 {
                bail!("question_id must be greater than 0");
            }
            init_solution(question_id, force)
        }
    }
}
