use anyhow::Result;
use plt::prelude::*;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), Uom::Px, Debug::On);
    sketch.render().save_default()?;
    Ok(())
}
