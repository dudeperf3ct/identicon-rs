/*
Reference: https://github.blog/news-insights/company-news/identicons/

Algorithm
Identicons are simple 5×5 “pixel” sprites that are generated using a hash of the user’s ID.
The algorithm walks through the hash and turns pixels on or off depending on even or odd values.
These generated patterns, combined with hash-determined color values, ensures a huge number of unique Identicons.

*/

use clap::Parser;
use gh_identicons::{Identicon, Pattern};

#[derive(Debug, Parser, Clone)]
struct CliArgs {
    // Unique IP or string
    input: String,
    // Image height
    height: u32,
    // Image width
    width: u32,
    /// Pattern generation approach
    #[arg(long, value_enum, default_value_t = Pattern::Symmetrical)]
    pattern: Pattern,
}

fn main() {
    let args = CliArgs::parse();
    let unique_id = args.input.as_str();
    println!("Generating identicon for {}", unique_id);
    println!("Pattern: {:?}", args.pattern);

    // TODO: Parameterize this?
    let block_width = 5;
    let block_height = 5;

    let identicon = Identicon::new(args.height, args.width, block_height, block_width);
    let unique_hash = identicon.calculate_hash(unique_id);

    // Walk through hash and turn pixel on or off depending on even or odd value
    let generated_pattern = identicon.generate_pattern(&unique_hash, args.pattern);
    let rgb_color = identicon.get_random_color(&unique_hash);
    identicon.render_pattern(&rgb_color, &generated_pattern);

    println!("Generated indeticon at test.png")
}
