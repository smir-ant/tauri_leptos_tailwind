# Пара слов про стек:
- Tauri - rust фреймворк для создания кроссплатформенных приложений
- Leptos - фронтенд фреймворк, где у тебя rust вместо js
- Tailwind - где-то между vanila css и bootstrap, хотя сам сперва не любил, но переобулся поработав с ним.

<br>

---

<br>

# Как подтянуть Tailwind если вы не хотите node.js 

https://tailwindcss.com/blog/standalone-cli

От него нам нужно только `tailwind.config.js`(его в корневую папку положить), если что.

Далее в css запихнуть это:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

И в html вставить (обрати внимание, что это не стулещит):
```html
<link data-trunk rel="tailwind-css" href="css/tailwind.css" />
```

<br>

---

<br>

# Остальные отличия от стандартного шаблона tauri+leptos:

### 1) перенесен html из корневой папки в src:<br> 
https://stackoverflow.com/questions/76740053/in-yew-how-is-it-possible-to-locate-index-html-in-a-static-directory
<br>
добавлен hfref в html:
```html
<link data-trunk rel="rust" data-wasm-opt="z" href="../Cargo.toml"/>
```
trunk.toml:
```toml
[build]
target = "./src/page.html"
```


### 2) css и assets(бывшая public) перенесены в папку src

### 3) ну и да, прикручен tailwind
Он при сборке формируется и оптимизируется, оказываясь потом в dist рядом с html+css+js+wasm. и подключается легко - за это leptos реал респект. [Тут в примерах](https://github.com/leptos-rs/leptos/tree/main/examples)  у них ещё с `actix` и `axum` есть.

<br>

---
<sup>p.s. сделал шаблон для себя, чтобы потом это не искать, но если понравилось/пригодилось, то звездочку хочу<br><br>делалось на бете tauri 2.0; версии видно в toml, дата = коммит</sup>

