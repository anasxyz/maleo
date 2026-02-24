<p align="center">
  <img width="200" height="300" alt="bento" src="https://github.com/user-attachments/assets/378d298a-3cd3-4e5b-a17b-ffd6e22b0538" />
</p>
<h1 align="center">Bento</h1> 
<p align="center">cross platform rust GUI framework with Elm style architecture</p>

## Features

* Simplified API
* Cross-platform 
* GPU accelerated rendering
* Elm inspired architecture
* Flexbox layout engine
* Composable styling (colors, borders, border radius, opacity, shadows, and more)
* Async task system (futures, background threads, delays, repeating intervals, exclusive tasks, and timeouts)
* Builtin widget library
* Font loading and management
* Keyboard, mouse, and window event handling

## Examples
```rust
        column(vec![
            text("Hello, Bento!", Color::hex("#ffffff"))
                .font_size(24.0)
                .font_weight(700),
            text_input("name")
                .value(&self.name)
                .placeholder("Enter your name...")
                .on_change(|v| Action::UpdateName(v))
                .width(px(300.0)),
            button("Submit")
                .on_click(Action::Submit)
                .background(Color::hex("#2563eb"))
                .border_radius(6.0),
        ])
        .padding(Edges::all(32.0))
        .gap(12.0)
```

![Demo3](screenshots/demo3.gif)

<img width="1847" height="1013" alt="image" src="https://github.com/user-attachments/assets/af5d9930-a7f0-4e57-8d42-3ee90cabf231" />

![Demo](screenshots/demo.gif)
