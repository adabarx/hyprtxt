
fn main() {
    let x = hyprtxt::hyprtxt!(
        html {
            lang="en"
            head {
                meta* { ifor="got" }
                title: "this is a test"
            }
            body {
                div {
                    .="class"
                    #="id"
                    type="something"
                    p: "paragraph"
                    $: "moar stuff"
                }
            }
        }
    );
    dbg!(x);
}
