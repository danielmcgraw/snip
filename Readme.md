##### Text snippets on the command line. Quickly create topics and store snippets for easy recall and reuse.

### Installation
```
> cargo install snip
``` 

### Example
```
> snip put rust
// rust topic created

> snip rust docs https://docs.rs/
< Entry inserted in to rust

> snip put rust main https://www.rust-lang.org/
< Entry inserted in to rust

> snip show rust
< main: https://www.rust-lang.org/
< docs: https://docs.rs/

> snip show rust docs
< https://docs.rs/

> snip get rust docs
// docs value copied to your clipboard use with ctrl-v
```

### Commands
#### Create
Create a topic
```
> snip put <topic_name>
```

Create a topic snippet
```
> snip put <topic_name> <snippet_key> <snippet_value>
```

#### Get
Get a specific snippet from a topic (copied in to your clipboard)
```
> snip get <topic_name> <snippet_key>
```

#### Print
Print all snippets for all topics
```
> snip show all
```

Print all snippets for a specific topic
```
> snip show <topic_name>
```

Print a specific snippet from a specific topic
```
> snip show <topic_name> <snippet_key>
```

#### Delete
Delete a topic and all its snippets
```
> snip del <topic_name>
```

Delete a specific snippet from a topic
```
> snip del <topic_name> <snippet_key>
```

Nuke all the topics and their contents (delete everything)
```
> snip nuke
```
