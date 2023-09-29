// console.log("\u{1B}[?2026$p")

let width = process.stdout.columns;
let height = 32;

function print_black() {
    // █
    process.stdout.write("\u{1B}[?2026$h")
// console.log("\u{1B}[?2026$h")
    for (let i = 0; i < height; i++) {
        for (let j = 0; j < width; j++) {
            process.stdout.write("█");
        }
        process.stdout.write("\n");
    }
    process.stdout.write("\u{1B}[?2026$l")
}

function print_white() {
    // █
    process.stdout.write("\u{1B}[?2026$h")
    for (let i = 0; i < height; i++) {
        for (let j = 0; j < width; j++) {
            process.stdout.write("▅");
        }
        process.stdout.write("\n");
    }
    process.stdout.write("\u{1B}[?2026$l")
}

let toggle = true;
let lastTime = performance.now();
let lastId = setInterval(next_frame, 0);
let time_spent = [2, 2, 2, 2, 2, 2, 2, 2, 2, 2];
async function next_frame() {
    let now = performance.now();
    let diff = now - lastTime;
    if (diff > 100) {
        clearInterval(lastId);
        time_spent.shift();
        time_spent.push(diff);
        draw();
        lastTime = now;
        setInterval(next_frame, 0);
    }
}
function draw() {
    process.stdout.cursorTo(0, 0);
    toggle = !toggle;
    if (toggle) print_black();
    else print_white();
    let sum = 0;
    for (let i = 0; i < time_spent.length; i++) {
        sum += time_spent[i];
    }
    sum = sum / 10;
    process.stdout.cursorTo(0, 32 + 1);
    process.stdout.write(`fps: ${ 1000 / sum }\n`);
}
