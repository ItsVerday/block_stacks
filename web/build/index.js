import * as wasm from "./block_stacks.js";

async function run() {
	await wasm.default();
	const requestedSize = wasm.requested_size();
	const requestedTickrate = wasm.requested_tickrate();

	const canvas = document.createElement("canvas");
	const context = canvas.getContext("2d");
	context.imageSmoothingEnabled = false;
	document.body.appendChild(canvas);
	fitCanvas(canvas, requestedSize);
	addEventListener("resize", () => fitCanvas(canvas, requestedSize));
	addEventListener("mousedown", event => {
		handleMouseInput(event, true);
		handleMouseMove(event, requestedSize);
	});
	addEventListener("mouseup", event => {
		handleMouseInput(event, false);
		handleMouseMove(event, requestedSize);
	});
	addEventListener("mousemove", event => handleMouseMove(event, requestedSize));
	addEventListener("keydown", event => handleKeyInput(event, true));
	addEventListener("keyup", event => handleKeyInput(event, false));

	wasm.init_game(requestedSize.width, requestedSize.height);
	
	let accumTime = 0;
	let ticksExecuted = 0;

	let time = Date.now() / 1000;
	function loop() {
		let now = Date.now() / 1000;
		let delta = now - time;
		time = now;

		accumTime += delta * requestedTickrate;
		while (accumTime > ticksExecuted) {
			ticksExecuted++;
			wasm.tick_game(1 / requestedTickrate);
		}

		const buffer = wasm.draw_game(ticksExecuted / requestedTickrate);
		const imageData = new ImageData(new Uint8ClampedArray(buffer), requestedSize.width, requestedSize.height);

		context.putImageData(imageData, 0, 0);
		requestAnimationFrame(loop);
	}

	loop();
}

function fitCanvas(canvas, canvasSize) {
	const windowWidth = window.innerWidth;
	const windowHeight = window.innerHeight;
	const canvasWidth = canvasSize.width;
	const canvasHeight = canvasSize.height;

	const widthScale = windowWidth / canvasWidth;
	const heightScale = windowHeight / canvasHeight;

	let offsetX = 0;
	let offsetY = 0;

	canvas.width = canvasSize.width;
	canvas.height = canvasSize.height;

	const scale = Math.min(widthScale, heightScale);
	if (widthScale < heightScale) {
		offsetY = (windowHeight - widthScale * canvasHeight) / 2;
	} else {
		offsetX = (windowWidth - heightScale * canvasWidth) / 2;
	}

	canvas.style.transform = `scale(${scale})`;
	canvas.style.left = `${offsetX}px`;
	canvas.style.top = `${offsetY}px`;
}

function handleMouseInput(event, pressed) {
	wasm.handle_mouse_input(event.button, pressed);
}

function handleMouseMove(event, canvasSize) {
	const windowWidth = window.innerWidth;
	const windowHeight = window.innerHeight;
	const canvasWidth = canvasSize.width;
	const canvasHeight = canvasSize.height;

	const widthScale = windowWidth / canvasWidth;
	const heightScale = windowHeight / canvasHeight;

	let mouseX = event.pageX;
	let mouseY = event.pageY;

	const scale = Math.min(widthScale, heightScale);
	if (widthScale < heightScale) {
		mouseY -= (windowHeight - widthScale * canvasHeight) / 2;
	} else {
		mouseX -= (windowWidth - heightScale * canvasWidth) / 2;
	}

	mouseX /= scale;
	mouseY /= scale;
	wasm.handle_mouse_move(mouseX, mouseY);
}

function handleKeyInput(event, pressed) {
	if (!event.repeat) {
		wasm.handle_key_input(event.key.toLowerCase(), pressed);
	}
}

run();