import { Shape } from "rust-demo-squares";

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
