import init, { find_primes } from './pkg/prime_finder.js';

init().then(() => {
    document.getElementById("submit").addEventListener("click", (e) => {
        find_primes();
    });
})