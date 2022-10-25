use std::io::{Read, Write};

use anyhow::{anyhow, bail};
use clap::{Parser, Subcommand};
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use reqwest::blocking::Client;

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
        "p{:04}_{}.rs",
        question_id,
        question.title_slug.replace('-', "_")
    );
    if !force
        && (std::path::Path::new(&format!("src/{filename}")).exists()
            || std::path::Path::new(&format!("src/archived/{filename}")).exists())
    {
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

    println!("metadata: {:?}", question_data.meta_data);
    println!("test case: {:?}", question_data.sample_test_case);

    let snippet = question_data
        .code_snippets
        .iter()
        .find(|s| s.lang_slug == "rust")
        .ok_or_else(|| anyhow!("rust is unsupported"))?;

    // Write solution file
    {
        let mut file = std::fs::File::create(format!("src/{filename}"))?;
        file.write_all("#![allow(dead_code)]\npub struct Solution {}\n".as_bytes())?;
        file.write_all(snippet.code.as_bytes())?;
        file.write_all("\n#[cfg(test)]\nmod tests {\n    use super::*;\n".as_bytes())?;
        file.write_all("}\n".as_bytes())?;
        file.sync_all()?;
    }

    // Add solution module to lib.rs
    {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("src/lib.rs")?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let mod_name = format!(
            "p{:04}_{}",
            question_id,
            question.title_slug.replace('-', "_")
        );
        if !content.contains(&format!("mod {mod_name};")) {
            file.write_all(format!("mod {mod_name};\n").as_bytes())?;
            file.sync_all()?;
        }
    }

    println!("Solution created: {filename}");
    println!(
        "Problem link: https://leetcode.com/problems/{}/",
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
