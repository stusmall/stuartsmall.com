use std::collections::HashMap;
use std::borrow::Cow;
use std::error::Error;
use std::fmt::Write;
use std::fs::{read_dir, read_to_string, File};
use std::io;
use std::io::stdout;
use std::path::Path;

use cooklang::{Converter, CooklangParser, Extensions, Item, ScaledRecipe, Section, Step};
use serde::Serialize;

pub fn build_folder(print_to_stdout: bool, input_folder: &Path) -> Result<(), Box<dyn Error>> {
    for file in read_dir(input_folder)? {
        let file_path = file?.path();
        if let Some(extension) = file_path.extension() {
            if extension == "cook" {
                build_md(print_to_stdout, &file_path)?;
            }
        }
    }
    Ok(())
}

pub fn build_md(print_to_stdout: bool, input_file: &Path) -> Result<(), Box<dyn Error>> {
    let parser = CooklangParser::new(Extensions::all(), Converter::default());
    let contents = read_to_string(input_file)?;
    let parsed = parser.parse(&contents);
    let scaled_recipe = parsed.into_result()?.0.default_scale();

    if print_to_stdout {
        print_md(&scaled_recipe, parser.converter(), &mut stdout().lock())?;
    } else {
        let mut output_path = input_file.to_path_buf();
        output_path.set_extension("md");
        let mut file = File::create(output_path)?;
        print_md(&scaled_recipe, parser.converter(), &mut file)?;
    };

    Ok(())
}

fn print_md(
    recipe: &ScaledRecipe,
    converter: &Converter,
    mut writer: impl io::Write,
) -> Result<(), Box<dyn Error>> {
    frontmatter(&mut writer, recipe)?;
    ingredients(&mut writer, recipe, converter)?;
    cookware(&mut writer, recipe)?;
    sections(&mut writer, recipe)?;
    Ok(())
}

fn frontmatter(
    writer: &mut impl io::Write,
    scaled_recipe: &ScaledRecipe,
) -> Result<(), Box<dyn Error>> {
    #[derive(Serialize)]
    struct Frontmatter<'a> {
        #[serde(flatten)]
        metadata: serde_yaml::mapping::Mapping, //IndexMap<String, String>,
        taxonomies: HashMap<String, Vec<Cow<'a, str>>>,
    }

    if scaled_recipe.metadata.map.is_empty() {
        return Ok(());
    }

    let mut metadata = scaled_recipe.metadata.map.clone();
    metadata.remove("tags");

    let mut taxonomies = HashMap::new();
    if let Some(tags) = scaled_recipe.metadata.tags() {
        taxonomies.insert("tags".to_string(), tags.to_vec());
    }

    let frontmatter = Frontmatter {
        metadata,
        taxonomies,
    };
    write!(writer, "+++\n{}+++\n", toml::to_string(&frontmatter)?)?;
    Ok(())
}

fn ingredients(
    w: &mut impl io::Write,
    recipe: &ScaledRecipe,
    converter: &Converter,
) -> Result<(), Box<dyn Error>> {
    if recipe.ingredients.is_empty() {
        return Ok(());
    }

    writeln!(w, "## Ingredients")?;

    for entry in recipe.group_ingredients(converter) {
        let ingredient = entry.ingredient;

        if !ingredient.modifiers().should_be_listed() {
            continue;
        }

        write!(w, "- ")?;
        if !entry.quantity.is_empty() {
            write!(w, "{} ", entry.quantity)?;
        }

        if ingredient.modifiers().is_recipe() {
            write!(w, "[{}](../{})", ingredient.display_name(), ingredient.display_name().replace(" ", "-"))?;
        } else {
            write!(w, "{}", ingredient.display_name())?;
        }

        if ingredient.modifiers().is_optional() {
            write!(w, " (optional)")?;
        }

        if let Some(note) = &ingredient.note {
            write!(w, " ({note})")?;
        }
        writeln!(w)?;
    }
    writeln!(w)?;

    Ok(())
}

fn cookware(w: &mut impl io::Write, recipe: &ScaledRecipe) -> Result<(), Box<dyn Error>> {
    if recipe.cookware.is_empty() {
        return Ok(());
    }

    writeln!(w, "## Cookware")?;
    for item in recipe.group_cookware() {
        let cw = item.cookware;
        write!(w, "- ")?;
        if !item.amount.is_empty() {
            write!(w, "{} ", item.amount)?;
        }
        write!(w, "{}", cw.display_name())?;

        if cw.modifiers().is_optional() {
            write!(w, " (optional)")?;
        }

        if let Some(note) = &cw.note {
            write!(w, " ({note})")?;
        }
        writeln!(w)?;
    }

    writeln!(w)?;
    Ok(())
}

fn sections(w: &mut impl io::Write, recipe: &ScaledRecipe) -> Result<(), Box<dyn Error>> {
    writeln!(w, "## Steps")?;
    for (idx, section) in recipe.sections.iter().enumerate() {
        write_section(w, section, recipe, idx + 1)?;
    }
    Ok(())
}

fn write_section(
    w: &mut impl io::Write,
    section: &Section,
    recipe: &ScaledRecipe,
    idx: usize,
) -> Result<(), Box<dyn Error>> {
    if section.name.is_some() || recipe.sections.len() > 1 {
        if let Some(name) = &section.name {
            writeln!(w, "### {name}")?;
        } else {
            writeln!(w, "### Section {idx}")?;
        }
    }
    for content in &section.content {
        match content {
            cooklang::Content::Step(step) => write_step(w, step, recipe)?,
            cooklang::Content::Text(text) => print_wrapped(w, text)?,
        };
        writeln!(w)?;
    }
    Ok(())
}

fn write_step(
    w: &mut impl io::Write,
    step: &Step,
    recipe: &ScaledRecipe,
) -> Result<(), Box<dyn Error>> {
    let mut step_str = String::new();

    step_str += &format!("{}. ", step.number);

    for item in &step.items {
        match item {
            Item::Text { value } => step_str.push_str(value),
            &Item::Ingredient { index } => {
                let igr = &recipe.ingredients[index];
                step_str.push_str(igr.display_name().as_ref());
            }
            &Item::Cookware { index } => {
                let cw = &recipe.cookware[index];
                step_str.push_str(&cw.name);
            }
            &Item::Timer { index } => {
                let t = &recipe.timers[index];
                if let Some(name) = &t.name {
                    write!(&mut step_str, "({name})").unwrap();
                }
                if let Some(quantity) = &t.quantity {
                    write!(&mut step_str, "{}", quantity).unwrap();
                }
            }
            &Item::InlineQuantity { index } => {
                let q = &recipe.inline_quantities[index];
                write!(&mut step_str, "{}", q.value()).unwrap();
                if let Some(u) = q.unit() {
                    step_str.push_str(u);
                }
            }
        }
    }
    print_wrapped(w, &step_str)?;
    Ok(())
}

fn print_wrapped(w: &mut impl io::Write, text: &str) -> Result<(), Box<dyn Error>> {
    print_wrapped_with_options(w, text, |o| o)
}

static TERM_WIDTH: once_cell::sync::Lazy<usize> =
    once_cell::sync::Lazy::new(|| textwrap::termwidth().min(80));

fn print_wrapped_with_options<F>(
    w: &mut impl io::Write,
    text: &str,
    f: F,
) -> Result<(), Box<dyn Error>>
where
    F: FnOnce(textwrap::Options) -> textwrap::Options,
{
    let options = f(textwrap::Options::new(*TERM_WIDTH));
    let lines = textwrap::wrap(text, options);
    for line in lines {
        writeln!(w, "{}", line)?;
    }
    Ok(())
}
