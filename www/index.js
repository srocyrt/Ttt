import * as wasm from "ascii_displayer";
import { memory } from "ascii_displayer/ascii_displayer_bg.wasm"

const text = document.querySelector("p");

let width = 256 + 1;
let height = 64

let time_spent = [2, 2, 2, 2, 2, 2, 2, 2, 2, 2];
function next_frame() {
    // time_spent.shift();
    // time_spent.push(diff);
    print();
    // let sum = 0;
    // for (let i = 0; i < time_spent.length; i++) {
    //     sum += time_spent[i];
    // }
    // sum = sum / 10;
    // console.log("fps: ", 1000 / sum);
    requestAnimationFrame(next_frame);
}

// requestAnimationFrame(next_frame);

function print() {
    let ptr = wasm.add();
    let n = new Uint8Array(memory.buffer, ptr, width * height);
    let t = new TextDecoder();
    text.textContent = t.decode(n);
}