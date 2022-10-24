A code snippet that can transform something like
```
[
    {
        "first.0":"foo",
        "first.1":"bar",
        "second": {
            "third.0": "foo",
            "third.1": "bar"
        }
    }
}
```

to

```
[
    {
        "first":{
            "0": "foo",
            "1": "bar"
        },
        "second": {
            "third": {
                "0": "foo",
                "1": "bar"
            }
        }
    }
]
```

I was inspired to do this by [A Reddit Thread on r/rust](https://www.reddit.com/r/rust/comments/yck5pq/help_optimize_painfully_slow_rust_code_ported/)