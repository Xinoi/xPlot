
struct TokenTree {
    mid: Token, 
    left: Box<Option<TokenTree>>, 
    right: Box<Option<TokenTree>>, 

}
