use super::*;
use crate::graphics_engine::shaped_segment::ShapedSegment;
use crate::graphics_engine::font::Font;

#[derive(Debug)]
pub(in crate::layout) struct InlineFormattingContext {
    pub(super) parent_style: Arc<ComputedValues>,
    pub(super) inline_level_boxes: Vec<Arc<InlineLevelBox>>,
}

#[derive(Debug)]
pub(in crate::layout) enum InlineLevelBox {
    InlineBox(InlineBox),
    TextRun(TextRun),
    OutOfFlowAbsolutelyPositionedBox(AbsolutelyPositionedBox),
    OutOfFlowFloatBox(FloatBox),
    Atomic {
        style: Arc<ComputedValues>,
        // FIXME: this should be IndependentFormattingContext:
        contents: ReplacedContent,
    },
}

#[derive(Debug)]
pub(in crate::layout) struct InlineBox {
    pub style: Arc<ComputedValues>,
    pub first_fragment: bool,
    pub last_fragment: bool,
    pub children: Vec<Arc<InlineLevelBox>>,
}

/// https://www.w3.org/TR/css-display-3/#css-text-run
#[derive(Debug)]
pub(in crate::layout) struct TextRun {
    pub parent_style: Arc<ComputedValues>,
    pub text: String,
    pub font: Font,
}

/// A struct representing the current inline box as it is being assembled.
/// These get nested when inline boxes are nested into each other.
struct InlineNestingLevelState<'box_tree> {
    /// Iterator over the boxes that haven't been processed yet.
    remaining_boxes: std::slice::Iter<'box_tree, Arc<InlineLevelBox>>,
    /// Fragments belonging to the current line. Once the line is completed,
    /// these will be moved.
    fragments_so_far: Vec<Fragment>,
    /// Start position of this set of fragments, relative to the containing block,
    /// **not** relative to the containing nesting level.
    inline_start: Length,
    /// Ascent block value of the font (not the line).
    block_ascent: Length,
    /// Descent block value of the font (not the line).
    block_descent: Length,
    /// Value of 1ex for the given font.
    block_ex: Length,
    /// Adjustment of the baseline (due to vertical-align property), relative
    /// to overall baseline, not to containing nesting level.
    block_baseline_adjustment: Length,
    /// Maximum block ascent and descent of all fragments encountered so far. Note: These
    /// values include leading/half-leading. See: https://drafts.csswg.org/css2/#line-height
    max_block_ascent_of_fragments_so_far: Length,
    max_block_descent_of_fragments_so_far: Length,
}

struct PartialInlineBoxFragment<'box_tree> {
    style: Arc<ComputedValues>,
    start_corner: Vec2<Length>,
    padding: Sides<Length>,
    border: Sides<Length>,
    margin: Sides<Length>,
    block_ascent: Length,
    block_descent: Length,
    last_box_tree_fragment: bool,
    parent_nesting_level: InlineNestingLevelState<'box_tree>,
}

struct InlineFormattingContextState<'box_tree, 'cb> {
    containing_block: &'cb ContainingBlock,
    line_boxes: LinesBoxes,
    /// Alignment setting for text in this inline formatting context.
    text_align: TextAlign,
    /// Current inline position given the boxes remaining on the stack.
    inline_position: Length,
    partial_inline_boxes_stack: Vec<PartialInlineBoxFragment<'box_tree>>,
    current_nesting_level: InlineNestingLevelState<'box_tree>,
}

struct LinesBoxes {
    boxes: Vec<Fragment>, // vector of lines; each line gets represented as one anonymous fragment
    next_line_block_position: Length, // position of the next line we are currently assembling
}

impl InlineFormattingContext {
    pub(super) fn new(parent_style: &Arc<ComputedValues>) -> InlineFormattingContext {
        InlineFormattingContext {
            parent_style: parent_style.clone(),
            inline_level_boxes: Default::default(),
        }
    }

