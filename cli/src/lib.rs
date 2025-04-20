
use std::error::Error;
use std::{env, fs, string, vec};

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
   let file = fs::read_to_string(config.file_name.clone())?;

  let results =  if config.case_sensetive {
       search(&file, &config.word)
   }else {
       search_case_insensetive(&file, &config.word)
   };

   for i in results{
        print!("{} \n", i);
   }
    // println!("{:?}", file);
    Ok(())
    
}

#[derive(Clone)]
pub struct Config {
    pub word: String,
    pub file_name: String,
    pub case_sensetive:bool
}

impl Config {
    pub fn new(config_str: &[String]) -> Result<Config, &str> {
        if config_str.len() < 3 {
            return Err("NOT ENOUGH ARUMENTS.");
        }

        let word = config_str[1].clone();
        let file_name = config_str[2].clone();
        
        let case_sensetive = env::var("CASE_INSENSETIVE").is_err();
        Ok(Config { word, file_name, case_sensetive })
    }
}


pub fn search<'a>(doc: &'a str, word: &'a str)->Vec<&'a str>{
    let mut result_lines= Vec::new();

    for i in doc.lines(){
        if i.contains(word){
            result_lines.push(i);
        }
    }
    result_lines

}
pub fn search_case_insensetive <'a>(doc: &'a str, word: &'a str)->Vec<&'a str>{
    let mut result_lines= Vec::new();

    for i in doc.lines(){
        if i.to_lowercase().contains(&word.to_lowercase()){
            result_lines.push(i);
        }
    }
    result_lines

}



#[cfg(test)]
mod tests{
    #[test]
    fn test(){


    }
}