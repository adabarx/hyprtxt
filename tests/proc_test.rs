#[test]
fn readme_example() {
    let x = hyprtxt::hyprtxt!(
        "html" {
            "lang"="en"
            "head" {
                "title" { $: "this is a test" }
                "meta"* { "dum"="brane" }
            }
            "body" {
                "div" {
                    "class"="class"
                    "id"="id"
                    "type"="idk"

                    "p"  { $: "paragraph" }
                    $: "stuff"
                    $: "moar stuff"
                }
            }
        }
    );
    assert_eq!(x, "<html lang=\"en\"><head><title>this is a test</title><meta dum=\"brane\"></head><body><div class=\"class\" id=\"id\" type=\"idk\"><p>paragraph</p>stuffmoar stuff</div></body></html>".to_string())
}
