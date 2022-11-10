                                                                                              use anyhow::anyhow;
                                                                                        use anyhow::{Ok, Result};
                                                                   use efc::{Formatter, Params, FORMATTER_NAMES};
                                                                                   use std::{env, path::PathBuf};
                                                                                                                 
                                                                                        fn main() -> Result<()> {
                                                                          let mut args = env::args().skip(1);    
                                                                          let formatter = match args.next() {    
                                             Some(formatter) => match formatter.to_lowercase().as_ref() {        
                                                  "c" | "centre" | "center" => Ok(Formatter::Centre),            
                                                           "o" | "outdent" => Ok(Formatter::Outdent),            
                                                                                 "-h" | "--help" => {            
                                                                                     send_help();                
                                                                                   return Ok(());                
                                                                                                    }            
                                                              _ => Err(anyhow!("Invalid formatter")),            
                                                                                                       },        
                                                                                                None => {        
                                                                                         send_help();            
                                                                                       return Ok(());            
                                                                                                        }        
                                                                                                          }?;    
                                                                                    let mut params = Params {    
                                                                                       files: Vec::new(),        
                                                                                               formatter,        
                                                                                            write: false,        
                                                                                                           };    
                                                                                          args.for_each(|a| {    
                                                                               // Currently the only flag        
                                                                                           if a == "-w" {        
                                                                                  params.write = true            
                                                                                                 } else {        
                                                                  params.files.push(PathBuf::from(a))            
                                                                                                        }        
                                                                                                          });    
                                                                                                                 
                                                                                        efc::format(params)?;    
                                                                                                                 
                                                                                                       Ok(())    
                                                                                                                }
                                                                                                                 
                                                                                                 fn send_help() {
                                                                                                    println!(    
"EFC - A collection of next generation code formatters.\nformatters: {}\nUsage: efc <formatter> <files>",        
                      FORMATTER_NAMES.iter().map(|f|f.to_lowercase()).collect::<Vec<String>>().join(", ")        
                                                                                                           );    
                                                                                                                }