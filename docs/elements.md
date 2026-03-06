# Element

An element is the basic building block of a Bento UI. Everything is an element. 
rectangles, rows, and columns are all elements. Elements can be nested inside each 
other to build layouts.

Every element can have a size, spacing, position, and visual properties set on it 
using builder methods chained after the constructor. For example:   

    `rect().w(Size::Fixed(100.0)).h(Size::Fixed(100.0)).bg(Color::RED)`   

Elements are rebuilt from scratch every frame inside your App's view() method.

List of available elements:
1. [Row](elements/row.md)
2. [Col](elements/col.md)
3. [Rect](elements/rect.md)
4. [Text](elements/text.md)
