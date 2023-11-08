# hyprtxt

## Concept

A simple static site generator. I was inspired to make this after reading [hypermedia.systems](https://hypermedia.systems).

The main feature is the procedural macro in `/txt`, which implements a templating language which renders into a stringified HTML node.

### Example

```
hyprtxt!(
    div {
        .="content-root"
        "tag"
        $: "content"
    }
)
```