    pub(super) fn layout<'a>(
        &'a self,
        containing_block: &ContainingBlock,
        tree_rank: usize,
        absolutely_positioned_fragments: &mut Vec<AbsolutelyPositionedFragment<'a>>,
    ) -> FlowChildren {
        let font = Font::new_from_computed_values(&self.parent_style);
        let block_ascent = font.get_ascent();
        let block_descent = font.get_descent();
        let block_ex = font.get_ex();
        let mut ifc = InlineFormattingContextState {
            containing_block,
            partial_inline_boxes_stack: Vec::new(),
            line_boxes: LinesBoxes {
                boxes: Vec::new(),
                next_line_block_position: Length::zero(),
            },
            text_align: self.parent_style.text_inherited.text_align,
            inline_position: Length::zero(),
            current_nesting_level: InlineNestingLevelState {
                remaining_boxes: self.inline_level_boxes.iter(),
                fragments_so_far: Vec::with_capacity(self.inline_level_boxes.len()),
                inline_start: Length::zero(),
                block_ascent,
                block_descent,
                block_ex,
                block_baseline_adjustment: Length::zero(),
                max_block_ascent_of_fragments_so_far: Length::zero(),
                max_block_descent_of_fragments_so_far: Length::zero(),
            },
        };
        loop {
            if let Some(child) = ifc.current_nesting_level.remaining_boxes.next() {
                match &**child {
                    InlineLevelBox::InlineBox(inline) => {
                        let partial = inline.start_layout(&mut ifc);
                        ifc.partial_inline_boxes_stack.push(partial)
                    }
                    InlineLevelBox::TextRun(run) => run.layout(&mut ifc),
                    InlineLevelBox::Atomic { style: _, contents } => {
                        // FIXME
                        match *contents {}
                    }
                    InlineLevelBox::OutOfFlowAbsolutelyPositionedBox(box_) => {
                        let initial_start_corner = match box_.style.specified_display {
                            Display::GeneratingBox(DisplayGeneratingBox::OutsideInside {
                                outside,
                                inside: _,
                            }) => Vec2 {
                                inline: match outside {
                                    DisplayOutside::Inline => ifc.inline_position,
                                    DisplayOutside::Block => Length::zero(),
                                },
                                block: ifc.line_boxes.next_line_block_position,
                            },
                            Display::Contents => {
                                panic!("display:contents does not generate an abspos box")
                            }
                            Display::None => panic!("display:none does not generate an abspos box"),
                        };
                        absolutely_positioned_fragments
                            .push(box_.layout(initial_start_corner, tree_rank));
                    }
                    InlineLevelBox::OutOfFlowFloatBox(_box_) => {
                        // TODO
                        continue;
                    }
                }
            } else { // no more boxes in the current nesting level
                // are there partial boxes that need finishing?
                if let Some(mut partial) = ifc.partial_inline_boxes_stack.pop() {
                    // yes, finish partial boxes
                    partial.finish_layout(
                        &mut ifc.current_nesting_level,
                        &mut ifc.inline_position,
                        false,
                    );
                    ifc.current_nesting_level = partial.parent_nesting_level
                } else {
                    // no, finish current inline formatting context, we are done
                    ifc.finish_line();
                    //ifc.line_boxes
                    //    .finish_line(&mut ifc.current_nesting_level, containing_block);
                    return FlowChildren {
                        fragments: ifc.line_boxes.boxes,
                        block_size: ifc.line_boxes.next_line_block_position,
                        collapsible_margins_in_children: CollapsedBlockMargins::zero(),
                    };
                }
            }
        }
    }
}

impl<'box_tree, 'cb> InlineFormattingContextState<'box_tree, 'cb> {
    /// Finish off the current line and reset.
    fn finish_line(&mut self) {
        // To complete a line, we need to iterate over all open boxes in reverse
        // order and finish them up for this line
        let mut nesting_level = &mut self.current_nesting_level;
        for partial in self.partial_inline_boxes_stack.iter_mut().rev() {
            // if there are any boxes to finish then we are at a line break
            partial.finish_layout(nesting_level, &mut self.inline_position, true);
            nesting_level.inline_start = Length::zero();
            nesting_level.max_block_ascent_of_fragments_so_far = Length::zero();
            nesting_level.max_block_descent_of_fragments_so_far = Length::zero();
            partial.start_corner.inline = Length::zero();
            partial.padding.inline_start = Length::zero();
            partial.border.inline_start = Length::zero();
            partial.margin.inline_start = Length::zero();
            nesting_level = &mut partial.parent_nesting_level;
        }
        nesting_level.inline_start = Length::zero();
        // We don't zero `nesting_level.max_block_ascent/descent_of_fragments_so_far` here, as
        // these values are still needed in the `finish_line()` call.
        //ifc.finish_line(nesting_level);
        self.line_boxes
            .finish_line(
                nesting_level,
                self.containing_block,
                &self.inline_position,
                &self.text_align,
            );
        self.inline_position = Length::zero();
    }
}

