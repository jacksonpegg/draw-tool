import { Canvas, Color } from "draw-tool";
import { memory } from "draw-tool/draw_tool_bg";

const WIDTH = 800;
const HEIGHT = 600;

const red = Color.new(255, 0, 0);
const blue = Color.new(0, 0, 255);
const green = Color.new(0, 255, 0);

const js_canvas = document.getElementById("draw-canvas");
js_canvas.width = WIDTH;
js_canvas.height = HEIGHT;

const ctx = js_canvas.getContext("2d");
const imageData = ctx.getImageData(0, 0, WIDTH, HEIGHT);

const rust_canvas = Canvas.new(WIDTH, HEIGHT);
{
    rust_canvas.fill(red);
    rust_canvas.draw_circle(WIDTH / 2, HEIGHT / 2, 50, blue);
    rust_canvas.draw_circle(WIDTH / 2, HEIGHT / 2, 25, green);

    render(rust_canvas);
}

function render(canvas) {
    imageData.data.set(
        new Uint8ClampedArray(
            memory.buffer,
            canvas.get_pixels_ptr(),
            canvas.height * canvas.width * 4
        )
    );
    ctx.putImageData(imageData, 0, 0);
}
