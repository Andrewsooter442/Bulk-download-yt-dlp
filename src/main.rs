
use std::env;
use std::fs::read_to_string;
use std::io;
use std::fs;
use std::process;
use std::process::Command;

fn get_links(path:&str) -> Vec<String> {
    let links = std::fs:: read_to_string(path);
    let links: Vec<String> = match links {
        Ok(message) => {
            let links: Vec<String> = message.lines().map(String::from).collect();
               links
        },
        Err(_) => panic!("Can't read the links file make sure the formate is correct"),
        
    };
    links

}

fn download(links:&Vec<String> , to_store:&str) {
    let mut output_file = String::new();
    let mut command = Command::new("yt-dlp");
    if to_store == "\n" {
         match env::current_dir(){
            Ok(message) => output_file= message.to_string_lossy().to_string(),
            Err(err) =>panic!("problem getting the pwd {}", err),
         }
    }
    else {
        output_file = to_store.trim().to_string();
    }
    for https in links {
        command.arg(format! ("{}",https))
            .arg("-o")
            .arg(format!("{}",output_file));
        let status = command.output().expect("Failed to download");
        println!( "{:?}",status);
    }

    
    
}

fn main() {

    //get the path to the links directory
    let mut links_path = String::new();
    std::io::stdin().read_line(&mut links_path)
    .expect("file path not provided");
    links_path  = links_path.trim().to_string();


    // get's the path to the output file can be left empty
    let mut destination_path = String::new();
    std::io::stdin().read_line(&mut destination_path)
    .expect("msg");


     let test = get_links( &links_path);
     println! (" {:?}", test);

  //
  
    download(&test,&destination_path )


}
