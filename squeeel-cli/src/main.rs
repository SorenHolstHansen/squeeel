use serde::Deserialize;
use simple_logger::SimpleLogger;
use squeeel_cli::AstVisitor;
use squeeel_cli::Query;
use squeeel_cli::SupportedLib;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use swc_common::SourceMap;
use swc_common::sync::Lrc;
use swc_ecma_parser::TsSyntax;
use swc_ecma_parser::{Parser, StringInput, Syntax, lexer::Lexer};

fn find_package_json_dir(from_dir: &Path) -> anyhow::Result<&Path> {
    let mut dir = from_dir;

    while !std::fs::exists(dir.join("package.json"))? {
        let Some(new_dir) = dir.parent() else {
            return Err(anyhow::anyhow!("Could not find the root of the package"));
        };
        dir = new_dir;
    }

    Ok(dir)
}

#[derive(Deserialize)]
struct PackageJson {
    dependencies: HashMap<String, serde_json::Value>,
}

fn detect_sql_libs_in_package_json(package_json_path: &Path) -> anyhow::Result<Vec<SupportedLib>> {
    let content = std::fs::read_to_string(package_json_path)?;

    let package_json: PackageJson = serde_json::from_str(&content)?;
    let libs: Vec<_> = package_json
        .dependencies
        .into_keys()
        .filter_map(|lib| SupportedLib::try_from(lib).ok())
        .collect();

    Ok(libs)
}

fn main() -> anyhow::Result<()> {
    SimpleLogger::new().init().unwrap();
    let vars = env::args().skip(1).collect::<Vec<_>>();
    let dir = match vars.first() {
        Some(dir) => PathBuf::from_str(dir)?,
        None => std::env::current_dir()?,
    };
    log::info!("Detecting package root");
    let root_dir = find_package_json_dir(&dir)?;
    log::info!("Found package root located at {:?}", root_dir);
    let sql_libs = detect_sql_libs_in_package_json(&root_dir.join("package.json"))?;
    if sql_libs.is_empty() {
        return Err(anyhow::anyhow!("Did not detect any libs"));
    }
    log::info!(
        "Detected the following libs: {}",
        sql_libs
            .iter()
            .map(|lib| lib.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );

    let queries = detect_queries(&dir, sql_libs);
    let queries_by_lib: HashMap<SupportedLib, Vec<String>> =
        queries.into_iter().fold(HashMap::new(), |mut acc, query| {
            acc.entry(query.lib).or_default().push(query.query);
            acc
        });

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { create_d_ts_files(root_dir, queries_by_lib).await });

    Ok(())
}

fn detect_queries(dir: &Path, supported_libs: Vec<SupportedLib>) -> Vec<Query> {
    let supported_libs = Arc::new(supported_libs);
    let mut handles = Vec::new();
    for entry in walkdir::WalkDir::new(dir) {
        let entry = entry.unwrap();
        if entry.path().to_string_lossy().contains("node_modules")
            || entry.path().to_string_lossy().ends_with(".d.ts")
        {
            continue;
        };
        if !entry.path().to_string_lossy().ends_with(".ts")
            && !entry.path().to_string_lossy().ends_with(".tsx")
        {
            continue;
        }

        let supported_libs = supported_libs.clone();
        handles.push(std::thread::spawn(move || {
            let cm: Lrc<SourceMap> = Default::default();

            let fm = cm.load_file(entry.path()).unwrap();

            let lexer = Lexer::new(
                Syntax::Typescript(TsSyntax {
                    no_early_errors: true,
                    tsx: entry.path().to_string_lossy().ends_with(".tsx"),
                    ..Default::default()
                }),
                Default::default(),
                StringInput::from(&*fm),
                None,
            );

            let mut parser = Parser::new_from(lexer);

            let module = parser.parse_typescript_module().unwrap();
            let mut ast_visitor = AstVisitor::new(&supported_libs);
            ast_visitor.visit(&module);
            ast_visitor.statements()
        }));
    }

    handles
        .into_iter()
        .flat_map(|handle| handle.join().unwrap())
        .collect()
}

async fn create_d_ts_files(dir: &Path, queries_by_lib: HashMap<SupportedLib, Vec<String>>) {
    let mut tasks = Vec::with_capacity(queries_by_lib.keys().len());
    for (lib, queries) in queries_by_lib {
        tasks.push(tokio::spawn({
            async move { lib.create_d_ts_file(queries).await }
        }));
    }

    let mut outputs = Vec::with_capacity(tasks.len());
    for task in tasks {
        outputs.push(task.await.unwrap());
    }
    let d_ts_path = dir.join("src/squeeel.d.ts");
    std::fs::write(d_ts_path, outputs.join("\n\n")).unwrap();
}
