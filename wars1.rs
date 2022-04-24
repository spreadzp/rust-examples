fn rgb(r: i32, g: i32, b: i32) -> String {
  format!("{}{}{}",convert(r),convert(g),convert(b))
}

fn convert(mut dec: i32) -> String {
    let cash = dec;
    let mut result = String::from(""); 
    if dec <0 {return String::from("00")} 
    if dec > 255 {dec = 255;}
    let s: String = String::from("0123456789ABCDEF");  
  	loop {
  	   
        let quotient = dec /16; 
		let index = (dec - 16 * quotient) as usize; 
        result = format!("{}{}", s.chars().nth(index.try_into().unwrap()).unwrap(), result);
      	if quotient < 1 {
        	break;
        }
        dec = quotient;
    }
    if cash <=15 {result = format!("{}{}","0".to_string(), result);}
  	result.to_string()
}


fn first_word(s: &str) -> Vec<String> {  
    let mut pasrsed_string = String::from(s); 
    println!("{}", s.len());
    if s.len()%2 != 0 {
    println!("{}", s.len());
         pasrsed_string.push('_');
         println!("{:?} ", pasrsed_string);
    }  
     let mut v = Vec::new();
    let mut n = String::from("");
        for (i,ch) in pasrsed_string.chars().enumerate() {
            n.push(ch);
            if (i+1)%2 == 0 {
                println!("{} {}", i, n);
                v.push(n);
                n = String::from("");
            }
        }
        v
    }
    
    
    fn main() {
        let mut s = String::from("hello worldr4");
    
        let _word = first_word(&s); // word will get the value 5
        println!("{:?} ", _word);
        s.clear(); // this empties the String, making it equal to ""
    
        // word still has the value 5 here, but there's no more string that
        // we could meaningfully use the value 5 with. word is now totally invalid!
    }
