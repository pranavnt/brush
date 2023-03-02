/* {
    type: "KEYWORD",
    value: "const"
}
---
{
    type: "OPERATOR",
    value: "+"
}
---
{
    type: "NUMBER",
    value: "2"
}*/

pub fn code_to_token(input: String) {
    let mut lines = input.split("\n");
    println!("{:#?}", lines.next());
}