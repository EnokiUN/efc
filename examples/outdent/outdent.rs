                         pub fn outdent(code: String) -> String {
                let code: Vec<&str> = code.lines().collect();    
                                                                 
let line_width = code.iter().map(|l| l.len()).max().unwrap();    
                                                                 
                                                  code.iter()    
                                               .map(|l| {        
     let indent_len = l.len() - l.trim_start().len();            
                              let l = l.trim_start();            
                                             format!(            
                                        "{}{}{}",                
   " ".repeat(line_width - l.len() - indent_len),                
                                               l,                
                           " ".repeat(indent_len)                
                                                    )            
                                                       })        
                                .collect::<Vec<String>>()        
                                              .join("\n")        
                                                                }