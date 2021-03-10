use super::*;
use crate::paper::{Paper, PaperClass};

/// `page`: Configure pages.
///
/// # Positional arguments
/// - Paper name: optional, of type `string`, see [here](crate::paper) for a
///               full list of all paper names.
/// - Body:       optional, of type `template`.
///
/// # Named arguments
/// - Width of the page:         `width`, of type `length`.
/// - Height of the page:        `height`, of type `length`.
/// - Margins for all sides:     `margins`, of type `linear` relative to sides.
/// - Left margin:               `left`, of type `linear` relative to width.
/// - Right margin:              `right`, of type `linear` relative to width.
/// - Top margin:                `top`, of type `linear` relative to height.
/// - Bottom margin:             `bottom`, of type `linear` relative to height.
/// - Flip width and height:     `flip`, of type `bool`.
/// - Main layouting direction:  `main-dir`, of type `direction`.
/// - Cross layouting direction: `cross-dir`, of type `direction`.
pub fn page(ctx: &mut EvalContext, args: &mut ValueArgs) -> Value {
    let paper = args.find::<Spanned<String>>(ctx).and_then(|name| {
        Paper::from_name(&name.v).or_else(|| {
            ctx.diag(error!(name.span, "invalid paper name"));
            None
        })
    });

    let width = args.get(ctx, "width");
    let height = args.get(ctx, "height");
    let margins = args.get(ctx, "margins");
    let left = args.get(ctx, "left");
    let top = args.get(ctx, "top");
    let right = args.get(ctx, "right");
    let bottom = args.get(ctx, "bottom");
    let flip = args.get(ctx, "flip");
    let main = args.get(ctx, "main-dir");
    let cross = args.get(ctx, "cross-dir");
    let body = args.find::<ValueTemplate>(ctx);

    Value::template("page", move |ctx| {
        let snapshot = ctx.state.clone();

        if let Some(paper) = paper {
            ctx.state.page.class = paper.class;
            ctx.state.page.size = paper.size();
            ctx.state.page.expand = Spec::uniform(Expansion::Fill);
        }

        if let Some(width) = width {
            ctx.state.page.class = PaperClass::Custom;
            ctx.state.page.size.width = width;
            ctx.state.page.expand.horizontal = Expansion::Fill;
        }

        if let Some(height) = height {
            ctx.state.page.class = PaperClass::Custom;
            ctx.state.page.size.height = height;
            ctx.state.page.expand.vertical = Expansion::Fill;
        }

        if let Some(margins) = margins {
            ctx.state.page.margins = Sides::uniform(Some(margins));
        }

        if let Some(left) = left {
            ctx.state.page.margins.left = Some(left);
        }

        if let Some(top) = top {
            ctx.state.page.margins.top = Some(top);
        }

        if let Some(right) = right {
            ctx.state.page.margins.right = Some(right);
        }

        if let Some(bottom) = bottom {
            ctx.state.page.margins.bottom = Some(bottom);
        }

        if flip.unwrap_or(false) {
            let page = &mut ctx.state.page;
            std::mem::swap(&mut page.size.width, &mut page.size.height);
            std::mem::swap(&mut page.expand.horizontal, &mut page.expand.vertical);
        }

        ctx.set_dirs(Gen::new(main, cross));

        let mut softness = ctx.end_page_group(|_| false);
        if let Some(body) = &body {
            // TODO: Restrict body to a single page?
            ctx.start_page_group(Softness::Hard);
            body.exec(ctx);
            ctx.end_page_group(|s| s == Softness::Hard);
            softness = Softness::Soft;
            ctx.state = snapshot;
        }

        ctx.start_page_group(softness);
    })
}

/// `pagebreak`: Start a new page.
pub fn pagebreak(_: &mut EvalContext, _: &mut ValueArgs) -> Value {
    Value::template("pagebreak", move |ctx| {
        ctx.end_page_group(|_| true);
        ctx.start_page_group(Softness::Hard);
    })
}