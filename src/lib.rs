use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, HtmlParagraphElement};

const PRIMES: [u32; 109] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 
                           53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 
                           113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 
                           181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 
                           251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 
                           317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 
                           397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 
                           463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 
                           557, 563, 569, 571, 577, 587, 593, 599];

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

#[wasm_bindgen]
pub fn find_primes() -> Result<(), JsValue> {
    // Get JS window objects
    let window = web_sys::window().expect("No global `window` exists.");
    let document = window.document().expect("Should have a document on window.");
    // Start nunmber element
    let start_number = document
        .get_element_by_id("start-num")
        .expect("No element found with expected ID.");
    let start_number = start_number
        .dyn_ref::<HtmlInputElement>()
        .expect("Element should be a input element.")
        .value()
        .parse::<u32>();
    // End number element
    let end_number = document
        .get_element_by_id("end-num")
        .expect("No element found with expected ID.");
    let end_number = end_number
        .dyn_ref::<HtmlInputElement>()
        .expect("Element should be a input element.")
        .value()
        .parse::<u32>();
    // Result field
    let result_field = document
        .get_element_by_id("result-field")
        .expect("No element found with expected ID.");
    let result_field = result_field
        .dyn_ref::<HtmlParagraphElement>()
        .expect("Element should be a paragraph element.");
    // Show loading
    result_field.set_inner_text("Loading...");
    // Initialise vector for storing primes
    let mut primes: Vec<u32> = Vec::new();
    // Match
    match (start_number, end_number) {
        (Ok(s), Ok(e)) => {
            if s > e {
                result_field.set_inner_text("Starting number cannot be greater than end number.");
                return Ok(());
            }
            for n in s..e {
                if PRIMES.contains(&n) {
                    primes.push(n);
                    continue;
                }
                if PRIMES.iter().any(|p| n % p == 0) || n == 1{
                    continue;
                }
                let mut i = 5;
                while i * i <= n {
                    if n % i == 0 || n % (i + 2) == 0 {
                        continue;
                    }
                    i += 6;
                }
                primes.push(n);
            }
            result_field.set_inner_text(
                &primes
                    .iter()
                    .filter_map(|num: &u32| Some(num.to_string()))
                    .collect::<Vec<String>>().join(", "));
        },
        (_, _) => {
            result_field.set_inner_text("Invalid number(s). Make sure that both numbers are posistive integers.");
        }
    }
    Ok(())
}