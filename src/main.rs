use std::cmp;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Content that monkey says
   #[arg(short, long)]
   text: String,

   /// Line width
   #[arg(short, long, default_value_t = 30)]
   width: u8,
}

fn print_box(quote: String, line_width: usize) {
    let mut offset = 0;
    let quote_bytes = quote.as_bytes();

    if quote.len() == 0 { return ; }

    if quote.len() < line_width {
        let cage = String::from("-").repeat(quote.len()+2);
        println!("/{}\\", cage);
        println!("| {} |", quote);
        println!("\\{}/", cage);
        return;
    }

    let cage = String::from("-").repeat(line_width+2);

    for i in 0..(quote.len() / line_width) {
        let mut start = i * line_width - offset;
        let mut end = cmp::min((i + 1) * line_width, quote.len() - 1) - offset;
        let mut pad = String::from("");
        while end < quote.len()
            && end > 0
            && quote_bytes[end - 1] != b' '
            && quote_bytes[end] != b' '
        {
            offset += 1;
            end -= 1;
        }
        if quote_bytes[start] == b' ' {
            start += 1;
        }
        if end - start < line_width {
            let n_repeat = line_width * (i+1) - end + 1;
            pad = String::from(" ").repeat(n_repeat);
        }
        let sub_quote = &quote[start..end];
        println!("| {}{} |", sub_quote, pad);
    }
    println!("\\{}/", cage);
}

fn print_monkey(offset: usize) {
    let monkey = [
        String::from("  \\             "),
        String::from("w  c(..)o   (    "),
        String::from(" \\__(-)    __)  "),
        String::from("     /\\   (     "),
        String::from("    /(_)___)     "),
        String::from("    w /|         "),
        String::from("     | \\        "),
        String::from("    m  m         "),
    ];
    let str_offset = String::from(" ").repeat(offset);
    for i in 0..monkey.len() {
        println!("{}{}", str_offset, &monkey[i]);
    }
}

fn main() {
    let args = Args::parse();

    let monkey_offset = cmp::min(args.text.len(), args.width as usize) / 2;
    print_box(String::from(args.text), args.width as usize);
    print_monkey(monkey_offset);
}