impl LinesBoxes {
    /// Takes all the fragments that have accumulated on the stack
    /// (`top_nesting_level.fragments_so_far`), sticks them into an anonymous
    /// fragment, and pushes it on the stack of lines (`self.boxes`).
    fn finish_line(
        &mut self,
        top_nesting_level: &mut InlineNestingLevelState,
        containing_block: &ContainingBlock,
        inline_position: &Length,
        text_align: &TextAlign,
    ) {
        // available line length minus used line length
        let inline_delta = containing_block.inline_size - *inline_position;
        // fractional shift
        let p = match text_align {
            TextAlign::Left => 0.0,
            TextAlign::Right => 1.0,
            TextAlign::Center => 0.5,
            // Justify is not implemented, use TextAlign::Left instead
            TextAlign::Justify => 0.0, // TODO: properly implement Justify
            TextAlign::Percentage(p) => p.unit_value,
        };

        let start_corner = Vec2 {
            inline: inline_delta * p,
            block: self.next_line_block_position,
        };
        let max_block_ascent = std::mem::replace(
            &mut top_nesting_level.max_block_ascent_of_fragments_so_far,
            Length::zero(),
        );
        let max_block_descent = std::mem::replace(
            &mut top_nesting_level.max_block_descent_of_fragments_so_far,
            Length::zero(),
        );
        let size = Vec2 {
            inline: containing_block.inline_size,
            block: max_block_ascent + max_block_descent,
        };
        self.next_line_block_position += size.block;

        // calculate block adjustment required if some fragments have excess block ascent
        let block_adjustment =
            max_block_ascent - top_nesting_level.block_ascent;
        for fragment in &mut top_nesting_level.fragments_so_far.iter_mut() {
            fragment.translate_block(block_adjustment);
        }

        // create an anonymous fragment containing the next line and push onto stack of lines
        self.boxes.push(Fragment::Anonymous(AnonymousFragment {
            children: take(&mut top_nesting_level.fragments_so_far),
            rect: Rect { start_corner, size },
            mode: containing_block.mode,
        }));
    }
}

impl InlineBox {
    fn start_layout<'box_tree>(
        &'box_tree self,
        ifc: &mut InlineFormattingContextState<'box_tree, '_>,
    ) -> PartialInlineBoxFragment<'box_tree> {
        let style = self.style.clone();
        let font = Font::new_from_computed_values(&style);
        let block_ascent = font.get_ascent();
        let block_descent = font.get_descent();
        let block_ex = font.get_ex();
        let block_baseline_shift = calculate_baseline_shift(
            &style, &block_ascent, &block_descent, &ifc.current_nesting_level
        );
        let block_baseline_adjustment = ifc.current_nesting_level.block_baseline_adjustment +
            block_baseline_shift;
        let cbis = ifc.containing_block.inline_size;
        let mut padding = style.padding().percentages_relative_to(cbis);
        let mut border = style.border_width().percentages_relative_to(cbis);
        let mut margin = style
            .margin()
            .percentages_relative_to(cbis)
            .auto_is(Length::zero);
        if self.first_fragment {
            ifc.inline_position += padding.inline_start + border.inline_start + margin.inline_start;
        } else {
            padding.inline_start = Length::zero();
            border.inline_start = Length::zero();
            margin.inline_start = Length::zero();
        }
        let mut start_corner = Vec2 {
            block: block_baseline_shift,
            inline: ifc.inline_position - ifc.current_nesting_level.inline_start,
        };
        start_corner += &relative_adjustement(
            &style,
            ifc.containing_block.inline_size,
            ifc.containing_block.block_size,
        );
        PartialInlineBoxFragment {
            style,
            start_corner,
            padding,
            border,
            margin,
            block_ascent,
            block_descent,
            last_box_tree_fragment: self.last_fragment,
            parent_nesting_level: std::mem::replace(
                &mut ifc.current_nesting_level,
                InlineNestingLevelState {
                    remaining_boxes: self.children.iter(),
                    fragments_so_far: Vec::with_capacity(self.children.len()),
                    inline_start: ifc.inline_position,
                    block_ascent,
                    block_descent,
                    block_ex,
                    block_baseline_adjustment,
                    max_block_ascent_of_fragments_so_far: Length::zero(),
                    max_block_descent_of_fragments_so_far: Length::zero(),
                },
            ),
        }
    }
}

