# Quiz app

Markdown style quiz generator. A custom Markdown language that includes custom commands for creating HTML.

### How to run
~~~bash
cargo run <input file> <output directory>
~~~

### Example

```markdown
# Quiz

Litt teori om Uncertainty...

[quiz
? P(A,B)=P(A|B)*P(B)
- ja
+ nei
]
[quiz
? KAtter
+ ja
- nei
+ kanskje
]
```

![](docs/Skjermbilde%202023-05-28%20kl.%2014.25.00.png)
