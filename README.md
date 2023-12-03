# hyprtxt

## Concept

A simple HTML templating proc macro. I was inspired to make this after reading [hypermedia.systems](https://hypermedia.systems).

### Example

```
use hyprtxt::hyprtxt;

hyprtxt!(
    "div" {
        "class"="content-root"
        "tag"
        $: "content"
        "p" {
            "id"="nested"
            $: "nested content"
            $: "nested content 2"
        }
    }
)
```