impl<'box_tree> PartialInlineBoxFragment<'box_tree> {
    fn finish_layout(
        &mut self,
        nesting_level: &mut InlineNestingLevelState,
        inline_position: &mut Length,
        at_line_break: bool,
    ) {
        // Calculate the block adjustment necessary if different font sizes are nested
        let block_adjustment = self.parent_nesting_level.block_ascent - self.block_ascent;
        let mut start_corner = self.start_corner.clone();
        start_corner.block += block_adjustment;

        let mut fragment = BoxFragment {
            style: self.style.clone(),
            children: take(&mut nesting_level.fragments_so_far),
            content_rect: Rect {
                size: Vec2 {
                    inline: *inline_position - nesting_level.inline_start,
                    // The block size should be given by the
                    // height of the very first font encountered
                    // https://drafts.csswg.org/css2/#strut
                    block: self.block_ascent + self.block_descent,
                },
                start_corner,
            },
            padding: self.padding.clone(),
            border: self.border.clone(),
            margin: self.margin.clone(),
            block_margins_collapsed_with_children: CollapsedBlockMargins::zero(),
        };
        let last_fragment = self.last_box_tree_fragment && !at_line_break;
        if last_fragment {
            *inline_position += fragment.padding.inline_end
                + fragment.border.inline_end
                + fragment.margin.inline_end;
        } else {
            fragment.padding.inline_end = Length::zero();
            fragment.border.inline_end = Length::zero();
            fragment.margin.inline_end = Length::zero();
        }
        self.parent_nesting_level
            .max_block_ascent_of_fragments_so_far
            .max_assign(nesting_level.max_block_ascent_of_fragments_so_far);
        self.parent_nesting_level
            .max_block_descent_of_fragments_so_far
            .max_assign(nesting_level.max_block_descent_of_fragments_so_far);
        self.parent_nesting_level
            .fragments_so_far
            .push(Fragment::Box(fragment));
    }
}

impl TextRun {
    fn layout(&self, ifc: &mut InlineFormattingContextState) {
        match self.parent_style.text_inherited.white_space {
            WhiteSpace::Nowrap | WhiteSpace::Pre => self.layout_nowrap(ifc),
            _ => self.layout_wrap(ifc)
        };
    }

    /// Text layout with word wrap.
    fn layout_wrap(&self, ifc: &mut InlineFormattingContextState) {
        let mut chars = self.text.chars();
        loop { // loop over lines
            let mut newline = false;
            let available = ifc.containing_block.inline_size - ifc.inline_position;
            let mut shaped = ShapedSegment::new( self.font.clone());
            let mut last_break_opportunity = Some((shaped.save(), chars.clone()));
            loop { // loop over text within lines
                let next = chars.next();
                if matches!(next, Some(' ') | Some('\n') | None) {
                    let inline_size: Length = shaped.get_advance_width().unwrap().into(); // TODO: handle potential error nicely, don't just unwrap()
                    if inline_size > available {
                        if let Some((state, iter)) = last_break_opportunity.take() {
                            shaped.restore(&state);
                            chars = iter;
                        }
                        break;
                    }
                }
                if let Some(ch) = next {
                    if ch == '\n' {
                        shaped.strip_space();
                        newline = true;
                        break;
                    }
                    if ch == ' ' {
                        last_break_opportunity = Some((shaped.save(), chars.clone()))
                    }
                    shaped.append_char(ch).unwrap(); // TODO: handle potential error nicely, don't just unwrap()
                } else {
                    break;
                }
            }
            let inline_size = shaped.get_advance_width().unwrap().into(); // TODO: handle potential error nicely, don't just unwrap()
            let line_ascent_descent = calculate_line_ascent_descent(&*self.parent_style, &self.font);
            let content_rect = Rect {
                start_corner: Vec2 {
                    block: Length::zero(),
                    inline: ifc.inline_position - ifc.current_nesting_level.inline_start,
                },
                size: Vec2 {
                    block: line_ascent_descent.0 + line_ascent_descent.1,
                    inline: inline_size,
                },
            };
            if !shaped.empty() {
                // add the new segment if it is not empty. empty segments can arise
                // if a line break occurs at the beginning of a text run, or if a
                // newline character is encountered
                ifc.inline_position += inline_size;
                ifc.current_nesting_level
                    .max_block_ascent_of_fragments_so_far
                    .max_assign(
                        line_ascent_descent.0 - ifc.current_nesting_level.block_baseline_adjustment
                    );
                ifc.current_nesting_level
                    .max_block_descent_of_fragments_so_far
                    .max_assign(
                        line_ascent_descent.1 + ifc.current_nesting_level.block_baseline_adjustment
                    );
                ifc.current_nesting_level
                    .fragments_so_far
                    .push(Fragment::Text(TextFragment {
                        parent_style: self.parent_style.clone(),
                        content_rect,
                        text: shaped,
                    }));
            }
            if chars.as_str().is_empty() && !newline {
                break;
            } else {
                // line break; finish the line and start a new one
                ifc.finish_line();
            }
        }
    }

