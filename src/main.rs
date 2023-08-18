fn main() {

     let resp = reqwest::blocking::get("https://api.ipify.org");

     match resp {

        Ok(r) => {
            let ip = r.text().unwrap();
            println!("{}", &ip);
        }
        
        Err(_) => {
            println!("network error");
        }

     }

}
