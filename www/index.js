import { Shape, validate } from "rust-demo-test";

// Shapes
const content = new class {
    constructor() {
        this.elements = ['canvas1', 'canvas2', 'canvas3', 'canvas4', 'canvas5', 'canvas6'];
    }

    render() {
        this.elements.forEach(element => {
            let shape = Shape.new();
            shape.draw(element);
        });
    }
};

setInterval(() => content.render(), 2000);

// Validation
const input = document.getElementById('input');
const message = document.getElementById('message');

input.addEventListener('input', (event) => {
  let result = validate(event.target.value);
  message.textContent = result ? 'Valid' : 'Invalid';
}, true);