    /// Text layout without word wrap
    fn layout_nowrap(&self, ifc: &mut InlineFormattingContextState) {
        let mut chars = self.text.chars();
        loop {
            let mut newline = false;
            let mut shaped = ShapedSegment::new( self.font.clone());
            loop {
                let next = chars.next();
                match next {
                    None => break,
                    Some('\n') => {
                        newline = true;
                        break;
                    },
                    Some(ch) => {
                        shaped.append_char(ch).unwrap(); // TODO: handle potential error nicely, don't just unwrap()
                    }
                }
            }
            let inline_size = shaped.get_advance_width().unwrap().into(); // TODO: handle potential error nicely, don't just unwrap()
            let line_ascent_descent = calculate_line_ascent_descent(&*self.parent_style, &self.font);
            let content_rect = Rect {
                start_corner: Vec2 {
                    block: Length::zero(),
                    inline: ifc.inline_position - ifc.current_nesting_level.inline_start,
                },
                size: Vec2 {
                    block: line_ascent_descent.0 + line_ascent_descent.1,
                    inline: inline_size,
                },
            };
            if !shaped.empty() {
                // add the new segment if it is not empty. empty segments can arise
                // if a line break occurs at the beginning of a text run, or if a
                // newline character is encountered
                ifc.inline_position += inline_size;
                ifc.current_nesting_level
                    .max_block_ascent_of_fragments_so_far
                    .max_assign(
                        line_ascent_descent.0 - ifc.current_nesting_level.block_baseline_adjustment
                    );
                ifc.current_nesting_level
                    .max_block_descent_of_fragments_so_far
                    .max_assign(
                        line_ascent_descent.1 + ifc.current_nesting_level.block_baseline_adjustment
                    );
                ifc.current_nesting_level
                    .fragments_so_far
                    .push(Fragment::Text(TextFragment {
                        parent_style: self.parent_style.clone(),
                        content_rect,
                        text: shaped,
                    }));
            }
            if chars.as_str().is_empty() && !newline {
                break;
            } else {
                // line break; finish the line and start a new one
                ifc.finish_line();
            }
        }
    }
}


// convenience helper functions

/// Calculate the line ascent and descent (font ascent/descent + half leading)
/// https://drafts.csswg.org/css2/#leading
fn calculate_line_ascent_descent(style: &ComputedValues, font: &Font) -> (Length, Length) {
    let line_height = style.line_inherited.line_height.percentage_or_number_relative_to(
        style.font.font_size.0
    );
    let font_ascent = font.get_ascent();
    let font_descent = font.get_descent();
    let leading = line_height - (font_ascent + font_descent);
    (font_ascent + leading / 2., font_descent + leading / 2.)
}

fn calculate_baseline_shift(
    style: &ComputedValues,
    block_ascent: &Length,
    block_descent: &Length,
    parent_nesting_level: &InlineNestingLevelState,
) -> Length {
    match style.line_reset.vertical_align {
        VerticalAlign::Sub => {
            // move by 1ex down, ~half a character's height
            parent_nesting_level.block_ascent * 0.5
        },
        VerticalAlign::Super => {
            // move by 1ex up, ~half a character's height
            -parent_nesting_level.block_ascent * 0.5
        },
        //VerticalAlign::Top => { },
        //VerticalAlign::TextTop => { },
        VerticalAlign::Middle => {
            // "Align the vertical midpoint of the box with the baseline of the parent box plus half the x-height of the parent."
            // https://drafts.csswg.org/css2/#propdef-vertical-align
            let block_total = *block_ascent + *block_descent;
            parent_nesting_level.block_ex * (-0.5) + block_total - *block_ascent
        },
        //VerticalAlign::Bottom => { },
        //VerticalAlign::TextBottom => { },
        VerticalAlign::Length(shift) => {
            -shift
        },
        VerticalAlign::Percentage(shift_percent) => {
            // Percent is calculated relative to the line height of the element itself
            // https://www.w3.org/TR/CSS22/visudet.html#propdef-vertical-align
            let line_height = style.line_inherited.line_height.percentage_or_number_relative_to(
                style.font.font_size.0
            );
            -line_height * shift_percent
        },
        _ => { // VerticalAlign::Baseline and cases that are not implemented
            Length::zero()
        }
    }
}