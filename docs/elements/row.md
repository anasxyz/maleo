# Row `row()`
A row is a horizontal stack of child elements. It lays out children left to right and can be nested inside other columns and rows.

## Properties
> For all property info, see [Element Properties](../element_props.md)

### Sizing
`.w()` `.h()` `.min_w()` `.min_h()` `.max_w()` `.max_h()` `.aspect_ratio()`   

### Spacing
`.p()` `.pt()` `.pr()` `.pb()` `.pl()` `.px()` `.py()`   
`.m()` `.mt()` `.mr()` `.mb()` `.ml()` `.mx()` `.my()`   
`.gap()` `.row_gap()` `.col_gap()`   

### Flex
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
row(vec![
    rect().w(Size::Fixed(100.0)).h(Size::Fixed(100.0)).bg(Color::RED),
    rect().grow(1.0).h(Size::Fixed(100.0)).bg(Color::GREEN),
])
    .w(Size::Percent(100.0))
    .h(Size::Fixed(100.0))
    .p([10.0, 10.0, 10.0, 10.0])
    .gap(8.0)
    .align_items(AlignItems::Center)
    .justify_content(JustifyContent::SpaceBetween)
    .flex_wrap(FlexWrap::NoWrap)
    .overflow(Overflow::Hidden)
    .opacity(1.0)
    .z_index(0)
```
