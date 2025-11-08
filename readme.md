# JunStr

Unnest json serialized as strings inside json.

Sample input:
```
{
  "a": 1,
  "b": "string",
  "c": "2",
  "d": [1, 2, 3],
  "e": "[1, 2, 3]",
  "f": "[1, \"string\", \"3\"]",
  "f": { "a": 1, "b": "257" },
  "g": "{ \"a\": 1, \"b\": \"257\" }",
  "h": "{ \"a\": 1, \"b\": \"257\""
}
```

Sample output:
```json
{
  "a": 1,
  "b": "string",
  "c": 2,
  "d": [
    1,
    2,
    3
  ],
  "e": [
    1,
    2,
    3
  ],
  "f": {
    "a": 1,
    "b": 257
  },
  "g": {
    "a": 1,
    "b": 257
  },
  "h": "{ \"a\": 1, \"b\": \"257\""
}
```
