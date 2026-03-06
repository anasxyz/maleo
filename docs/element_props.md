# Element Properties

**w / h** : the width and height of the element. Can be a fixed pixel value or a percentage of the parent's size.

**min_w / min_h** : the element won't shrink below this size even if the parent is too small.

**max_w / max_h** : the element won't grow beyond this size even if there is space available.

**aspect_ratio** : locks the ratio between width and height so it doesn't distort.

**padding** : space inside the element between its edges and its children.

**margin** : space outside the element pushing it away from its siblings. Can be set to AUTO to fill available space, which is useful for centering.

**gap** : space between children inside a row or col.

**grow** : how much this element expands to fill leftover space in its parent. 0 means don't grow, 1 means take all remaining space.

**shrink** : how much this element compresses when there isn't enough room. Defaults to 1.

**basis** : the starting size before grow and shrink are applied.

**align_self** : overrides the parent's alignment for just this one element.

**position** : relative (default, participates in normal flow) or absolute (removed from flow, positioned by top/right/bottom/left).

**top / right / bottom / left** : inset from each edge, only used when position is absolute.

**overflow** : what happens when children are larger than the container. Hidden clips them, Visible lets them show outside, Scroll reserves scrollbar space.

**bg** : the background fill color.

**border** : the thickness of the border in pixels.

**border_color** : the color of the border.

**border_radius** : how rounded the corners are in pixels.

**opacity** : how transparent the element is. 0 is invisible, 1 is fully opaque.

**z_index** : which layer the element sits on. Higher values appear in front of lower ones.

**visible** : whether the element and all its children are drawn at all.
