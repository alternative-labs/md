# 🦀 Rust markdown parser
## This is a **sample** markdown `text`.
### Header 3
#### Header 4
##### Header 5
###### Header 6
####### Not a header
#Not a header

We are using _Rust_ to tokenize, parse *Markdown* AST
And this is supposed to be a single paragraph block.

But this one is **NEW BLOCK**.\r\nAnd it continues\rSpanning for three lines.

> Quote block  
> can span  
> multiple lines

```js
// this is a code block too
function main() {
    console.log("this entire block should be rendered as is");
}
```
- list item 1
- list item 2
-not a list item

> __More__ *advanced* `variations`:
> ```ts
> const canHaveNestedBlocks: boolean = true;
> ```
> # Also, renders heading
> ```
> should end code block if parent quoteblock ends
```
this actually starts a new codeblock.
