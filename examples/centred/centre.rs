              pub fn centre(code: String) -> String {              
           let code: Vec<&str> = code.lines().collect();           
                                                                    
   let line_width = code.iter().map(|l| l.len()).max().unwrap();   
                                                                    
                            code.iter()                            
                             .map(|l| {                             
                         let l = l.trim();                         
      let whitespace = " ".repeat((line_width - l.len()) / 2);      
            format!("{}{}{}", whitespace, l, whitespace)            
                                 })                                 
                     .collect::<Vec<String>>()                     
                            .join("\n")                            
                                 }                                 