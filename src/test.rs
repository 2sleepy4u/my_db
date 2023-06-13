#[derive(Debug)]
struct Token {
    token_type: TokenType,
    value: String,
    start_pos: i32
}

impl Default for Token {
    fn default() -> Self {
        Token { 
            token_type: TokenType::Empty,
            value: "".to_string()
        }
    }
}

#[derive(Debug)]
enum TokenType {
    Error,
    Num,
    Plus,
    Minus,
    Mul,
    Div,
    LParenthesis,
    RParenthesis,
    LCurlyParen,
    RCurlyParent
}

fn is_num(input: &str) -> bool {
    let test = String::from(input).parse::<f64>();

    match test {
        Ok(ok) => true,
        Err(e) => false
    }
}

fn get_token_type(input: &str) -> TokenType {
    if is_num(input) { TokenType::Num } else 
    if input == "+" { TokenType::Plus } else 
    if input == "-" { TokenType::Minus } else 
    if input == "*" { TokenType::Mul } else 
    if input == "/" { TokenType::Div } else 
    { TokenType::Error }
}

fn lexer(input: String) -> Vec<Token> {
   let mut list: Vec<Token> =  vec![]; 
   for word in input.split(" ") {
       let token: Token = Token {
           value: word.to_string(), 
           token_type: get_token_type(word)
       } ;
       list.push(token);
   }
   list
}

fn main() {
    let list = lexer("1 + 1".to_string());
    for word in list {
        println!("{:?}", word);
    }
}
