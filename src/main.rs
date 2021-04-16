
use futures::prelude::*;
use futures::future::{join_all, ok, err};
use std::{error::Error, fs::File};
use std::io::prelude::*;

#[tokio::main]
async fn main()->Result<(),Box<dyn Error>> {
    let mut tasks=vec![];
    tasks.push(download("https://cn.bing.com/search?form=MOZLBR&pc=MOZI&q=reqwest+download+file","1.html"));
    tasks.push(download("https://cn.bing.com/search?form=MOZLBR&pc=MOZI&q=reqwest+download+file","2.html"));
    tasks.push(download("https://cn.bing.com/search?form=MOZLBR&pc=MOZI&q=reqwest+download+file","3.html"));
    let _=join_all(tasks).await;
    Ok(())
}

async fn download(url:&str,name:&str) -> Result<(), Box<dyn Error>> {
    let resp = reqwest::get(url).await?;
    let bytes = resp.bytes().await?;
    let mut out = tokio::fs::File::create(name).await?;
    tokio::io::copy(&mut &*bytes, &mut out).await?;
    Ok(())
}