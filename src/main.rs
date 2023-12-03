
fn main() {
    let x = hyprtxt::hyprtxt!(
        "html" {
            "lang"="en"
            "head" {
                "title" { $: "this is a test" }
                "meta"* { "ifor"="got" }
            }
            "body" {
                "div" {
                    "class"="class"
                    "id"="id"
                    "type"="something"
                    "p"  { $: "paragraph" }
                    $: "moar stuff"
                }
            }
        }
    );
    dbg!(x);
}
