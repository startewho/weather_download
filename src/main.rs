use clap::{App, Arg};
use futures::{future::join_all};
use futures::prelude::*;
use std::env;
use std::{fs};
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::{
    error::Error,
    fs::File,
    path::{Path, PathBuf},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("WeatherDownload")
        .version("1.0")
        .author("JustCheng<hoolooday@live.com>")
        .about("Download Weahter Data")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Sets a input file cotions urls")
                .default_value("input")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output path dir")
                .help("Sets the output file to use")
                .default_value("out")
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    let mut tasks = vec![];

    let input = matches.value_of("input").unwrap_or("input");
    let cur_dir = env::current_dir()?;
    let output: PathBuf = Path::new(&cur_dir).join(matches.value_of("ouput").unwrap_or("output"));
    fs::create_dir(&output)?;
    println!("output path:{:?}",output.as_os_str());

    let file = File::open(input).unwrap();
    let mut i = 1;
    let reader = BufReader::new(file);
    reader.lines().for_each(|s| {
        if s.is_ok() {
            let name = format!(r#"{}.html"#, &i);
            let path = output.clone().join(name);
            println!("download path:{:?}",path.as_os_str());
            tasks.push(download(s.unwrap(), path));
            i = i + 1;
        }
    });

    let resuts = join_all(tasks).await;
    resuts.into_iter().for_each(|r|
          if r.is_err(){
               println!("{:?}",r.unwrap_err());
          });
    Ok(())
}

async fn download(url: String, name: PathBuf) -> Result<(), Box<dyn Error>> {
    let resp = reqwest::get(&url).await?;
    let bytes = resp.bytes().await?;

    let mut out = tokio::fs::File::create(&name).await?;
    tokio::io::copy(&mut &*bytes, &mut out).await?;
    Ok(())
}
