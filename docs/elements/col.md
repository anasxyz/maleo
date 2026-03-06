# Column `col()`
A column is a vertical stack of child elements. It lays out children top to bottom and can be nested inside other columns and rows.

## Properties
> For all property info, see [Element Properties](../element_props.md)

### Sizing
`.w()` `.h()` `.min_w()` `.min_h()` `.max_w()` `.max_h()` `.aspect_ratio()`   

### Spacing
`.p()` `.pt()` `.pr()` `.pb()` `.pl()` `.px()` `.py()`   
`.m()` `.mt()` `.mr()` `.mb()` `.ml()` `.mx()` `.my()`   
`.gap()` `.row_gap()` `.col_gap()`   

### Flex Container
`.align_items()` `.justify_content()` `.flex_wrap()` `.flex_direction()`   
### Layout
`.grow()` `.shrink()` `.basis()` `.align_self()`   
### Position
`.absolute()` `.relative()` `.top()` `.right()` `.bottom()` `.left()`   
### Overflow
`.overflow()` `.overflow_x()` `.overflow_y()`   
### Visual
`.opacity()` `.z_index()` `.hide()` `.show()`   
## Example
```rust
col(vec![
    rect().w(Size::Percent(100.0)).h(Size::Fixed(50.0)).bg(Color::RED),
    rect().w(Size::Percent(100.0)).grow(1.0).bg(Color::GREEN),
])
    .w(Size::Fixed(300.0))
    .h(Size::Percent(100.0))
    .p([10.0, 10.0, 10.0, 10.0])
    .gap(8.0)
    .align_items(AlignItems::Stretch)
    .justify_content(JustifyContent::Start)
    .flex_wrap(FlexWrap::NoWrap)
    .overflow(Overflow::Hidden)
    .opacity(1.0)
    .z_index(0)
```
