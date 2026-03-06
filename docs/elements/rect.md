# Rectangle `rect()`
A plain rectangle with no children.

## Properties
> For all property info, see [Element Properties](../element_props.md)

### Sizing
`.w()` `.h()` `.min_w()` `.min_h()` `.max_w()` `.max_h()` `.aspect_ratio()`   

### Spacing
`.p()` `.pt()` `.pr()` `.pb()` `.pl()` `.px()` `.py()`   
`.m()` `.mt()` `.mr()` `.mb()` `.ml()` `.mx()` `.my()`   

### Layout
`.grow()` `.shrink()` `.basis()` `.align_self()`   

### Position
`.absolute()` `.relative()` `.top()` `.right()` `.bottom()` `.left()`   

### Visual
`.bg()` `.border()` `.border_color()` `.border_radius()` `.opacity()` `.z_index()` `.hide()` `.show()`   

## Example
```rust
rect()
    .w(Size::Fixed(100.0))
    .h(Size::Fixed(100.0))
    .min_w(Size::Fixed(0.0))
    .min_h(Size::Fixed(0.0))
    .max_w(Size::Fixed(500.0))
    .max_h(Size::Fixed(500.0))
    .aspect_ratio(1.0)
    .p([0.0, 0.0, 0.0, 0.0])
    .pt(0.0)
    .pr(0.0)
    .pb(0.0)
    .pl(0.0)
    .px(0.0)
    .py(0.0)
    .m([0.0, 0.0, 0.0, 0.0])
    .mt(0.0)
    .mr(0.0)
    .mb(0.0)
    .ml(0.0)
    .mx(0.0)
    .my(0.0)
    .grow(0.0)
    .shrink(1.0)
    .basis(Size::Auto)
    .align_self(AlignSelf::Auto)
    .absolute()
    .relative()
    .top(Size::Fixed(0.0))
    .right(Size::Fixed(0.0))
    .bottom(Size::Fixed(0.0))
    .left(Size::Fixed(0.0))
    .bg(Color::WHITE)
    .border(0.0)
    .border_color(Color::BLACK)
    .border_radius(0.0)
    .opacity(1.0)
    .z_index(0)
    .hide()
    .show()
```
