let wasm = require("ascii_displayer");
let { BitMap, Screen } = wasm;
// let { memory } = import("ascii_displayer/ascii_displayer_bg.wasm");

let width = 256;
let height = 64;

class white_noise {
    constructor() {
        this.size_x = 8;
        this.size_y = 4;
        this.x = Math.floor(Math.random() * (width - this.size_x));
        this.y = Math.floor(Math.random() * (height - this.size_y));
        this.dx = (Math.floor(Math.random() * 2) * 2) - 1;
        this.dy = (Math.floor(Math.random() * 2) * 2) - 1;
        // this.noise = BitMap.white_noise(this.size_x, this.size_y);
    }
    move() {
        this.x += this.dx;
        this.y += this.dy;
        if (this.x + this.size_x > width || this.x < 0) {
            this.dx = -this.dx;
            this.x += this.dx;
        }
        if (this.y + this.size_y > height || this.y < 0) {
            this.dy = -this.dy;
            this.y += this.dy;
        }
        // console.log(this.noise, this.x, this.y);
        let noise = BitMap.white_noise(this.size_x, this.size_y);
        screen.paint(noise, this.x, this.y);
    }
}

console.clear();
let screen = Screen.with_bg(width, height, 189);
process.stdout.write(screen.next());
let w = Array.from(Array(2), _ => new white_noise());

let lastTime = performance.now();
let lastId = setInterval(next_frame, 0);
let time_spent = [2, 2, 2, 2, 2, 2, 2, 2, 2, 2];
async function next_frame() {
    let now = performance.now();
    let diff = now - lastTime;
    if (diff > 16) {
        clearInterval(lastId);
        time_spent.shift();
        time_spent.push(diff);
        draw();
        lastTime = now;
        setInterval(next_frame, 0);
    }
}



function draw() {
    w.forEach(x => x.move());
    let ptr = screen.diff();
    let num = screen.diff_count();
    let arr = new Uint32Array(wasm.__wasm.memory.buffer, ptr, num * 2);
    for (let i = 0; i < num; i ++) {
        let index = arr[2 * i];
        let color = arr[2 * i + 1];
        process.stdout.cursorTo(index % width, ~~(index / width));
        process.stdout.write(`\u001b[38;5;${color}m█\u001b[0m`);
    }
    let sum = 0;
    for (let i = 0; i < time_spent.length; i++) {
        sum += time_spent[i];
    }
    sum = sum / 10;
    process.stdout.cursorTo(0, height + 1);
    process.stdout.write(`fps: ${ 1000 / sum }`);

}
