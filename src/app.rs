// Импортируем необходимые модули и типы
use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

// Определяем функцию `invoke` из пространства имен Tauri в JavaScript
// Эта функция позволяет вызывать команды на стороне сервера
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// Определяем структуру для аргументов функции `greet`
// Эта структура будет использоваться для сериализации данных перед отправкой их на сервер
#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

// Определяем компонент `App` для нашего приложения
#[component]
pub fn App() -> impl IntoView {
    // Создаем сигналы для имени и сообщения приветствия
    // Сигналы позволяют нам отслеживать изменения состояния и обновлять интерфейс пользователя
    let (name, set_name) = create_signal(String::new());
    let (greet_msg, set_greet_msg) = create_signal(String::new());

    // Определяем обработчик событий для обновления имени
    // Когда пользователь вводит имя, мы обновляем сигнал `name` новым значением
    let update_name = move |ev| {
        let v = event_target_value(&ev); // Получаем текущее значение текстового поля
        set_name.set(v); // Обновляем сигнал `name` новым значением
    };

    // Определяем обработчик событий для отправки приветствия
    // Когда пользователь нажимает кнопку "Greet", мы вызываем команду `greet` на сервере
    let greet = move |ev: SubmitEvent| {
        ev.prevent_default(); // Предотвращаем стандартное поведение формы
        spawn_local(async move { // Запускаем асинхронную задачу
            let name = name.get_untracked(); // Получаем текущее значение сигнала `name` без его отслеживания
            // Мы используем `get_untracked` здесь, чтобы избежать нежелательной перерисовки интерфейса пользователя, если имя изменится во время выполнения асинхронного замыкания.
            if name.is_empty() {
                return;
            }

            // Сериализуем аргументы и вызываем команду `greet`
            let args = to_value(&GreetArgs { name: &name }).unwrap();
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg); // Обновляем сигнал `greet_msg` новым сообщением
        });
    };

    // Возвращаем представление нашего приложения
    // Здесь мы используем сигналы `name` и `greet_msg` для обновления интерфейса пользователя
    view! {
        <h1 class="text-red-500 dark:bg-orange-300">"если этот текст красный, значит tailwind сработал"</h1>
        <main class="container bg-orange-200">
            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="assets/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://docs.rs/leptos/" target="_blank">
                    <img src="assets/leptos.svg" class="leptos logo" alt="Leptos logo"/>
                </a>
            </div>

            <p>"Click on the Tauri and Leptos logos to learn more."</p>

            <form class="row" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="Enter a name..."
                    on:input=update_name
                />
                <button type="submit">"Greet"</button>
            </form>

            <p><b>{ move || greet_msg.get() }</b></p> // Отображаем текущее значение сигнала `greet_msg`
        </main>
    }
}
