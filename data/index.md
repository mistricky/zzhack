--
title: Data datalog
description: Root directory includes all demo files and images 
tags: root, demo, documentation
--

Here is a **complete English Markdown test document** that covers **all commonly used Markdown syntax**, including GitHub-flavored Markdown (GFM).
You can copy it directly and use it for renderer testing.

---

# Markdown Test Document

*A comprehensive test of all Markdown syntax (CommonMark + GFM).*

---

## 1. Headings

# Heading 1

## Heading 2

### Heading 3

#### Heading 4

##### Heading 5

###### Heading 6

---

## 2. Paragraphs & Line Breaks

This is a paragraph with two lines.
Here is the second line after a hard break.

This is another paragraph separated by a blank line.

---

## 3. Emphasis

**Bold text**
*Italic text*
***Bold and italic***
~~Strikethrough~~ <ins>Underlined text (HTML)</ins>

---

## 4. Blockquotes

> This is a blockquote.
>
> > Nested blockquote.
> >
> > Another level.

---

## 5. Lists

### Unordered list

* Item A
* Item B

  * Subitem B1
  * Subitem B2

    * Sub-subitem

### Ordered list

1. First
2. Second
3. Third

   1. Nested item
   2. Nested item

### Mixed list

* Item

  1. Subitem
  2. Subitem
* Item

---

## 6. Code

### Inline code

Use `npm install` or `cargo build`.

### Code blocks

```bash
# Bash example
echo "Hello world"
ls -al /usr/local
```

```javascript
// JavaScript example
function add(a, b) {
  return a + b;
}
console.log(add(2, 3));
```

```rust
// Rust example
fn main() {
    println!("Hello, Rust!");
}
```

```json
{
  "name": "example",
  "version": "1.0.0"
}
```

---

## 7. Horizontal Rules

---

---

---

## 8. Links

[Inline link](https://example.com)

[Reference link][ref]

[ref]: https://example.com "Optional title"

[https://example.com](https://example.com) ← auto-link

---

## 9. Images

![Alt text](/data/COVER.png)

Reference image:

![Placeholder][imgref]

[imgref]: /data/img.png 

---

## 10. Tables (GFM)

| Name  | Age | Role      |
| ----- | --- | --------- |
| Alice | 24  | Developer |
| Bob   | 31  | Designer  |
| Eve   | 28  | Manager   |

### Alignment

| Left | Center | Right |
| :--- | :----: | ----: |
| A    |    B   |     C |
| D    |    E   |     F |

---

## 11. Task Lists (GFM)

* [x] Write documentation
* [ ] Add more tests
* [ ] Fix rendering issues

---

## 12. Footnotes (GFM)

This is a sentence with a footnote.[^1]

Another footnote reference.[^note]

[^1]: This is the first footnote.

[^note]: This is another footnote.

---

## 13. Definition Lists (GFM extension)

Term 1
: Definition of term 1

Term 2
: Definition of term 2
: Another definition

---

## 14. Emoji (GFM)

Supported GitHub emojis:

:smile: :tada: :rocket: :+1:

---

## 15. HTML inside Markdown

<div style="border:1px solid #ccc;padding:8px;">
  <strong>HTML block</strong><br />
  Works inside Markdown.
</div>

---

## 16. Code Fence Highlight Test

```
No language → treated as plain text.
```

---

## 17. Escaping Characters

*Literal asterisks*
(Parentheses)
_Underscores_

---

## 18. Math (If renderer supports KaTeX/MathJax)

Inline math: $( E = mc^2 )$

Block math:

$$
\int_0^\infty e^{-x^2} , dx = \frac{\sqrt\pi}{2}
$$

---

# End of Document

If you want a **much bigger version**, or a **Chinese + English bilingual test**, or a **GitHub README-optimized version**, tell me!

