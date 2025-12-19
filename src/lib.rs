use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlInputElement, HtmlParagraphElement, window};

const PRIMES: [u32; 109] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
    557, 563, 569, 571, 577, 587, 593, 599,
];

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

#[wasm_bindgen]
pub fn find_primes() -> Result<(), JsValue> {
    // Get JS window objects
    let window = window().expect("No global `window` exists.");
    let document = window
        .document()
        .expect("Should have a document on window.");
    // Start number
    let start_number = get_element::<HtmlInputElement>(
        &document,
        "start-num",
        (
            "No element found with expected ID.",
            "Element should be a input element.",
        ),
    )
    .value()
    .parse::<u32>();
    // End number
    let end_number = get_element::<HtmlInputElement>(
        &document,
        "end-num",
        (
            "No element found with expected ID.",
            "Element should be a input element.",
        ),
    )
    .value()
    .parse::<u32>();
    // Result field
    let result_field: HtmlParagraphElement = get_element(
        &document,
        "result-field",
        (
            "No element found with expected ID.",
            "Element should be a paragraph element.",
        ),
    );
    // Show loading
    result_field.set_inner_text("Loading...");
    // Match
    match (start_number, end_number) {
        (Ok(s), Ok(e)) => {
            // Initialise vector for storing primes
            let mut primes: Vec<u32> = Vec::new();
            if s > e {
                result_field.set_inner_text("Starting number cannot be greater than end number.");
                return Ok(());
            }
            get_primes(s, e, &mut primes);
            result_field.set_inner_text(
                &primes
                .iter()
                    .filter_map(|num: &u32| Some(num.to_string()))
                    .collect::<Vec<String>>()
                    .join(", "),
            );
        }
        (_, _) => {
            result_field.set_inner_text(
                "Invalid number(s). Make sure that both numbers are posistive integers.",
            );
        }
    }
    Ok(())
}

fn get_primes(start: u32, end: u32, out: &mut Vec<u32>) {
    for n in start..end {
        if PRIMES.contains(&n) {
            out.push(n);
            continue;
        }
        if PRIMES.iter().any(|p| n % p == 0) || n == 1 {
            continue;
        }
        let mut i = 5;
        let mut prime = true;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                prime = false;
                break;
            }
            i += 6;
        }
        if !prime {
            continue;
        }
        out.push(n);
    }
}

fn get_element<T: JsCast>(document: &Document, id: &str, error_messages: (&str, &str)) -> T {
    document
        .get_element_by_id(id)
        .expect(error_messages.0)
        .dyn_into::<T>()
        .expect(error_messages.1)
}
