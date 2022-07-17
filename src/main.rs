mod shell;
mod cmus;

fn main() {
    let cmus = cmus::CmusQuery::new();

    if !cmus.remote.is_empty() {
        println!("{} status: {}", cmus.status_symbol.unwrap(), cmus.status.unwrap());
        println!("file: {}", cmus.file.unwrap());
        println!("duration: {}", cmus.duration.unwrap());
        println!("position: {}", cmus.position.unwrap());
        println!("time left: -{}", cmus.time_left.unwrap());
        println!("repeat: {}", cmus.repeat.unwrap());
    }
    
}
