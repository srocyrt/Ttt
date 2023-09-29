let wasm = require("ascii_displayer");
let { GifDispalyer } = wasm;
let fs = require("node:fs");
const { GifDisplayer } = require("../pkg/ascii_displayer");


// async function main() {
//         let blob = await fs.openAsBlob("./node/A.gif");
//         let data = new Uint8Array(await blob.arrayBuffer());
//         let gif = GifDisplayer.new(data);
//         gif.next();
//         console.log(gif.to_string());
// }


process.stdin.setRawMode(true);
process.stdin.resume();
process.stdin.setEncoding('utf8');
process.stdin.on("data", (k) => {
    if (k === "q") {
        process.exit();
    }
    console.log(k);
})

async function main() {
    let blob = await fs.openAsBlob("./node/A.gif");
    let data = new Uint8Array(await blob.arrayBuffer());
    let gif = GifDisplayer.new(data);

    console.clear();
    process.stdout.write('\u001B[?25l');
    process.stdout.write("\u{1B}[?2026$h")
    process.stdout.write(gif.to_string());
    process.stdout.write("\u{1B}[?2026$l")

    let lastTime = performance.now();
    let lastId = setInterval(next_frame, 0);
    let time_spent = [2, 2, 2, 2, 2, 2, 2, 2, 2, 2];
    let diff_spent = [2, 2, 2, 2, 2, 2, 2, 2, 2, 2];
    async function next_frame() {
        let now = performance.now();
        let diff = now - lastTime;
        if (diff > 100) {
            clearInterval(lastId);
            time_spent.shift();
            time_spent.push(diff);
            let n = draw();
            diff_spent.shift();
            diff_spent.push(n);
            lastTime = now;
            setInterval(next_frame, 0);
        }
    }
    function draw() {
        // console.time()
        process.stdout.write("\u{1B}[?2026$h")
        process.stdout.cursorTo(0, 0);
        gif.next();
        // process.stdout.write(gif.to_string());
        // process.stdout.write("\u{1B}[?2026$l")

        let ptr = gif.diff();
        let num = gif.diff_count();
        let arr = new Uint8Array(wasm.__wasm.memory.buffer, ptr, num * 8);
        // console.timeEnd()
        for (let i = 0; i < num; i ++) {
            let row = arr[8 * i + 0];
            let col = arr[8 * i + 1];
            let foreR = arr[8 * i + 2];
            let foreG = arr[8 * i + 3];
            let foreB = arr[8 * i + 4];
            let backR = arr[8 * i + 5];
            let backG = arr[8 * i + 6];
            let backB = arr[8 * i + 7];

            process.stdout.cursorTo(col, row / 2);
            process.stdout.write(`\x1b[38;2;${foreR};${foreG};${foreB};48;2;${backR};${backG};${backB}m▀\x1b[0m`);
        }
        process.stdout.write("\u{1B}[?2026$l")
        // let sum = 0;
        // for (let i = 0; i < time_spent.length; i++) {
        //     sum += time_spent[i];
        // }
        // sum = sum / 10;
        // process.stdout.cursorTo(0, 64 + 1);
        // process.stdout.write(`fps: ${ 1000 / sum }\n`);

        // sum = 0;
        // for (let i = 0; i < diff_spent.length; i++) {
        //     sum += diff_spent[i];
        // }
        // sum = sum / 10;
        // process.stdout.cursorTo(0, 64 + 2);
        // process.stdout.write(`diff: ${ sum / 10 }\n`);
        // return num;
        // process.stdout.write("\u{1B}[?2026$l")
    }
}

main()
